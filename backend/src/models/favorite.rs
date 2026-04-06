// ============================================================
// models/favorite.rs — お気に入りモデル（TBL_FAVORITE）
// ============================================================
// ユーザーが他のユーザーをお気に入りに登録する機能のデータ構造。
// お気に入りにしたユーザーはコミュニティ一覧の上部に優先表示される。

use serde::Deserialize;
use uuid::Uuid;

/// TBL_FAVORITE テーブルのレコードに対応する構造体
///
/// DB カラム名の対応:
/// - favid → お気に入りレコードのID（主キー）
/// - favui → お気に入りを登録したユーザーのID（user id）
/// - favti → お気に入り対象ユーザーのID（target id）
/// - favca → 作成日時
///
/// DB の UNIQUE(favui, favti) 制約により、同じペアは1件しか登録できない。
/// CHECK(favui != favti) 制約により、自分自身をお気に入りにはできない。
#[derive(Debug, Deserialize)]
pub struct Favorite {
    /// お気に入りレコードの ID（主キー）
    pub favid: Uuid,

    /// お気に入りを登録したユーザーの ID（「この人が」登録した）
    pub favui: Uuid,

    /// お気に入りに登録された対象ユーザーの ID（「この人を」登録した）
    pub favti: Uuid,

    /// レコードの作成日時（お気に入り登録日時）
    pub favca: String,
}
