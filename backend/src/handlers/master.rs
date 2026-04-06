// ============================================================
// handlers/master.rs — 資格マスタハンドラ
// ============================================================
// このファイルは「資格マスタ（TBL_MASTER）」の検索エンドポイントを処理する。
//
// 【担当エンドポイント】
// GET /api/master/certifications?q={keyword} — 資格名インクリメンタル検索
//
// フロントエンドの資格名入力フォームで、入力中にリアルタイムで
// 候補を表示する「オートコンプリート」機能のためのエンドポイント。
//
// 【クエリパラメータ】
// `q` パラメータが 2 文字以上の場合に検索を実行する。
// 2 文字未満では空配列を返す（サービス層の制限）。

use actix_web::{web, HttpResponse};
// クエリパラメータのデシリアライズに必要
use serde::Deserialize;

use crate::db::SupabaseClient;
use crate::errors::AppError;
// 認証が必要なエンドポイント（未認証なら 401）
use crate::middleware::auth::AuthenticatedUser;
use crate::services::master_service;

// ============================================================
// SearchQuery — クエリパラメータの構造体
// ============================================================
/// GET /api/master/certifications?q={keyword} のクエリパラメータ
///
/// `Deserialize` を derive することで、
/// `web::Query<SearchQuery>` により URL クエリから自動的に解析される。
/// `q` が指定されない場合は `None` になる（Option のため）。
#[derive(Deserialize)]
pub struct SearchQuery {
    /// 検索キーワード（任意。未指定なら検索しない）
    q: Option<String>,
}

// ============================================================
// search — GET /api/master/certifications
// ============================================================
/// 資格名でインクリメンタル検索する（オートコンプリート用）
///
/// # 引数
/// - `db`: DB クライアント
/// - `_user`: 認証チェック専用（値は使わない。`_` プレフィックスで警告を抑制）
/// - `query`: `?q=keyword` のクエリパラメータ
///
/// # 戻り値
/// 200 OK + `{ "certifications": [{ "id": "...", "name": "...", "category": "..." }] }` JSON
pub async fn search(
    db: web::Data<SupabaseClient>,
    _user: AuthenticatedUser,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, AppError> {
    // `as_deref()` で Option<String> → Option<&str> に変換し、
    // `unwrap_or("")` で None の場合は空文字列を使う
    let q = query.q.as_deref().unwrap_or("");

    // 2 文字未満の場合はサービス層で空配列が返る
    let results = master_service::search(&db, q).await?;

    // フロントエンドとの約束どおり `certifications` キーで包んで返す
    Ok(HttpResponse::Ok().json(serde_json::json!({ "certifications": results })))
}
