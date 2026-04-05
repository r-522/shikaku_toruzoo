use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Session {
    pub sesid: Uuid,
    pub sesui: Uuid,
    pub sestk: String,
    pub sesea: String,
    pub sesca: String,
}
