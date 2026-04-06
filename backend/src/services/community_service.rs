// ============================================================
// services/community_service.rs — コミュニティビジネスロジック
// ============================================================
// このファイルは「コミュニティ（他のユーザーの公開情報）」の取得を担うサービス層。
//
// 【コミュニティ機能の概要】
// 自分以外の全ユーザーの資格・目標・勉強時間を一覧表示する。
// お気に入りフラグ・グッドマーク（1つ以上合格）も計算して返す。
//
// 【集計方法について】
// N+1 クエリを避けるため、ユーザー一覧・資格一覧・目標一覧・
// お気に入り一覧をそれぞれ 1 回ずつ全件取得し、
// Rust 側で HashMap を使ってユーザーID をキーに集計する。
//
// 【N+1 問題とは】
// ユーザー N 人 × 「1人分のデータを取得するクエリ」= N+1 回のクエリが発生し、
// 非常に遅くなる問題。全件一括取得 + プログラム側集計で解消する。
//
// 【ページネーション】
// DB 側ではなく Rust 側で skip/take を使って行う。
// 全件取得してから絞り込むため、データが増えると要最適化。

// HashMap: ユーザーID をキーに各種データを集計するために使用
use std::collections::HashMap;
use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;

// ============================================================
// 補助構造体の定義
// ============================================================

/// コミュニティ画面で表示する「所持資格」の簡易情報
///
/// `serde::Serialize` で JSON に変換できる（ハンドラのレスポンスに使用）
#[derive(Debug, serde::Serialize)]
pub struct CommunityCert {
    pub certification_name: String,
    /// 取得日（未設定の場合は None → JSON では null）
    pub acquired_date: Option<String>,
}

/// コミュニティ画面で表示する「目標」の簡易情報
#[derive(Debug, serde::Serialize)]
pub struct CommunityGoal {
    pub certification_name: String,
    /// ステータス（"exam_date" / "passed" / "failed" / "abandoned"）
    pub status: String,
    pub study_hours: f64,
    pub target_date: String,
}

/// コミュニティ一覧画面の 1 ユーザー分のデータ
#[derive(Debug, serde::Serialize)]
pub struct CommunityUser {
    pub id: Uuid,
    pub username: String,
    /// 所持資格の件数
    pub certification_count: i64,
    /// 目標の件数
    pub goal_count: i64,
    /// 合格済み目標の件数（status == "passed"）
    pub achieved_count: i64,
    /// 総勉強時間（目標の goash を合計）
    pub total_study_hours: f64,
    /// グッドマーク: 1 つ以上合格していれば true
    pub has_good_mark: bool,
    /// 現在ログイン中のユーザーがお気に入りにしているか
    pub is_favorite: bool,
    pub certifications: Vec<CommunityCert>,
    pub goals: Vec<CommunityGoal>,
}

/// ユーザー詳細画面用のデータ（資格・目標を詳細に含む）
#[derive(Debug, serde::Serialize)]
pub struct CommunityUserDetail {
    pub id: Uuid,
    pub username: String,
    pub has_good_mark: bool,
    /// 資格一覧（JSON Value のまま: TBL_MASTER JOIN 済み）
    pub certifications: Vec<serde_json::Value>,
    /// 目標一覧（JSON Value のまま: TBL_MASTER JOIN 済み）
    pub goals: Vec<serde_json::Value>,
}

