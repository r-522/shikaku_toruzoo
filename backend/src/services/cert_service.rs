// ============================================================
// services/cert_service.rs — 所持資格ビジネスロジック
// ============================================================
// このファイルは「所持資格（TBL_HOLDING）」の CRUD 処理を担うサービス層。
//
// 【TBL_HOLDING とは】
// ユーザーが「既に取得済みの資格」を記録するテーブル。
// 取得日（holdt）と資格マスタへの外部キー（holmi）を持つ。
//
// 【所有権チェック（Ownership check）の重要性】
// update/delete では必ず `holui=eq.{user_id}` 条件を含める。
// これにより「自分のレコードしか変更できない」ことを保証する。
// セッション検証だけでは「他人のIDを URL に含めた攻撃」を防げない。
//
// 【master_service との連携】
// 資格名は TBL_MASTER で管理される。
// ユーザーが補完リストから選択した場合は master_id が既知、
// 手入力の場合は find_or_create で既存レコードを探すか新規作成する。

use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::models::certification::{CertificationRequest, CertificationResponse};
// マスタの検索・作成（資格名から master_id を解決するため）
use crate::services::master_service;

// ============================================================
// list — 所持資格一覧取得
// ============================================================
/// ログイン中ユーザーの所持資格一覧を取得する
///
/// TBL_HOLDING と TBL_MASTER を JOIN して資格名も返す。
/// PostgREST の埋め込み（`TBL_MASTER(masid,masnm)`）を使って
/// 1 回のリクエストで JOIN 結果を取得している。
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: 取得対象のユーザーID
///
/// # 戻り値
/// 所持資格のリスト（登録日時の降順）
pub async fn list(
    db: &SupabaseClient,
    user_id: Uuid,
) -> Result<Vec<CertificationResponse>, AppError> {
    // `TBL_MASTER(masid,masnm)` は PostgREST の埋め込み結合（Embedded Joins）
    // 外部キー holmi → TBL_MASTER の masid を通じて JOIN し、
    // 結果の JSON に `"TBL_MASTER": {"masid": "...", "masnm": "..."}` が含まれる
    let result = db
        .select(
            "TBL_HOLDING",
            &format!(
                "select=holid,holmi,holdt,holca,TBL_MASTER(masid,masnm)&holui=eq.{}&order=holca.desc",
                user_id
            ),
        )
        .await?;

    let holdings: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // `iter()` で各レコードを順に処理し、`filter_map` で変換成功したものだけ収集する
    // `filter_map` は `None` を自動的にスキップするため、
    // フィールドが欠けているレコードを安全に無視できる
    Ok(holdings
        .iter()
        .filter_map(|h| {
            // 埋め込みの TBL_MASTER オブジェクトを参照（`&` で借用）
            let master = &h["TBL_MASTER"];
            // `Some(...)` を返すと filter_map がこのレコードを結果に含める
            Some(CertificationResponse {
                // `Uuid::parse_str(...).ok()?` は UUID パース失敗時に None を返して
                // このレコードをスキップする（`?` は Option のための早期リターン）
                id: Uuid::parse_str(h["holid"].as_str()?).ok()?,
                certification_name: master["masnm"].as_str()?.to_string(),
                master_id: Uuid::parse_str(master["masid"].as_str()?).ok()?,
                // `as_str().map(|s| s.to_string())` は Some(&str) → Some(String) に変換
                acquired_date: h["holdt"].as_str().map(|s| s.to_string()),
                created_at: h["holca"].as_str()?.to_string(),
            })
        })
        .collect()) // Vec<CertificationResponse> に収集
}

// ============================================================
// create — 所持資格登録
// ============================================================
/// 新しい所持資格を登録する
///
/// 処理の流れ:
/// 1. master_id が指定されていれば使用、なければ find_or_create で解決
/// 2. TBL_HOLDING に INSERT
/// 3. 挿入されたレコードを返す（Supabase の `return=representation` を使用）
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: 登録するユーザーID
/// - `req`: 登録リクエスト（certification_name, master_id, acquired_date）
///
/// # 戻り値
/// 登録された資格情報（自動生成された holid, holca を含む）
pub async fn create(
    db: &SupabaseClient,
    user_id: Uuid,
    req: &CertificationRequest,
) -> Result<CertificationResponse, AppError> {
    // master_id の解決:
    // - Some(id): ユーザーが補完候補から選択した場合はそのIDをそのまま使う
    // - None: 手入力の場合は normalize_name した名前で検索し、
    //         なければカテゴリ「その他」で新規作成する
    let master_id = match req.master_id {
        Some(id) => id,
        None => {
            master_service::find_or_create(db, &req.certification_name, "その他").await?
        }
    };

    // TBL_HOLDING にレコードを挿入
    // `return=representation` ヘッダー（db.insert 内部で設定）により
    // 挿入後のレコード（holid, holca 等の自動生成フィールドを含む）が返ってくる
    let result = db
        .insert(
            "TBL_HOLDING",
            &serde_json::json!({
                "holui": user_id.to_string(),
                "holmi": master_id.to_string(),
                "holdt": req.acquired_date,
            }),
        )
        .await?;

    let created: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    // INSERT は 1 件のはずなので first() で取得
    let holding = created
        .first()
        .ok_or_else(|| AppError::Internal("Insert returned no data".to_string()))?;

    Ok(CertificationResponse {
        id: Uuid::parse_str(holding["holid"].as_str().unwrap_or_default())
            .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)))?,
        // 資格名は DB から再取得せず、リクエストの値をそのまま使う（パフォーマンス最適化）
        certification_name: req.certification_name.clone(),
        master_id,
        acquired_date: req.acquired_date.clone(),
        created_at: holding["holca"].as_str().unwrap_or_default().to_string(),
    })
}

