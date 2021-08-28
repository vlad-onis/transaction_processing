#[cfg(test)]
pub mod repository_tests {

    use crate::model;
    use crate::repository::account_repository;
    use crate::utils;

    use mongodb::bson::doc;

    #[test]
    pub fn test_duplicates() -> Result<(), mongodb::error::Error> {
        let account_repository =
            account_repository::AccountRepository::new().ok_or("repository creation failed");

        let mock_account = model::account::Account {
            client_id: 1000,
            available: 0.0,
            total: 0.0,
            held: 0.0,
            locked: false,
        };

        let mock_account_duplicate = model::account::Account {
            client_id: 1000,
            available: 0.0,
            total: 0.0,
            held: 0.0,
            locked: false,
        };

        let account_repository = account_repository.unwrap();
        account_repository.insert_account(&mock_account);
        account_repository.insert_account(&mock_account_duplicate);

        let test_doc = doc! {
            "client_id" : 1000
        };

        let cnt = account_repository.db_connection.collections[utils::db_utils::ACCOUNT_COLLECTION]
            .count_documents(test_doc, None)
            .unwrap();

        assert_eq!(cnt, 1);

        Ok(())
    }
}