// ============================================================
// list_users — コミュニティユーザー一覧取得
// ============================================================
/// 自分以外のユーザー一覧をページネーション付きで取得する
///
/// 処理の流れ:
/// 1. 自分以外の全ユーザーを取得
/// 2. TBL_HOLDING（所持資格）を全件取得し、HashMap で集計
/// 3. TBL_GOAL（目標）を全件取得し、HashMap で集計
/// 4. TBL_FAVORITE（お気に入り）を取得してフラグを確認
/// 5. 集計結果を組み合わせて CommunityUser のリストを構築
/// 6. ページネーション（skip/take）を適用
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: ログイン中のユーザーID（自分自身を除外するため）
/// - `page`: ページ番号（1 始まり）
/// - `per_page`: 1 ページあたりの件数
///
/// # 戻り値
/// `(ユーザーリスト, 総件数)` のタプル
pub async fn list_users(
    db: &SupabaseClient,
    user_id: Uuid,
    page: i64,
    per_page: i64,
) -> Result<(Vec<CommunityUser>, i64), AppError> {
    // ---- ステップ 1: 自分以外のユーザーを全件取得 ----
    // `useid=neq.{user_id}` は「useid が自分のIDと等しくない」という条件
    // `neq` は PostgREST の「not equal」演算子
    let users_result = db
        .select(
            "TBL_USER",
            &format!("select=useid,usenm&useid=neq.{}&order=useca.desc", user_id),
        )
        .await?;
    let all_users: Vec<serde_json::Value> = serde_json::from_value(users_result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // ページネーション用の総件数（Rust 側で計算）
    let total = all_users.len() as i64;

    // ---- ステップ 2: 所持資格を全件取得して HashMap に集計 ----
    // 全ユーザーの資格を 1 回のクエリで取得（N+1 回避）
    let holdings_result = db
        .select("TBL_HOLDING", "select=holui,holdt,TBL_MASTER(masnm)")
        .await?;
    let holdings: Vec<serde_json::Value> = serde_json::from_value(holdings_result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // cert_counts: ユーザーID → 資格件数
    // certs_map: ユーザーID → 資格リスト
    let mut cert_counts: HashMap<String, i64> = HashMap::new();
    let mut certs_map: HashMap<String, Vec<CommunityCert>> = HashMap::new();

    for h in &holdings {
        if let Some(uid) = h["holui"].as_str() {
            // `entry(...).or_insert(0)` は「キーがなければ 0 を挿入」
            // `+= 1` でカウントアップ
            *cert_counts.entry(uid.to_string()).or_insert(0) += 1;
            let cert_name = h["TBL_MASTER"]["masnm"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let acquired_date = h["holdt"].as_str().map(|s| s.to_string());
            // `or_insert_with(Vec::new)` は「キーがなければ空の Vec を挿入」
            // その後 `.push(...)` で資格を追加
            certs_map
                .entry(uid.to_string())
                .or_insert_with(Vec::new)
                .push(CommunityCert {
                    certification_name: cert_name,
                    acquired_date,
                });
        }
    }

    // ---- ステップ 3: 目標を全件取得して HashMap に集計 ----
    let goals_result = db
        .select("TBL_GOAL", "select=goaui,goast,goash,goatd,TBL_MASTER(masnm)")
        .await?;
    let goals: Vec<serde_json::Value> = serde_json::from_value(goals_result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // goal_counts: ユーザーID → 目標件数
    // passed_counts: ユーザーID → 合格件数
    // study_hours_map: ユーザーID → 総勉強時間
    // goals_map: ユーザーID → 目標リスト
    let mut goal_counts: HashMap<String, i64> = HashMap::new();
    let mut passed_counts: HashMap<String, i64> = HashMap::new();
    let mut study_hours_map: HashMap<String, f64> = HashMap::new();
    let mut goals_map: HashMap<String, Vec<CommunityGoal>> = HashMap::new();

    for g in &goals {
        if let Some(uid) = g["goaui"].as_str() {
            *goal_counts.entry(uid.to_string()).or_insert(0) += 1;

            // 勉強時間を累積加算
            let hours = g["goash"].as_f64().unwrap_or(0.0);
            *study_hours_map.entry(uid.to_string()).or_insert(0.0) += hours;

            // ステータスが "passed" の場合のみ合格カウントを増やす
            if g["goast"].as_str() == Some("passed") {
                *passed_counts.entry(uid.to_string()).or_insert(0) += 1;
            }

            let cert_name = g["TBL_MASTER"]["masnm"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let status = g["goast"].as_str().unwrap_or("").to_string();
            let target_date = g["goatd"].as_str().unwrap_or("").to_string();

            goals_map
                .entry(uid.to_string())
                .or_insert_with(Vec::new)
                .push(CommunityGoal {
                    certification_name: cert_name,
                    status,
                    study_hours: hours,
                    target_date,
                });
        }
    }

    // ---- ステップ 4: ログイン中ユーザーのお気に入りを取得 ----
    // `favti` = お気に入りされているユーザーID のみ取得
    let favs_result = db
        .select(
            "TBL_FAVORITE",
            &format!("select=favti&favui=eq.{}", user_id),
        )
        .await?;
    let favs: Vec<serde_json::Value> = serde_json::from_value(favs_result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // お気に入りユーザーIDのリスト（contains で O(n) 検索）
    let fav_set: Vec<String> = favs
        .iter()
        .filter_map(|f| f["favti"].as_str().map(|s| s.to_string()))
        .collect();

    // ---- ステップ 5 & 6: 集計結果を組み合わせてリストを構築し、ページネーションを適用 ----
    // `offset` はスキップする件数（例: page=2, per_page=10 なら 10 件スキップ）
    let offset = ((page - 1) * per_page) as usize;
    let users: Vec<CommunityUser> = all_users
        .iter()
        .filter_map(|u| {
            let uid_str = u["useid"].as_str()?;
            let uid = Uuid::parse_str(uid_str).ok()?;

            // HashMap から各集計値を取得（存在しない場合はデフォルト値）
            // `*cert_counts.get(uid_str).unwrap_or(&0)` の `*` は参照外し（&i64 → i64）
            let cert_count = *cert_counts.get(uid_str).unwrap_or(&0);
            let goal_count = *goal_counts.get(uid_str).unwrap_or(&0);
            let achieved_count = *passed_counts.get(uid_str).unwrap_or(&0);
            let total_study_hours = *study_hours_map.get(uid_str).unwrap_or(&0.0);
            let is_favorite = fav_set.contains(&uid_str.to_string());

            // `.remove(uid_str)` で HashMap から取り出す（`get` と違い所有権を奪う）
            // `unwrap_or_default()` で存在しない場合は空の Vec を使う
            let user_certs = certs_map.remove(uid_str).unwrap_or_default();
            let user_goals = goals_map.remove(uid_str).unwrap_or_default();

            Some(CommunityUser {
                id: uid,
                username: u["usenm"].as_str()?.to_string(),
                certification_count: cert_count,
                goal_count,
                achieved_count,
                total_study_hours,
                // グッドマーク: 1 つ以上合格していれば true
                has_good_mark: achieved_count > 0,
                is_favorite,
                certifications: user_certs,
                goals: user_goals,
            })
        })
        // `.skip(offset)` で先頭 offset 件を読み飛ばす
        // `.take(per_page)` で per_page 件だけ取得する
        .skip(offset)
        .take(per_page as usize)
        .collect();

    Ok((users, total))
}

// ============================================================
// get_user — ユーザー詳細取得
// ============================================================
/// 特定ユーザーの詳細情報を取得する（コミュニティ詳細画面用）
///
/// 資格・目標は TBL_MASTER と JOIN した詳細情報を返す。
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `target_user_id`: 取得対象のユーザーID
/// - `_viewer_user_id`: 閲覧者のユーザーID（将来の機能拡張用・現在は未使用）
///   プレフィックス `_` により「未使用引数」であることをコンパイラに伝える
///
/// # 戻り値
/// ユーザー詳細情報（`CommunityUserDetail`）
///
/// # エラー
/// - `AppError::NotFound`: 指定 ID のユーザーが存在しない場合
pub async fn get_user(
    db: &SupabaseClient,
    target_user_id: Uuid,
    _viewer_user_id: Uuid,
) -> Result<CommunityUserDetail, AppError> {
    // ---- ユーザー基本情報を取得 ----
    let user_result = db
        .select(
            "TBL_USER",
            &format!("select=useid,usenm&useid=eq.{}", target_user_id),
        )
        .await?;
    let users: Vec<serde_json::Value> = serde_json::from_value(user_result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    let user = users
        .first()
        .ok_or_else(|| AppError::NotFound("ユーザーが見つかりません".to_string()))?;

    // ---- 所持資格を TBL_MASTER と JOIN して取得 ----
    let certs = db
        .select(
            "TBL_HOLDING",
            &format!(
                "select=holid,holdt,holca,TBL_MASTER(masid,masnm)&holui=eq.{}&order=holca.desc",
                target_user_id
            ),
        )
        .await?;
    let certs: Vec<serde_json::Value> = serde_json::from_value(certs)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // ---- 目標を TBL_MASTER と JOIN して取得 ----
    let goals = db
        .select(
            "TBL_GOAL",
            &format!(
                "select=goaid,goatd,goast,goamm,goash,goaca,TBL_MASTER(masid,masnm)&goaui=eq.{}&order=goaca.desc",
                target_user_id
            ),
        )
        .await?;
    let goals: Vec<serde_json::Value> = serde_json::from_value(goals)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // グッドマーク: 目標の中に 1 つでも "passed" があれば true
    // `iter().any(|g| ...)` は「少なくとも 1 つの要素が条件を満たすか」を確認する
    let has_good_mark = goals.iter().any(|g| g["goast"].as_str() == Some("passed"));

    Ok(CommunityUserDetail {
        id: target_user_id,
        username: user["usenm"].as_str().unwrap_or_default().to_string(),
        has_good_mark,
        // JSON Value のままハンドラに渡す（詳細画面では生のデータをフロントが整形）
        certifications: certs,
        goals,
    })
}