// ============================================================
// update — 所持資格更新
// ============================================================
/// 既存の所持資格を更新する
///
/// 処理の流れ:
/// 1. 所有権チェック（対象レコードが自分のものか確認）
/// 2. master_id の解決（create と同様）
/// 3. TBL_HOLDING を PATCH で更新
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: 更新を行うユーザーID
/// - `holding_id`: 更新対象の所持資格 ID
/// - `req`: 更新リクエスト
///
/// # エラー
/// - `AppError::NotFound`: 指定 ID が存在しない、または他ユーザーのレコード
pub async fn update(
    db: &SupabaseClient,
    user_id: Uuid,
    holding_id: Uuid,
    req: &CertificationRequest,
) -> Result<CertificationResponse, AppError> {
    // ---- 所有権チェック ----
    // `holid=eq.{id}&holui=eq.{user_id}` の両条件がそろって初めてヒットする
    // これにより「指定された ID が自分のものでなければ 0 件返る」仕組みになっている
    let existing = db
        .select(
            "TBL_HOLDING",
            &format!("select=holid&holid=eq.{}&holui=eq.{}", holding_id, user_id),
        )
        .await?;
    let existing: Vec<serde_json::Value> = serde_json::from_value(existing)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    // 0 件なら「見つからない（= 存在しないか他人のもの）」として NotFound を返す
    if existing.is_empty() {
        return Err(AppError::NotFound("資格が見つかりません".to_string()));
    }

    // master_id の解決（create と同じ処理）
    let master_id = match req.master_id {
        Some(id) => id,
        None => {
            master_service::find_or_create(db, &req.certification_name, "その他").await?
        }
    };

    // PATCH リクエストで更新（更新するフィールドだけ送る）
    let result = db
        .update(
            "TBL_HOLDING",
            // 条件に user_id を含めることで排他的所有権を保証
            &format!("holid=eq.{}&holui=eq.{}", holding_id, user_id),
            &serde_json::json!({
                "holmi": master_id.to_string(), // 資格マスタ ID
                "holdt": req.acquired_date,      // 取得日
            }),
        )
        .await?;

    let updated: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    let holding = updated
        .first()
        .ok_or_else(|| AppError::Internal("Update returned no data".to_string()))?;

    Ok(CertificationResponse {
        id: holding_id, // URL パスパラメータから受け取った値を使いまわす
        certification_name: req.certification_name.clone(),
        master_id,
        acquired_date: req.acquired_date.clone(),
        created_at: holding["holca"].as_str().unwrap_or_default().to_string(),
    })
}

// ============================================================
// delete — 所持資格削除
// ============================================================
/// 所持資格を削除する
///
/// 処理の流れ:
/// 1. 所有権チェック
/// 2. TBL_HOLDING から DELETE
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: 削除を行うユーザーID
/// - `holding_id`: 削除対象の所持資格 ID
///
/// # 戻り値
/// 成功時は `Ok(())`
pub async fn delete(
    db: &SupabaseClient,
    user_id: Uuid,
    holding_id: Uuid,
) -> Result<(), AppError> {
    // ---- 所有権チェック ----
    // 削除前に「このレコードが自分のものか」を必ず確認する
    let existing = db
        .select(
            "TBL_HOLDING",
            &format!("select=holid&holid=eq.{}&holui=eq.{}", holding_id, user_id),
        )
        .await?;
    let existing: Vec<serde_json::Value> = serde_json::from_value(existing)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    if existing.is_empty() {
        return Err(AppError::NotFound("資格が見つかりません".to_string()));
    }

    // DELETE リクエスト（条件に user_id を含めて自分のレコードのみ削除）
    db.delete(
        "TBL_HOLDING",
        &format!("holid=eq.{}&holui=eq.{}", holding_id, user_id),
    )
    .await?;

    Ok(())
}
