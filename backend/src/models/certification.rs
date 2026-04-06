// ============================================================
// models/certification.rs — 所持資格モデル（TBL_HOLDING）
// ============================================================
// 「所持資格」とは、ユーザーが既に取得済みの資格のこと。
// TBL_HOLDING テーブルに格納される（holding = 所持）。
// 各レコードはユーザー（TBL_USER）と資格マスタ（TBL_MASTER）を結びつける。

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// TBL_HOLDING テーブルのレコードに対応する構造体（DB取得用）
///
/// DB カラム名の対応:
/// - holid → 所持資格レコードのID（主キー）
/// - holui → ユーザーID（user id）
/// - holmi → マスタID（master id）→ 資格名はマスタから取得する
/// - holdt → 取得日（date）
/// - holca → 作成日時
/// - holua → 更新日時
#[derive(Debug, Deserialize)]
pub struct Holding {
    pub holid: Uuid,
    /// このレコードを所有するユーザーの ID
    pub holui: Uuid,
    /// この資格の名称が登録されている TBL_MASTER のレコード ID
    pub holmi: Uuid,
    /// 資格の取得日（任意。未設定の場合は None）
    pub holdt: Option<String>,
    pub holca: String,
    pub holua: String,
}

/// 所持資格の登録・更新リクエストボディ
///
/// POST /api/certifications および PUT /api/certifications/:id に対応。
#[derive(Debug, Deserialize, Validate)]
pub struct CertificationRequest {
    /// 資格名（1〜200文字）
    /// フロントエンドの入力補完コンポーネントから入力される
    #[validate(length(min = 1, max = 200, message = "資格名は1〜200文字で入力してください"))]
    pub certification_name: String,

    /// 資格マスタの ID（任意）
    /// - 補完候補から選択した場合: 既存マスタの UUID
    /// - 新規資格名を入力した場合: None（マスタに自動追加される）
    pub master_id: Option<Uuid>,

    /// 資格の取得日（YYYY-MM-DD 形式、任意）
    pub acquired_date: Option<String>,
}

/// 所持資格の API レスポンスボディ
///
/// DB の生データをクライアントが扱いやすい形に変換したもの。
#[derive(Debug, Serialize)]
pub struct CertificationResponse {
    /// 所持資格レコードの ID
    pub id: Uuid,
    /// 資格名（TBL_MASTER の masnm から取得）
    pub certification_name: String,
    /// 紐づく資格マスタの ID
    pub master_id: Uuid,
    /// 取得日（未設定の場合は null）
    pub acquired_date: Option<String>,
    /// 登録日時
    pub created_at: String,
}
