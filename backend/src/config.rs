use std::env;

#[derive(Clone)]
pub struct Config {
    pub supabase_url: String,
    pub supabase_key: String,
    pub email_hmac_secret: String,
    pub server_port: u16,
    pub cors_origin: String,
    pub static_dir: String,
}

impl Config {
    pub fn port(&self) -> u16 {
        self.server_port
    }

    pub fn from_env() -> Self {
        Self {
            supabase_url: env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
            supabase_key: env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
            email_hmac_secret: env::var("EMAIL_HMAC_SECRET")
                .expect("EMAIL_HMAC_SECRET must be set"),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid port number"),
            cors_origin: env::var("CORS_ORIGIN").expect("CORS_ORIGIN must be set"),
            static_dir: env::var("STATIC_DIR").unwrap_or_else(|_| "./static".to_string()),
        }
    }
}
