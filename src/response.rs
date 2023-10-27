use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MultiPaymentResonse {
    pub error_message: Option<String>,
    pub transfer_responses: Vec<PaymentResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentResponse {
    pub authorisation_required: bool,
    pub beneficiary_account_id: String,
    pub beneficiary_name: String,
    #[serde(deserialize_with = "from_custom_date")]
    pub payment_date: NaiveDate,
    pub payment_reference_number: String,
    pub status: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BeneficiaryCategory {
    pub category_id: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub default_category: bool,
    pub category_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MultiTransferResonse {
    pub error_message: Option<String>,
    pub transfer_responses: Vec<TransferResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TransferResponse {
    pub authorisation_required: bool,
    pub beneficiary_account_id: String,
    pub beneficiary_name: String,
    #[serde(deserialize_with = "from_custom_date")]
    pub payment_date: NaiveDate,
    pub payment_reference_number: String,
    pub status: String,
}

// TODO!: determine if the correct fields are optional
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beneficiary {
    pub beneficiary_id: String,
    pub account_number: String,
    pub code: String,
    pub bank: String,
    pub beneficiary_name: Option<String>,
    #[serde(deserialize_with = "from_custom_amount")]
    pub last_payment_amount: Option<f32>,
    #[serde(deserialize_with = "from_custom_optional_date")]
    pub last_payment_date: Option<NaiveDate>,
    pub cell_no: Option<String>,
    pub email_address: Option<String>,
    pub name: String,
    pub reference_account_number: String,
    pub reference_name: Option<String>,
    pub category_id: String,
    pub profile_id: String,
    pub faster_payment_allowed: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub profile_id: String,
    pub profile_name: String,
    pub default_profile: bool,
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub data: T,
    // TODO!: create struct
    pub links: Links,
    pub meta: Meta,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DtCt {
    Debit,
    Credit,
}

#[derive(Debug, Deserialize)]
pub struct Transactions {
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionStatus {
    Posted,
    // is this all
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum TransactionType {
    VASTransactions,
    ATMWithdrawals,
    CardPurchases,
    FeesAndInterest,
    Deposits,
    OnlineBankingPayments,
    DebitOrders,
    FasterPay,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub account_id: String,
    #[serde(rename = "type")]
    pub type_: DtCt,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub description: String,
    pub card_number: String,
    pub posted_order: i32,
    pub posting_date: NaiveDate,
    pub value_date: NaiveDate,
    pub action_date: NaiveDate,
    pub transaction_date: NaiveDate,
    pub amount: f32,
    pub running_balance: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub account_id: String,
    pub current_balance: f32,
    pub available_balance: f32,
    pub budget_balance: Option<f32>,
    pub straight_balance: Option<f32>,
    pub cash_balance: Option<f32>,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub total_pages: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: String,
    pub account_number: String,
    pub account_name: String,
    pub reference_name: String,
    pub product_name: String,
    pub kyc_compliant: bool,
    pub profile_id: String,
    pub profile_name: String,
}

// deserializers

fn from_custom_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    if let Some(s) = s {
        if let Ok(dt) = NaiveDate::parse_from_str(&s, "%d/%m/%Y") {
            Ok(Some(dt))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn from_custom_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let dt = NaiveDate::parse_from_str(&s, "%d/%m/%Y").map_err(serde::de::Error::custom)?;
    Ok(dt)
}

fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let b = s.parse().map_err(serde::de::Error::custom)?;
    Ok(b)
}

fn from_custom_amount<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    if let Some(s) = s {
        if let Ok(amount) = s.parse::<f32>() {
            Ok(Some(amount))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
