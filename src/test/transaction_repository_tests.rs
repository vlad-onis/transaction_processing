#[cfg(test)]
pub mod repository_tests {

    use crate::model;
    use crate::repository::transaction_repository;
    use crate::utils;

    use mongodb::bson::doc;

    #[test]
    pub fn test_duplicates() -> Result<(), mongodb::error::Error> {
        let transaction_repository = transaction_repository::TransactionRepository::new()
            .ok_or("repository creation failed");

        let mock_transaction = model::transaction::Transaction {
            transaction_type: model::transaction::TransactionType::Default,
            client_id: 200,
            transaction_id: 1,
            amount: None,
        };

        let mock_transaction_duplicate = model::transaction::Transaction {
            transaction_type: model::transaction::TransactionType::Default,
            client_id: 200,
            transaction_id: 1,
            amount: None,
        };

        let transaction_repository = transaction_repository.unwrap();
        transaction_repository.insert_transaction(&mock_transaction);
        transaction_repository.insert_transaction(&mock_transaction_duplicate);

        let test_doc = doc! {
            "transaction_id" : 1
        };

        let cnt = transaction_repository.db_connection.collections
            [utils::db_utils::TRANSACTION_COLLECTION]
            .count_documents(test_doc, None)
            .unwrap();

        assert_eq!(cnt, 1);

        Ok(())
    }
}
