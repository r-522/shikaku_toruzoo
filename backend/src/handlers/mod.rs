// ============================================================
// handlers/mod.rs — ハンドラ層モジュールの入り口
// ============================================================
// ハンドラ層は HTTP リクエストを受け取り、サービス層を呼び出し、
// HTTP レスポンスを返す責務を持つ。
// いわゆる「コントローラ」に相当する層。

/// 認証ハンドラ（signup / signin / signout / me）
pub mod auth;

/// 所持資格ハンドラ（list / create / update / delete）
pub mod certification;

/// 目標ハンドラ（list / create / update / delete）
pub mod goal;

/// 資格マスタハンドラ（search）
pub mod master;

/// コミュニティハンドラ（list_users / get_user）
pub mod community;

/// お気に入りハンドラ（add / remove / list）
pub mod favorite;
