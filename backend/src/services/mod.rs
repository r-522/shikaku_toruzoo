// ============================================================
// services/mod.rs — サービス層モジュールの入り口
// ============================================================
// サービス層はビジネスロジック（データの検証・変換・DB操作の組み合わせ）を担う。
// ハンドラ（HTTP層）はリクエストの受け取りのみ行い、
// 実際の処理はすべてこのサービス層に委譲する設計になっている。

/// 認証サービス（サインアップ・サインイン・サインアウト・セッション管理）
pub mod auth_service;

/// 所持資格サービス（TBL_HOLDING の CRUD）
pub mod cert_service;

/// 目標サービス（TBL_GOAL の CRUD・勉強時間管理）
pub mod goal_service;

/// 資格マスタサービス（TBL_MASTER の検索・自動作成）
pub mod master_service;

/// コミュニティサービス（他ユーザーの資格・目標・勉強時間の集計）
pub mod community_service;

/// お気に入りサービス（TBL_FAVORITE の登録・解除・一覧）
pub mod favorite_service;
