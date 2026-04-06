// ============================================================
// middleware/auth.rs — 認証ミドルウェア
// ============================================================
// このファイルは「認証済みユーザー」を表す `AuthenticatedUser` 型を定義する。
//
// 【仕組み】
// Actix-web では、ハンドラ関数の引数に `AuthenticatedUser` を書くだけで
// 自動的に Cookie のセッションを検証してユーザーIDを取り出してくれる。
// 認証が必要なハンドラは全てこの型を引数に取る。
//
// 【認証フロー】
// リクエスト → Cookie "session_token" を取得 → DB で検索・期限確認 → ユーザーID返却

// Actix-web の型（リクエスト・ペイロード・抽出子の定義用）
use actix_web::{dev::Payload, FromRequest, HttpRequest, web};
// `ready` は即値を Future（非同期の値）として包むヘルパー
use std::future::{Ready, ready};
// UUID 型
use uuid::Uuid;

// 自モジュールへの参照
use crate::db::SupabaseClient;
use crate::errors::AppError;

// ============================================================
// AuthenticatedUser — 認証済みユーザーを表す型
// ============================================================
// タプル構造体（`(Uuid)` を1フィールドだけ持つ構造体）
// `user.0` でユーザーIDを取り出す。
//
// ハンドラ関数の引数に `user: AuthenticatedUser` と書くと、
// Actix-web が `FromRequest` トレイトの実装を呼び出して
// 自動的にセッション検証を行ってくれる。
#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub Uuid);

// ============================================================
// FromRequest トレイトの実装
// ============================================================
// `FromRequest` は「HTTP リクエストからこの型を取り出す方法」を定義するトレイト。
// ハンドラ引数として使うために必須の実装。
impl FromRequest for AuthenticatedUser {
    // エラー型
    type Error = AppError;
    // Future型（非同期処理の結果を表す型。Pinはメモリの位置を固定する）
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    /// HTTP リクエストから AuthenticatedUser を取り出す
    ///
    /// この関数は Actix-web が自動的に呼ぶ。
    /// 直接呼ぶことはない。
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // リクエストのクローンを作る（非同期ブロック内で借用できないため）
        let req = req.clone();

        // `Box::pin(async move { ... })` で非同期処理をボックス化して返す
        // `move` はブロック内でローカル変数（req）を所有権ごと移動させる
        Box::pin(async move {
            // ---- ステップ1: Cookie からセッショントークンを取得 ----
            let token = req
                // Cookie 名 "session_token" を探す
                .cookie("session_token")
                // Cookie が見つかれば値（文字列）を取り出す
                .map(|c| c.value().to_string())
                // Cookie がなければ 401 エラーを返す
                // `ok_or_else` は None を Err に変換する
                .ok_or_else(|| AppError::Unauthorized("認証が必要です".to_string()))?;

            // ---- ステップ2: app_data から DB クライアントを取得 ----
            // `app_data` は main.rs で登録したアプリ全体のデータを取り出す
            let db = req
                .app_data::<web::Data<SupabaseClient>>()
                .ok_or_else(|| AppError::Internal("Database client not configured".to_string()))?;

            // ---- ステップ3: DB でセッションを検索・期限確認 ----
            // `sestk=eq.{token}` はトークンが一致するレコードを検索
            // `sesea=gt.now()` は有効期限が現在時刻より大きい（未来）ものだけを返す
            // `urlencoding::encode` は URL に使えない文字をエスケープする（Base64の+や/対策）
            let query = format!(
                "select=sesui&sestk=eq.{}&sesea=gt.now()",
                urlencoding::encode(&token)
            );
            let result = db.select("TBL_SESSION", &query).await?;

            // JSON 配列を Vec<Value> に変換
            let sessions: Vec<serde_json::Value> = serde_json::from_value(result)
                .map_err(|e| AppError::Internal(format!("Session parse error: {}", e)))?;

            // セッションが0件なら「無効なセッション」エラー
            // `.first()` は最初の要素への参照を返す（空の場合は None）
            let session = sessions
                .first()
                .ok_or_else(|| AppError::Unauthorized("無効なセッションです".to_string()))?;

            // ---- ステップ4: ユーザーIDを取り出す ----
            // "sesui" フィールドを文字列として取り出し、UUID にパース
            let user_id = session["sesui"]
                .as_str()                          // JSON の文字列値を取り出す（&str）
                .and_then(|s| Uuid::parse_str(s).ok()) // UUID にパース（失敗なら None）
                .ok_or_else(|| AppError::Internal("Session user ID parse error".to_string()))?;

            // AuthenticatedUser にユーザーIDを包んで返す
            Ok(AuthenticatedUser(user_id))
        })
    }
}
