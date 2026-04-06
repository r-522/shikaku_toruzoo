// ============================================================
// services/goal_service.rs — 目標ビジネスロジック
// ============================================================
// このファイルは「学習目標（TBL_GOAL）」の CRUD 処理を担うサービス層。
//
// 【TBL_GOAL とは】
// ユーザーが「これから取得を目指す資格」を記録するテーブル。
// ステータス（受験日設定・合格・不合格・断念）、勉強時間、メモ、目標日を持つ。
//
// 【ステータスの有効値】
// - "exam_date": 受験日が設定された状態（目標中）
// - "passed": 合格
// - "failed": 不合格（再挑戦予定等）
// - "abandoned": 断念（撤退）
//
// 【部分更新（Partial Update）について】
// GoalUpdateRequest の全フィールドが Option のため、
// 送られてきたフィールドだけを更新するロジックを持つ。
// serde_json::Map を動的に組み立てることで実現している。

use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::models::goal::{GoalRequest, GoalResponse, GoalUpdateRequest};
// マスタの検索・作成（資格名から master_id を解決するため）
use crate::services::master_service;

// ============================================================
// 定数: 有効なステータス値
// ============================================================
// `&[&str]` は文字列スライスの配列への参照
// `const` はコンパイル時に確定する定数（グローバルに使える）
const VALID_STATUSES: &[&str] = &["exam_date", "passed", "failed", "abandoned"];

// ============================================================
// list — 目標一覧取得
// ============================================================
/// ログイン中ユーザーの目標一覧を取得する
///
/// TBL_GOAL と TBL_MASTER を JOIN して資格名も返す。
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: 取得対象のユーザーID
///
/// # 戻り値
/// 目標のリスト（登録日時の降順）
pub async fn list(
    db: &SupabaseClient,
    user_id: Uuid,
) -> Result<Vec<GoalResponse>, AppError> {
    // `TBL_MASTER(masid,masnm)` は PostgREST の埋め込み結合
    // goami → TBL_MASTER の masid を通じて JOIN する
    let result = db
        .select(
            "TBL_GOAL",
            &format!(
                "select=goaid,goami,goatd,goast,goamm,goash,goaca,TBL_MASTER(masid,masnm)&goaui=eq.{}&order=goaca.desc",
                user_id
            ),
        )
        .await?;

    let goals: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // `filter_map` でフィールドが欠けているレコードは安全にスキップ
    Ok(goals
        .iter()
        .filter_map(|g| {
            let master = &g["TBL_MASTER"];
            Some(GoalResponse {
                id: Uuid::parse_str(g["goaid"].as_str()?).ok()?,
                certification_name: master["masnm"].as_str()?.to_string(),
                master_id: Uuid::parse_str(master["masid"].as_str()?).ok()?,
                target_date: g["goatd"].as_str()?.to_string(),
                status: g["goast"].as_str()?.to_string(),
                // `as_str().map(|s| s.to_string())` で Option<&str> → Option<String> に変換
                memo: g["goamm"].as_str().map(|s| s.to_string()),
                // `unwrap_or(0.0)` で null（未設定）の場合は 0.0 時間とする
                study_hours: g["goash"].as_f64().unwrap_or(0.0),
                created_at: g["goaca"].as_str()?.to_string(),
            })
        })
        .collect())
}

// ============================================================
// create — 目標登録
// ============================================================
/// 新しい学習目標を登録する
///
/// 処理の流れ:
/// 1. master_id の解決
/// 2. ステータスのバリデーション（有効値チェック）
/// 3. TBL_GOAL に INSERT
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: 登録するユーザーID
/// - `req`: 登録リクエスト
///
/// # 戻り値
/// 登録された目標情報
///
/// # エラー
/// - `AppError::ValidationError`: 無効なステータス値が送られた場合
pub async fn create(
    db: &SupabaseClient,
    user_id: Uuid,
    req: &GoalRequest,
) -> Result<GoalResponse, AppError> {
    // master_id の解決（cert_service と同じパターン）
    let master_id = match req.master_id {
        Some(id) => id,
        None => master_service::find_or_create(db, &req.certification_name, "その他").await?,
    };

    // ステータスの取得（未指定なら "exam_date" をデフォルトとして使う）
    // `as_deref()` は Option<String> を Option<&str> に変換する
    // `unwrap_or("exam_date")` で None の場合のデフォルト値を設定
    let status = req.status.as_deref().unwrap_or("exam_date");

    // VALID_STATUSES に含まれない値はバリデーションエラーにする
    // `contains(&status)` で &[&str] にターゲット値が含まれるか確認
    if !VALID_STATUSES.contains(&status) {
        return Err(AppError::ValidationError(
            "無効なステータスです".to_string(),
        ));
    }

    // 勉強時間（未指定なら 0.0 時間）
    let study_hours = req.study_hours.unwrap_or(0.0);

    // TBL_GOAL にレコードを挿入
    let result = db
        .insert(
            "TBL_GOAL",
            &serde_json::json!({
                "goaui": user_id.to_string(), // ユーザーID
                "goami": master_id.to_string(), // 資格マスタID
                "goatd": req.target_date,       // 目標日
                "goast": status,                // ステータス
                "goamm": req.memo,              // メモ（null 許容）
                "goash": study_hours,           // 勉強時間
            }),
        )
        .await?;

    let created: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    let goal = created
        .first()
        .ok_or_else(|| AppError::Internal("Insert returned no data".to_string()))?;

    Ok(GoalResponse {
        id: Uuid::parse_str(goal["goaid"].as_str().unwrap_or_default())
            .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)))?,
        certification_name: req.certification_name.clone(),
        master_id,
        target_date: req.target_date.clone(),
        status: status.to_string(),
        memo: req.memo.clone(),
        study_hours,
        created_at: goal["goaca"].as_str().unwrap_or_default().to_string(),
    })
}

