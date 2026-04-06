// ============================================================
// models/mod.rs — モデルモジュールの入り口
// ============================================================
// 各テーブルに対応するデータ構造（struct）を持つモジュールを公開する。
// `pub mod` で宣言することで `crate::models::user::User` のように
// 他ファイルから参照できるようになる。

/// ユーザーテーブル（TBL_USER）関連の構造体
pub mod user;

/// セッションテーブル（TBL_SESSION）関連の構造体
pub mod session;

/// 所持資格テーブル（TBL_HOLDING）関連の構造体
pub mod certification;

/// 目標テーブル（TBL_GOAL）関連の構造体
pub mod goal;

/// 資格マスタテーブル（TBL_MASTER）関連の構造体
pub mod master;

/// お気に入りテーブル（TBL_FAVORITE）関連の構造体
pub mod favorite;
