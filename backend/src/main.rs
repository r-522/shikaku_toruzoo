// ============================================================
// main.rs — アプリケーションのエントリポイント
// ============================================================
// Rust プログラムは `main` 関数から実行が始まる。
// このファイルではサーバーの起動・ルーティング・CORS設定を行う。
//
// 【アーキテクチャ概要】
// HTTP リクエスト → Actix-web → ハンドラ → サービス → SupabaseClient → Supabase DB
// フロントエンド（Vue）は /app/static/ の静的ファイルとして配信される。
// SPA のため、/api/* 以外のリクエストはすべて index.html を返す。

// `mod` はサブモジュールを宣言する。ここに書かれたモジュールは
// 同名のファイル（または同名ディレクトリの mod.rs）を読み込む。
mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;

// Actix CORS（Cross-Origin Resource Sharing）ミドルウェア
use actix_cors::Cors;
// 静的ファイル配信（Vue ビルド成果物）
use actix_files::Files;
// Actix-web の主要な型・マクロ
use actix_web::{http::header, web, App, HttpRequest, HttpResponse, HttpServer, middleware::Logger};
use config::Config;
use db::SupabaseClient;

// ============================================================
// ヘルスチェックエンドポイント
// ============================================================
/// GET /api/health — サーバーの死活確認用エンドポイント
///
/// Cloud Run のヘルスチェックや監視ツールが定期的に呼び出す。
/// 常に 200 OK と `{"status": "ok"}` を返す。
async fn health() -> HttpResponse {
    // `serde_json::json!({...})` はJSONリテラルを作成するマクロ
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

// ============================================================
// SPA フォールバック
// ============================================================
/// すべての非API・非静的ファイルリクエストに対して index.html を返す
///
/// Vue Router は「URLは変わるがHTMLは常に同じ」という SPA の仕組みを使う。
/// /certifications や /dashboard などのパスへの直接アクセスでも
/// index.html を返すことで Vue が URL を解釈してページを表示できる。
///
/// # 引数
/// - `config`: サーバー設定（静的ファイルディレクトリのパスを持つ）
/// - `_req`: HTTP リクエスト（未使用だが引数に必要なため `_` プレフィックスで無視）
async fn spa_fallback(config: web::Data<Config>, _req: HttpRequest) -> actix_web::Result<actix_files::NamedFile> {
    // index.html のフルパスを組み立てる
    let index_path = format!("{}/index.html", config.static_dir);
    // ファイルを開いて返す（`?` で IO エラーを Actix-web のエラーに変換）
    Ok(actix_files::NamedFile::open(index_path)?)
}

// ============================================================
// メイン関数
// ============================================================
// `#[actix_web::main]` マクロ: 非同期 main 関数を Actix の非同期ランタイム上で実行する
// 通常 Rust の main は同期だが、Web サーバーは非同期処理が必要なためこのマクロが必要
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // .env ファイルが存在すれば環境変数として読み込む（`.ok()` でエラーを無視）
    // 本番環境（Cloud Run）では .env は不要で、実際の環境変数が使われる
    dotenvy::dotenv().ok();

    // ログ出力を初期化する（RUST_LOG 環境変数でレベルを制御）
    // 例: RUST_LOG=info でINFO以上のログが出力される
    env_logger::init();

    // 環境変数から設定を読み込む
    let config = Config::from_env();
    let port = config.port();
    // `clone()` で String を複製する（move クロージャ内で使うため）
    let static_dir = config.static_dir.clone();
    let cors_origin = config.cors_origin.clone();

    // Supabase クライアントを生成（接続プールを内部で管理）
    let supabase = SupabaseClient::new(&config.supabase_url, &config.supabase_key);

    log::info!("Starting CertManager on port {}", port);

    // ============================================================
    // HTTP サーバーの設定と起動
    // ============================================================
    // `HttpServer::new(|| {...})` はサーバーファクトリ。
    // Actix-web はスレッドごとに App インスタンスを作るため、
    // `move` クロージャで変数を各スレッドにコピーする。
    HttpServer::new(move || {
        // ---- CORS（クロスオリジンリソース共有）の設定 ----
        // ブラウザは異なるオリジン（ドメイン）へのリクエストを制限する。
        // CORS ヘッダーを返すことで、許可されたオリジンからのリクエストのみ通す。
        let cors = Cors::default()
            // 許可するオリジン（フロントエンドのURL）のみ
            .allowed_origin(&cors_origin)
            // 許可する HTTP メソッド
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            // 許可するリクエストヘッダー
            .allowed_headers(vec![
                header::CONTENT_TYPE,     // Content-Type: application/json
                header::AUTHORIZATION,    // Authorization: Bearer ...
                // X-Requested-With: CSRF 対策のカスタムヘッダー
                header::HeaderName::from_static("x-requested-with"),
            ])
            // Cookie を含むリクエスト（認証に必要）を許可
            .supports_credentials()
            // プリフライトリクエストのキャッシュ時間（秒）
            .max_age(3600);

        // ---- App の構築 ----
        App::new()
            // リクエストログを出力するミドルウェア
            .wrap(Logger::default())
            // CORS ミドルウェアを適用
            .wrap(cors)
            // アプリ全体で共有するデータ（各ハンドラから `web::Data<T>` で取り出せる）
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(supabase.clone()))

            // ---- API ルートの登録 ----
            // ヘルスチェック
            .route("/api/health", web::get().to(health))

            // 認証エンドポイント群（/api/auth/xxx）
            .service(
                web::scope("/api/auth")
                    .route("/signup", web::post().to(handlers::auth::signup))
                    .route("/signin", web::post().to(handlers::auth::signin))
                    .route("/signout", web::post().to(handlers::auth::signout))
                    .route("/me", web::get().to(handlers::auth::me)),
            )

            // 所持資格エンドポイント群（/api/certifications）
            .service(
                web::scope("/api/certifications")
                    .route("", web::get().to(handlers::certification::list))
                    .route("", web::post().to(handlers::certification::create))
                    // `{id}` はパスパラメータ（例: /api/certifications/uuid-here）
                    .route("/{id}", web::put().to(handlers::certification::update))
                    .route("/{id}", web::delete().to(handlers::certification::delete)),
            )

            // 目標エンドポイント群
            .service(
                web::scope("/api/goals")
                    .route("", web::get().to(handlers::goal::list))
                    .route("", web::post().to(handlers::goal::create))
                    .route("/{id}", web::put().to(handlers::goal::update))
                    .route("/{id}", web::delete().to(handlers::goal::delete)),
            )

            // 資格マスタ（入力補完）エンドポイント
            .service(
                web::scope("/api/master")
                    .route("/certifications", web::get().to(handlers::master::search)),
            )

            // コミュニティエンドポイント群
            .service(
                web::scope("/api/community")
                    .route("/users", web::get().to(handlers::community::list_users))
                    .route("/users/{id}", web::get().to(handlers::community::get_user)),
            )

            // お気に入りエンドポイント群
            .service(
                web::scope("/api/favorites")
                    .route("", web::get().to(handlers::favorite::list))
                    // `{userId}` はお気に入り対象ユーザーの ID
                    .route("/{userId}", web::post().to(handlers::favorite::add))
                    .route("/{userId}", web::delete().to(handlers::favorite::remove)),
            )

            // ---- 静的ファイル配信 ----
            // Vue のビルド成果物（JS, CSS, 画像等）を配信する
            // `.index_file("index.html")` で "/" へのアクセスに index.html を返す
            .service(Files::new("/", &static_dir).index_file("index.html"))

            // ---- SPA フォールバック ----
            // 静的ファイルに一致しないパス（例: /dashboard, /community/:id）へのアクセスに
            // index.html を返す（Vue Router がクライアント側でルーティングを処理する）
            .default_service(web::get().to(spa_fallback))
    })
    // 全インターフェースの指定ポートでリッスン開始
    // "0.0.0.0" はすべてのネットワークインターフェースを意味する（Cloud Run で必要）
    .bind(("0.0.0.0", port))?
    // サーバーを起動して終了まで待機
    .run()
    .await
}