// ============================================================
// update — 目標更新（部分更新）
// ============================================================
/// 既存の学習目標を部分更新する
///
/// GoalUpdateRequest の各フィールドは Option であり、
/// Some(値) だけを更新対象とする。
///
/// 処理の流れ:
/// 1. 所有権チェック（かつ現在のレコードを取得）
/// 2. ステータスのバリデーション（指定があれば）
/// 3. 更新する JSON を動的に組み立て
/// 4. PATCH で更新
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: 更新を行うユーザーID
/// - `goal_id`: 更新対象の目標 ID
/// - `req`: 更新リクエスト（すべてのフィールドが Option）
///
/// # 戻り値
/// 更新後の目標情報（資格名はマスタから取得）
pub async fn update(
    db: &SupabaseClient,
    user_id: Uuid,
    goal_id: Uuid,
    req: &GoalUpdateRequest,
) -> Result<GoalResponse, AppError> {
    // ---- 所有権チェック（かつ現在のレコードを取得）----
    // TBL_MASTER も JOIN して現在の資格名を取得する（更新後の返却値に使う）
    let existing = db
        .select(
            "TBL_GOAL",
            &format!(
                "select=goaid,goami,goatd,goast,goamm,goash,goaca,TBL_MASTER(masid,masnm)&goaid=eq.{}&goaui=eq.{}",
                goal_id, user_id
            ),
        )
        .await?;
    let existing: Vec<serde_json::Value> = serde_json::from_value(existing)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    // 0 件 = 存在しないか他人のもの
    let current = existing
        .first()
        .ok_or_else(|| AppError::NotFound("目標が見つかりません".to_string()))?;

    // ステータスが指定されている場合のみバリデーション
    // `if let Some(ref status) = req.status` は
    // req.status が Some の場合に中身を status として取り出す
    if let Some(ref status) = req.status {
        if !VALID_STATUSES.contains(&status.as_str()) {
            return Err(AppError::ValidationError(
                "無効なステータスです".to_string(),
            ));
        }
    }

    // ---- 動的な更新 JSON の組み立て ----
    // serde_json::Map は JSON オブジェクトを動的に組み立てるためのコレクション
    // 送られてきたフィールドだけを更新対象に含める（未送信フィールドはスキップ）
    let mut update_body = serde_json::Map::new();
    if let Some(ref target_date) = req.target_date {
        update_body.insert("goatd".to_string(), serde_json::json!(target_date));
    }
    if let Some(ref status) = req.status {
        update_body.insert("goast".to_string(), serde_json::json!(status));
    }
    if let Some(ref memo) = req.memo {
        update_body.insert("goamm".to_string(), serde_json::json!(memo));
    }
    if let Some(study_hours) = req.study_hours {
        update_body.insert("goash".to_string(), serde_json::json!(study_hours));
    }

    // 動的に組み立てた JSON で PATCH リクエストを送信
    // `serde_json::Value::Object(update_body)` で Map を Value に変換
    let result = db
        .update(
            "TBL_GOAL",
            &format!("goaid=eq.{}&goaui=eq.{}", goal_id, user_id),
            &serde_json::Value::Object(update_body),
        )
        .await?;

    let updated: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    let goal = updated
        .first()
        .ok_or_else(|| AppError::Internal("Update returned no data".to_string()))?;

    // 資格名はマスタ JOIN から取得した元のデータを使う
    // （TBL_GOAL には資格名が入っていないため）
    let master = &current["TBL_MASTER"];

    Ok(GoalResponse {
        id: goal_id,
        certification_name: master["masnm"].as_str().unwrap_or_default().to_string(),
        master_id: Uuid::parse_str(master["masid"].as_str().unwrap_or_default())
            .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)))?,
        // 更新後のレコードから値を取得する（更新されなかったフィールドは DB の現在値）
        target_date: goal["goatd"].as_str().unwrap_or_default().to_string(),
        status: goal["goast"].as_str().unwrap_or("exam_date").to_string(),
        memo: goal["goamm"].as_str().map(|s| s.to_string()),
        study_hours: goal["goash"].as_f64().unwrap_or(0.0),
        created_at: goal["goaca"].as_str().unwrap_or_default().to_string(),
    })
}

// ============================================================
// delete — 目標削除
// ============================================================
/// 学習目標を削除する
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: 削除を行うユーザーID
/// - `goal_id`: 削除対象の目標 ID
///
/// # 戻り値
/// 成功時は `Ok(())`
pub async fn delete(
    db: &SupabaseClient,
    user_id: Uuid,
    goal_id: Uuid,
) -> Result<(), AppError> {
    // ---- 所有権チェック ----
    let existing = db
        .select(
            "TBL_GOAL",
            &format!("select=goaid&goaid=eq.{}&goaui=eq.{}", goal_id, user_id),
        )
        .await?;
    let existing: Vec<serde_json::Value> = serde_json::from_value(existing)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    if existing.is_empty() {
        return Err(AppError::NotFound("目標が見つかりません".to_string()));
    }

    // DELETE（user_id 条件も含めて二重に所有権を保証）
    db.delete(
        "TBL_GOAL",
        &format!("goaid=eq.{}&goaui=eq.{}", goal_id, user_id),
    )
    .await?;

    Ok(())
}
