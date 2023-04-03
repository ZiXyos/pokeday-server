use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct AccountDataResponse<T> {
    pub data: Option<T>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    
    pub id: String,
    pub username: String,

    pub created_at: String,
}

pub type AccountReq = AccountDataResponse<String>;
