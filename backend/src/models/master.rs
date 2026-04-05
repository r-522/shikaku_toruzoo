use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Master {
    pub masid: Uuid,
    pub masnm: String,
    pub masct: String,
    pub masnr: String,
    pub masca: String,
}

#[derive(Debug, Serialize)]
pub struct MasterSearchResult {
    pub id: Uuid,
    pub name: String,
    pub category: String,
}
