use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub application_data: Option<String>,
    pub ephemeral_public_key: Option<String>,
    pub wrapped_key: Option<String>,
    pub public_key_hash: String,
    pub transaction_id: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PaymentToken {
    pub data: String,
    pub signature: String,
    pub header: Header,
    pub version: Version
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentData {
    online_payment_cryptogram: Option<String>,
    eci_indicator: Option<String>,
    emv_data: Option<String>,
    encrypted_pin_data: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentDataKeys {
    application_primary_account_number: String,
    application_expiration_date: String,
    currency_code: String,
    transactoin_amount: BigDecimal,
    cardholder_name: Option<String>,
    device_manufacturer_identified: String,
    payment_data_type: String,
    payment_data: PaymentData
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Version {
    #[serde(rename = "EC_v1")]
    Ecc,
    #[serde(rename = "RSA_v1")]
    Rsa
}

pub(super) struct MerchantCertData {
    key_pair: String,
    merchant_id: Vec<u8>
}