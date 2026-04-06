// ============================================================
// db.rs — Supabase PostgREST クライアント
// ============================================================
// このファイルは Supabase（PostgreSQL）との通信を担う。
//
// 【Supabase PostgREST とは？】
// Supabase はデータベース（PostgreSQL）を REST API として公開する。
// テーブル名と条件をURLに含めてHTTPリクエストを送ることで
// SQL文を書かずにデータの取得・更新ができる。
//
// 例: GET /rest/v1/TBL_USER?useid=eq.{uuid}&select=useid,usenm
//     → SELECT useid, usenm FROM TBL_USER WHERE useid = '{uuid}'
//
// 【セキュリティ】
// service_role キーを使うことで RLS（Row Level Security）をバイパスし、
// 全テーブルへのフルアクセスが可能になる。
// このキーは絶対に外部に漏らしてはいけない。

use crate::errors::AppError;
// reqwest は HTTP クライアントライブラリ
use reqwest::Client;
// serde_json の Value は任意の JSON 値を表す型
use serde_json::Value;

/// Supabase PostgREST へのリクエストを担うクライアント
///
/// `Clone` を derive することで Actix-web の各スレッドにコピーできる。
#[derive(Clone)]
pub struct SupabaseClient {
    /// HTTP クライアント（接続プールを管理する）
    client: Client,
    /// Supabase REST API のベース URL（例: https://xxx.supabase.co/rest/v1）
    base_url: String,
    /// Supabase の service_role キー（認証に使用）
    api_key: String,
}

impl SupabaseClient {
    /// 新しい SupabaseClient を作成する
    ///
    /// # 引数
    /// - `base_url`: Supabase プロジェクトの URL
    ///   - `https://xxx.supabase.co` でも `https://xxx.supabase.co/rest/v1` でも OK
    ///   - 末尾スラッシュの有無も自動で正規化する
    /// - `api_key`: service_role キー
    ///
    /// # 戻り値
    /// 正規化された URL を持つ SupabaseClient
    pub fn new(base_url: &str, api_key: &str) -> Self {
        // 末尾のスラッシュを除去する（"https://xxx.supabase.co/" → "https://xxx.supabase.co"）
        let mut url = base_url.trim_end_matches('/').to_string();

        // "/rest/v1" が付いていない場合は自動追加する
        // これにより環境変数に "/rest/v1" を付け忘れても動作する
        if !url.ends_with("/rest/v1") {
            url.push_str("/rest/v1");
        }

        Self {
            // `Client::new()` はデフォルト設定の HTTP クライアントを作成
            client: Client::new(),
            base_url: url,
            api_key: api_key.to_string(),
        }
    }

