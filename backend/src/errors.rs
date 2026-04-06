// ============================================================
// errors.rs — アプリケーション全体のエラー定義
// ============================================================
// Rust では `Result<T, E>` でエラーを表現する。
// このファイルでは E の型として使う `AppError` を定義し、
// HTTP レスポンスへの変換方法を実装する。

// Actix-web のHTTP応答・エラー応答の型をインポート
use actix_web::{HttpResponse, ResponseError};
// 標準ライブラリのフォーマット関連トレイト
use std::fmt;

// ============================================================
// AppError — エラーの種類を enum で列挙
// ============================================================
// `#[derive(Debug)]` で `{:?}` による出力を自動実装する
#[derive(Debug)]
pub enum AppError {
    /// バリデーションエラー（入力値が不正） → HTTP 400
    ValidationError(String),
    /// 未認証エラー（セッション切れ・Cookie なし） → HTTP 401
    Unauthorized(String),
    /// 権限エラー（他ユーザーのリソースへのアクセス） → HTTP 403
    Forbidden(String),
    /// リソースが見つからない → HTTP 404
    NotFound(String),
    /// 重複エラー（同一メール・同一お気に入り等） → HTTP 409
    Conflict(String),
    /// サーバー内部エラー（予期せぬエラー） → HTTP 500
    Internal(String),
}

// ============================================================
// Display トレイト — エラーを文字列として表示するための実装
// ============================================================
// Rust では `println!("{}", err)` や `format!("{}", err)` で使える
// 文字列表示を `fmt::Display` トレイトで定義する。
impl fmt::Display for AppError {
    /// エラーを人間が読める文字列に変換する
    ///
    /// # 引数
    /// - `f`: 書き込み先のフォーマッタ
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `match` で enum の各バリアントを分岐する
        match self {
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

// ============================================================
// ResponseError トレイト — AppError を HTTP レスポンスに変換
// ============================================================
// Actix-web のハンドラ関数が `Result<HttpResponse, AppError>` を返すとき、
// Err(AppError) が発生したときに自動的にこのメソッドが呼ばれる。
impl ResponseError for AppError {
    /// エラーの種類に応じた HTTP レスポンスを生成する
    ///
    /// # 戻り値
    /// JSON 形式のエラーレスポンス（`{ "error": "...", "code": "..." }`）
    fn error_response(&self) -> HttpResponse {
        // 各エラー種別をタプル (HTTPステータス, エラーコード, メッセージ) に変換
        let (status, code, message) = match self {
            AppError::ValidationError(msg) => {
                // 400 Bad Request: 入力値の問題はクライアント側の責任
                (actix_web::http::StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg.clone())
            }
            AppError::Unauthorized(msg) => {
                // 401 Unauthorized: 認証されていないリクエスト
                (actix_web::http::StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone())
            }
            AppError::Forbidden(msg) => {
                // 403 Forbidden: 認証済みだが権限がない
                (actix_web::http::StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone())
            }
            AppError::NotFound(msg) => {
                // 404 Not Found: リソースが存在しない
                (actix_web::http::StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone())
            }
            AppError::Conflict(msg) => {
                // 409 Conflict: 重複データ等の競合
                (actix_web::http::StatusCode::CONFLICT, "CONFLICT", msg.clone())
            }
            AppError::Internal(msg) => {
                // 500 Internal Server Error: サーバー側の予期せぬ問題
                // ⚠️ 詳細なエラー情報はログにのみ出力し、クライアントには汎用メッセージのみ返す
                // （セキュリティ上の理由：内部実装を露出しない）
                log::error!("Internal error: {}", msg);
                (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    // クライアントには「何かおかしい」とだけ伝える。詳細はログで確認する。
                    "サーバー内部エラーが発生しました".to_string(),
                )
            }
        };

        // `HttpResponse::build(status)` でステータスコードを指定したレスポンスを作成
        // `.json(...)` で JSON ボディを設定
        // `serde_json::json!({...})` は JSON リテラルマクロ
        HttpResponse::build(status).json(serde_json::json!({
            "error": message,
            "code": code,
        }))
    }
}

// ============================================================
// reqwest::Error → AppError への自動変換
// ============================================================
// `From` トレイトを実装すると、`?` 演算子で自動的に変換される。
// `db.select(...)await?` の `?` が失敗すると、reqwest::Error が
// AppError::Internal に自動変換される。
impl From<reqwest::Error> for AppError {
    /// HTTP クライアントエラーを内部エラーとして扱う
    ///
    /// # 引数
    /// - `err`: reqwest（HTTPクライアントライブラリ）のエラー
    fn from(err: reqwest::Error) -> Self {
        AppError::Internal(format!("HTTP client error: {}", err))
    }
}
