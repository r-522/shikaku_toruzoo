use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Favorite {
    pub favid: Uuid,
    pub favui: Uuid,
    pub favti: Uuid,
    pub favca: String,
}
