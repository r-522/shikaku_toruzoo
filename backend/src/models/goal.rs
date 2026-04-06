// ============================================================
// models/goal.rs — 目標モデル（TBL_GOAL）
// ============================================================
// 「目標」とは、ユーザーがこれから取得を目指す資格の記録。
// ステータス・勉強時間・メモ・目標日などを管理する。

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// TBL_GOAL テーブルのレコードに対応する構造体（DB取得用）
///
/// DB カラム名の対応:
/// - goaid → 目標ID（主キー）
/// - goaui → ユーザーID
/// - goami → マスタID（どの資格を目指しているか）
/// - goatd → 目標日（target date）
/// - goast → ステータス（status）
/// - goamm → メモ（memo）
/// - goash → 勉強時間（study hours）
/// - goaca → 作成日時
/// - goaua → 更新日時
#[derive(Debug, Deserialize)]
pub struct Goal {
    pub goaid: Uuid,
    pub goaui: Uuid,
    pub goami: Uuid,
    pub goatd: String,
    /// ステータス（有効値: exam_date / passed / failed / abandoned）
    pub goast: String,
    /// メモ（任意、最大1000文字）
    pub goamm: Option<String>,
    /// 勉強時間（時間単位、0.5h 単位で記録）
    pub goash: Option<f64>,
    pub goaca: String,
    pub goaua: String,
}

/// 目標の登録リクエストボディ
///
/// POST /api/goals に対応。
#[derive(Debug, Deserialize, Validate)]
pub struct GoalRequest {
    /// 資格名（1〜200文字）
    #[validate(length(min = 1, max = 200, message = "資格名は1〜200文字で入力してください"))]
    pub certification_name: String,

    /// 資格マスタ ID（任意。補完候補から選択した場合は UUID、新規入力は None）
    pub master_id: Option<Uuid>,

    /// 目標日（YYYY-MM-DD 形式）
    pub target_date: String,

    /// ステータス（任意。未指定なら "exam_date"）
    /// 有効値: "exam_date"（受験日設定）/ "passed"（合格）/ "failed"（不合格）/ "abandoned"（断念）
    pub status: Option<String>,

    /// メモ（任意、最大1000文字）
    #[validate(length(max = 1000, message = "メモは1000文字以内で入力してください"))]
    pub memo: Option<String>,

    /// 勉強時間（任意、時間単位）
    pub study_hours: Option<f64>,
}

/// 目標の更新リクエストボディ
///
/// PUT /api/goals/:id に対応。
/// すべてのフィールドが Option なのは「部分更新」のため。
/// 送られてきたフィールドだけを更新し、送られてこなかったものは変更しない。
#[derive(Debug, Deserialize, Validate)]
pub struct GoalUpdateRequest {
    /// 更新後の目標日（任意）
    pub target_date: Option<String>,
    /// 更新後のステータス（任意）
    pub status: Option<String>,
    /// 更新後のメモ（任意、最大1000文字）
    #[validate(length(max = 1000, message = "メモは1000文字以内で入力してください"))]
    pub memo: Option<String>,
    /// 更新後の勉強時間（任意）
    pub study_hours: Option<f64>,
}

/// 目標の API レスポンスボディ
///
/// GET/POST/PUT /api/goals で返されるデータ形式。
#[derive(Debug, Serialize)]
pub struct GoalResponse {
    pub id: Uuid,
    /// 資格名（TBL_MASTER から取得）
    pub certification_name: String,
    pub master_id: Uuid,
    pub target_date: String,
    /// ステータス文字列（"exam_date" / "passed" / "failed" / "abandoned"）
    pub status: String,
    pub memo: Option<String>,
    /// 勉強時間（デフォルト: 0.0）
    pub study_hours: f64,
    pub created_at: String,
}
