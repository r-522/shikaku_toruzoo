// ============================================================
// handlers/goal.rs — 目標ハンドラ
// ============================================================
// このファイルは「学習目標（TBL_GOAL）」に関する HTTP リクエストを処理する。
//
// 【担当エンドポイント】
// GET    /api/goals        — 目標一覧取得
// POST   /api/goals        — 目標登録
// PUT    /api/goals/{id}   — 目標更新
// DELETE /api/goals/{id}   — 目標削除
//
// certification ハンドラと構造が同じ。
// 違いは GoalUpdateRequest が部分更新（すべてのフィールドが Option）である点。

use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
// GoalRequest: 登録用、GoalUpdateRequest: 更新用（部分更新）
use crate::models::goal::{GoalRequest, GoalUpdateRequest};
use crate::services::goal_service;

// ============================================================
// list — GET /api/goals
// ============================================================
/// ログイン中ユーザーの目標一覧を取得する
///
/// # 戻り値
/// 200 OK + `{ "goals": [...] }` JSON
pub async fn list(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let goals = goal_service::list(&db, user.0).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "goals": goals })))
}

// ============================================================
// create — POST /api/goals
// ============================================================
/// 新しい学習目標を登録する
///
/// # 引数
/// - `body`: GoalRequest（certification_name, master_id?, target_date, status?, memo?, study_hours?）
///
/// # 戻り値
/// - 成功: 201 Created + 登録された目標情報
/// - バリデーション失敗: 422
pub async fn create(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    body: web::Json<GoalRequest>,
) -> Result<HttpResponse, AppError> {
    // バリデーション（certification_name の文字数チェック等）
    body.validate()
        .map_err(|e| AppError::ValidationError(format!("{}", e)))?;

    let goal = goal_service::create(&db, user.0, &body).await?;
    Ok(HttpResponse::Created().json(goal))
}

// ============================================================
// update — PUT /api/goals/{id}
// ============================================================
/// 既存の学習目標を更新する（部分更新）
///
/// GoalUpdateRequest は全フィールドが Option のため、
/// 指定されたフィールドのみが更新される。
///
/// # 引数
/// - `path`: URL パスパラメータ（目標 ID の UUID）
/// - `body`: GoalUpdateRequest（target_date?, status?, memo?, study_hours?）
///
/// # 戻り値
/// - 成功: 200 OK + 更新後の目標情報
pub async fn update(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<GoalUpdateRequest>,
) -> Result<HttpResponse, AppError> {
    // `#[validate]` でメモの文字数チェック等を実行
    body.validate()
        .map_err(|e| AppError::ValidationError(format!("{}", e)))?;

    let goal = goal_service::update(&db, user.0, path.into_inner(), &body).await?;
    Ok(HttpResponse::Ok().json(goal))
}

// ============================================================
// delete — DELETE /api/goals/{id}
// ============================================================
/// 学習目標を削除する
///
/// # 戻り値
/// - 成功: 204 No Content
/// - 所有権なし: 404 Not Found
pub async fn delete(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    goal_service::delete(&db, user.0, path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
