// ============================================================
// models/session.rs — セッションモデル（TBL_SESSION）
// ============================================================
// セッションはユーザーの「ログイン状態」を管理するデータ。
// Cookie に保存したトークン文字列と DB のレコードを照合することで認証を行う。

// Deserialize: JSON → Rust の struct に変換するために必要
use serde::Deserialize;
// UUID: 主キーや外部キーの型
use uuid::Uuid;

/// TBL_SESSION テーブルのレコードに対応する構造体
///
/// DB カラム名（ses始まりの省略形）と Rust のフィールド名の対応:
/// - sesid → セッションID（主キー）
/// - sesui → ユーザーID（TBL_USER への外部キー）
/// - sestk → セッショントークン（Cookie に保存する値）
/// - sesea → 有効期限（expire at）
/// - sesca → 作成日時（created at）
#[derive(Debug, Deserialize)]
pub struct Session {
    /// セッション ID（主キー・UUID）
    pub sesid: Uuid,

    /// このセッションを持つユーザーの ID（TBL_USER.useid への参照）
    pub sesui: Uuid,

    /// セッショントークン（Base64の乱数文字列。Cookie に格納される値）
    pub sestk: String,

    /// セッションの有効期限（ISO8601 形式の日時文字列）
    /// この日時を過ぎるとセッションは無効になる
    pub sesea: String,

    /// レコードの作成日時
    pub sesca: String,
}
