// ============================================================
// config.rs — アプリケーション設定
// ============================================================
// 環境変数からアプリの設定値を読み込む。
// Cloud Run では環境変数で機密情報（APIキー等）を渡すのが一般的。
// `.env` ファイルはローカル開発用。本番では環境変数を直接設定する。

// `std::env` は Rust 標準ライブラリの環境変数操作モジュール
use std::env;

// `#[derive(Clone)]` は Config をコピー（クローン）できるようにするマクロ。
// Actix-web は複数スレッドで動くため、設定を各スレッドにコピーする必要がある。
#[derive(Clone)]
pub struct Config {
    /// Supabase の REST API ベース URL
    /// 例: https://xxx.supabase.co
    pub supabase_url: String,

    /// Supabase の service_role キー（RLS をバイパスして全テーブルにアクセス可能）
    /// ⚠️ anon キーではなく service_role キーを使うこと
    pub supabase_key: String,

    /// メールアドレスをHMAC-SHA256でハッシュ化するときのシークレットキー
    /// ⚠️ 一度設定したら絶対に変更してはいけない（全ユーザーのログインが不能になる）
    pub email_hmac_secret: String,

    /// サーバーがリッスンするポート番号（デフォルト: 8080）
    pub server_port: u16,

    /// CORS で許可するフロントエンドのオリジン（例: https://my-app.run.app）
    pub cors_origin: String,

    /// Vue ビルド成果物が格納されているディレクトリのパス
    /// デフォルト: ./static
    pub static_dir: String,
}

impl Config {
    /// ポート番号を返すヘルパーメソッド
    ///
    /// # 戻り値
    /// サーバーがリッスンするポート番号（u16型）
    pub fn port(&self) -> u16 {
        // `self.server_port` はこの構造体自身のフィールドにアクセスする
        self.server_port
    }

    /// 環境変数から Config を構築する
    ///
    /// 必須の環境変数が設定されていない場合はパニック（起動失敗）する。
    /// これは起動時に問題を早期発見するための意図的な設計。
    ///
    /// # 戻り値
    /// 環境変数から生成した Config インスタンス
    pub fn from_env() -> Self {
        Self {
            // `env::var("KEY")` は環境変数を取得する。`Result` 型を返す。
            // `.expect("...")` は取得失敗時にパニックしてメッセージを表示する。
            supabase_url: env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
            supabase_key: env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
            email_hmac_secret: env::var("EMAIL_HMAC_SECRET")
                .expect("EMAIL_HMAC_SECRET must be set"),

            // SERVER_PORT は省略可能（デフォルト8080）。
            // `unwrap_or_else` は Err のときにクロージャ（|_| は引数を無視する無名関数）を実行する。
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string()) // 未設定なら "8080" を使う
                .parse() // 文字列 → u16 に変換
                .expect("SERVER_PORT must be a valid port number"),

            cors_origin: env::var("CORS_ORIGIN").expect("CORS_ORIGIN must be set"),

            // STATIC_DIR も省略可能（デフォルト ./static）
            static_dir: env::var("STATIC_DIR").unwrap_or_else(|_| "./static".to_string()),
        }
    }
}
