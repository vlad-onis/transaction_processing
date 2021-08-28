use serde::Deserialize;
use serde::Serialize;
use std::cmp::PartialEq;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
    Default,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client_id: i32,
    pub transaction_id: i32,
    pub amount: Option<f32>,
    pub disputed: bool,
}

pub const TRANSACTION_FIELDS: usize = 4;
