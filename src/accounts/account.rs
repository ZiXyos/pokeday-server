use crate::accounts::types::{Account, AccountReq};

impl Account {
    pub fn new(username: String) -> Self {
        Self{
            id: "id".to_string(),
            username,
            created_at: "now".to_string()
        }
    }
}

impl AccountReq {

    pub fn to_account(&self) -> Option<Account> {

        match &self.data {
            Some(data) => Some(Account::new(data.to_string())),
            None => None,
        }
    }
}