    /// 共通のリクエストビルダーを作成する（内部ヘルパー）
    ///
    /// すべてのリクエストに共通して必要な
    /// API キーと Authorization ヘッダーをここで設定する。
    ///
    /// # 引数
    /// - `method`: HTTP メソッド（GET / POST / PATCH / DELETE）
    /// - `path`: テーブル名またはRPCパス（例: "TBL_USER", "rpc/my_function"）
    fn build_request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            // URL を組み立てる: "https://xxx.supabase.co/rest/v1/TBL_USER"
            .request(method, format!("{}/{}", self.base_url, path))
            // Supabase の認証ヘッダー（anon/service_role キーを指定）
            .header("apikey", &self.api_key)
            // Bearer トークンとして認証（JWT 形式）
            .header("Authorization", format!("Bearer {}", self.api_key))
    }

    /// テーブルからデータを取得する（SELECT相当）
    ///
    /// # 引数
    /// - `table`: テーブル名（例: "TBL_USER"）
    /// - `query`: PostgREST のクエリ文字列
    ///   - 例: `"select=useid,usenm&useid=eq.{uuid}"` → `WHERE useid = '{uuid}'` に相当
    ///   - 例: `"select=holid,holdt,TBL_MASTER(masid,masnm)"` → JOIN に相当
    ///
    /// # 戻り値
    /// JSON の配列（`serde_json::Value`）
    ///
    /// # 使用例
    /// ```
    /// let result = db.select("TBL_USER", "select=useid,usenm&useid=eq.xxx").await?;
    /// ```
    pub async fn select(&self, table: &str, query: &str) -> Result<Value, AppError> {
        // クエリが空の場合はそのままテーブル名のみ、ある場合は "?" でつなぐ
        let url = if query.is_empty() {
            table.to_string()
        } else {
            format!("{}?{}", table, query)
        };

        // HTTP GET リクエストを送信して応答を待つ
        // `await` は非同期処理の完了を待つキーワード
        let resp = self
            .build_request(reqwest::Method::GET, &url)
            .send()
            .await?; // `?` でエラーを伝播（From<reqwest::Error> が実装済みなので自動変換）

        // 成功でない場合（4xx, 5xx）はエラーを返す
        if !resp.status().is_success() {
            let status = resp.status();
            // レスポンスボディを文字列として取得（デバッグ用）
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "Supabase SELECT failed ({}): {}",
                status, body
            )));
        }

        // レスポンスを JSON に変換して返す
        Ok(resp.json().await?)
    }

    /// テーブルにデータを挿入する（INSERT相当）
    ///
    /// `Prefer: return=representation` ヘッダーにより、
    /// 挿入後のレコード（自動生成された ID や日時を含む）がレスポンスで返ってくる。
    ///
    /// # 引数
    /// - `table`: テーブル名
    /// - `body`: 挿入するデータ（JSON オブジェクト）
    ///
    /// # 戻り値
    /// 挿入されたレコードの JSON（配列形式）
    pub async fn insert(&self, table: &str, body: &Value) -> Result<Value, AppError> {
        let resp = self
            .build_request(reqwest::Method::POST, table)
            .header("Content-Type", "application/json")
            // "return=representation" で挿入後のデータをレスポンスとして返してもらう
            .header("Prefer", "return=representation")
            .json(body) // body を JSON としてシリアライズして送信
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let resp_body = resp.text().await.unwrap_or_default();
            // 409 Conflict（重複データ）の場合は専用エラーを返す
            if status == reqwest::StatusCode::CONFLICT {
                return Err(AppError::Conflict(resp_body));
            }
            return Err(AppError::Internal(format!(
                "Supabase INSERT failed ({}): {}",
                status, resp_body
            )));
        }

        Ok(resp.json().await?)
    }

    /// テーブルのデータを更新する（UPDATE相当）
    ///
    /// HTTP の PATCH メソッドを使用（PUT は全フィールドの上書き、PATCH は部分更新）。
    ///
    /// # 引数
    /// - `table`: テーブル名
    /// - `query`: 更新対象を絞り込む条件（例: `"goaid=eq.{uuid}&goaui=eq.{user_id}"`）
    ///   - ⚠️ `&goaui=eq.{user_id}` を含めることで「自分のレコードのみ更新」を保証する
    /// - `body`: 更新するフィールドと値
    ///
    /// # 戻り値
    /// 更新後のレコードの JSON（配列形式）
    pub async fn update(&self, table: &str, query: &str, body: &Value) -> Result<Value, AppError> {
        // "TABLE?条件" の形式で URL を組み立てる
        let url = format!("{}?{}", table, query);

        let resp = self
            .build_request(reqwest::Method::PATCH, &url)
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let resp_body = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "Supabase UPDATE failed ({}): {}",
                status, resp_body
            )));
        }

        Ok(resp.json().await?)
    }

    /// テーブルからデータを削除する（DELETE相当）
    ///
    /// # 引数
    /// - `table`: テーブル名
    /// - `query`: 削除対象を絞り込む条件
    ///   - ⚠️ 条件を空にすると全件削除になるため、必ず条件を指定すること
    ///
    /// # 戻り値
    /// 成功した場合は `Ok(())`（削除は結果データを返さない）
    pub async fn delete(&self, table: &str, query: &str) -> Result<(), AppError> {
        let url = format!("{}?{}", table, query);

        let resp = self
            .build_request(reqwest::Method::DELETE, &url)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let resp_body = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "Supabase DELETE failed ({}): {}",
                status, resp_body
            )));
        }

        // 削除成功時はボディなしで `Ok(())` を返す
        Ok(())
    }

    /// Supabase の RPC（ストアドプロシージャ）を呼び出す
    ///
    /// 現在のコードでは直接使用していないが、将来の拡張のために残してある。
    ///
    /// # 引数
    /// - `function`: 関数名（例: "get_community_users"）
    /// - `body`: 関数に渡す引数（JSON オブジェクト）
    ///
    /// # 戻り値
    /// 関数の戻り値（JSON）
    pub async fn rpc(&self, function: &str, body: &Value) -> Result<Value, AppError> {
        // RPC の URL は "rpc/{関数名}" の形式
        let url = format!("rpc/{}", function);

        let resp = self
            .build_request(reqwest::Method::POST, &url)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let resp_body = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "Supabase RPC failed ({}): {}",
                status, resp_body
            )));
        }

        Ok(resp.json().await?)
    }
}
