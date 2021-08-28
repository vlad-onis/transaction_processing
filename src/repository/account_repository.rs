use crate::model;
use crate::utils::db_utils;

use mongodb::bson::doc;

pub struct AccountRepository {
    pub db_connection: db_utils::DatabaseAccess,
}

impl AccountRepository {
    pub fn new() -> Option<AccountRepository> {
        let database_access = db_utils::DatabaseAccess::new();

        if let Ok(db_access) = database_access {
            return Some(AccountRepository {
                db_connection: db_access,
            });
        }
        None
    }

    pub fn insert_account(&self, account: &model::account::Account) -> bool {
        let account_searched = doc! {
            "client_id": account.client_id
        };

        let account_exists = self.db_connection.collections[db_utils::ACCOUNT_COLLECTION]
            .find_one(account_searched, None)
            .unwrap()
            .is_some();

        if account_exists {
            return false;
        }

        let account_document = mongodb::bson::to_document(account).unwrap();
        self.db_connection.collections[db_utils::ACCOUNT_COLLECTION]
            .insert_one(account_document, None)
            .expect("Could not insert");

        true
    }
}
