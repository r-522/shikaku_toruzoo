use crate::errors::AppError;
use reqwest::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct SupabaseClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl SupabaseClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        let mut url = base_url.trim_end_matches('/').to_string();
        if !url.ends_with("/rest/v1") {
            url.push_str("/rest/v1");
        }
        Self {
            client: Client::new(),
            base_url: url,
            api_key: api_key.to_string(),
        }
    }

    fn build_request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, format!("{}/{}", self.base_url, path))
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
    }

    pub async fn select(&self, table: &str, query: &str) -> Result<Value, AppError> {
        let url = if query.is_empty() {
            table.to_string()
        } else {
            format!("{}?{}", table, query)
        };

        let resp = self
            .build_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "Supabase SELECT failed ({}): {}",
                status, body
            )));
        }

        Ok(resp.json().await?)
    }

    pub async fn insert(&self, table: &str, body: &Value) -> Result<Value, AppError> {
        let resp = self
            .build_request(reqwest::Method::POST, table)
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let resp_body = resp.text().await.unwrap_or_default();
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

    pub async fn update(&self, table: &str, query: &str, body: &Value) -> Result<Value, AppError> {
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

        Ok(())
    }

    pub async fn rpc(&self, function: &str, body: &Value) -> Result<Value, AppError> {
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
