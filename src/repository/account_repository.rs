use crate::model;
use crate::utils::db_utils;

use mongodb::bson::doc;

pub struct AccountRepository {
    pub db_connection: db_utils::DatabaseAccess,
}

impl AccountRepository {
    /// Returns a AccountRepository if the database connection can be established, None otherwise
    pub fn new() -> Option<AccountRepository> {
        let database_access = db_utils::DatabaseAccess::new();

        if let Ok(db_access) = database_access {
            db_access.collections[db_utils::ACCOUNT_COLLECTION]
                .drop(None)
                .expect("Could not drop account collection");
            return Some(AccountRepository {
                db_connection: db_access,
            });
        }
        None
    }

    /// Inserts the given account in the Account collection of the database.
    /// Returns true on success, false otherwise.
    /// # Arguments
    ///
    /// * account - an Account to be inserted into the db
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
            .expect("Could not insert account");

        true
    }

    /// Returns a list of all accounts present in the database.
    pub fn find_all_accounts(&self) -> Vec<model::account::Account> {
        let cursor =
            match self.db_connection.collections[db_utils::ACCOUNT_COLLECTION].find(None, None) {
                Ok(cursor) => cursor,
                Err(_) => {
                    return vec![];
                }
            };
        let mut accounts: Vec<model::account::Account> = Vec::new();

        for account_document in cursor {
            let account =
                mongodb::bson::from_document::<model::account::Account>(account_document.unwrap());

            // TODO: clippy improvement
            if account.is_ok() {
                accounts.push(account.unwrap());
            }
        }

        accounts
    }

    /// Searches for an account by it's client id.
    /// Returns Option<Account>.
    /// # Arguments
    /// * client_id - i32
    pub fn find_account_by_client_id(&self, client_id: i32) -> Option<model::account::Account> {
        let account_searched = doc! {
            "client_id": client_id
        };

        let account_result = self.db_connection.collections[db_utils::ACCOUNT_COLLECTION]
            .find_one(account_searched, None);

        // TODO: Clippy improvement
        if account_result.is_ok() {
            let account_document = account_result.unwrap();

            // TODO: Clippy improvement
            if account_document.is_none() {
                return None;
            }

            let account_document = account_document.unwrap();
            let account = mongodb::bson::from_document::<model::account::Account>(account_document);

            // TODO: clippy improvement
            if account.is_ok() {
                return Some(account.unwrap());
            }
        }

        None
    }

    /// Updates the account represented by an ID with a new given value
    /// Returns true on success, false otherwise.
    /// # Arguments
    /// * old_account_client_id - i32 representing the account to be updated
    /// * new_account - Account holding the new values for update.
    pub fn update_account(
        &self,
        old_account_client_id: i32,
        new_account: &model::account::Account,
    ) -> bool {
        let old_account_document = doc! {
            "client_id": old_account_client_id
        };
        let new_account_document = mongodb::bson::to_document(new_account);

        let result = self.db_connection.collections[db_utils::ACCOUNT_COLLECTION]
            .find_one_and_replace(old_account_document, new_account_document.unwrap(), None)
            .expect("Could not update account");

        result.is_some()
    }
}
