// ============================================================
// models/master.rs — 資格マスタモデル（TBL_MASTER）
// ============================================================
// 資格マスタは全ユーザーで共有する「資格名の辞書」。
// 入力補完の候補データとして使用し、重複登録を防ぐ。

// Serialize: Rust の struct → JSON に変換するために必要（APIレスポンス用）
// Deserialize: JSON → Rust の struct に変換するために必要（DB取得データ用）
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// TBL_MASTER テーブルのレコードに対応する構造体（DB取得用）
///
/// DB カラム名の対応:
/// - masid → マスタID
/// - masnm → 資格名（name）
/// - masct → カテゴリ（category）
/// - masnr → 正規化名（normalized）※重複判定に使用
/// - masca → 作成日時（created at）
#[derive(Debug, Deserialize)]
pub struct Master {
    /// マスタ ID（主キー）
    pub masid: Uuid,

    /// 資格名（表示用）例: "基本情報技術者"
    pub masnm: String,

    /// カテゴリ（IT / 語学 / 金融 / 法律 / その他）
    pub masct: String,

    /// 正規化済み資格名（重複チェック用）
    /// 小文字化 + 空白除去した文字列。例: "基本情報技術者" → "基本情報技術者"
    pub masnr: String,

    /// レコードの作成日時
    pub masca: String,
}

/// 資格マスタの検索結果を API レスポンスとして返すための構造体
///
/// DB の生データ（Master）とは別に、クライアントに返すフォーマットを定義する。
/// フロントエンドの入力補完候補として使用される。
#[derive(Debug, Serialize)]
pub struct MasterSearchResult {
    /// マスタ ID（フロントエンドが選択時に master_id として送信する）
    pub id: Uuid,

    /// 資格名（入力補完の候補として表示される）
    pub name: String,

    /// カテゴリ（補完候補に小さく表示される）
    pub category: String,
}
