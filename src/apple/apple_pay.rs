use crate::common::detokenizer::Detokenizer;
use super::domain::{MerchantCertData, PaymentToken};
use anyhow::Result;

pub struct ApplePay;

impl ApplePay {
    fn parse_token(&self, source: String) -> Result<PaymentToken> {
        let json_buff = base64::decode(source)?;
        let json_raw = String::from_utf8(json_buff)?;
        let token: PaymentToken = serde_json::from_str(&json_raw)?;
        Ok(token)
    }

    fn get_token_merchant_cert_data(token: PaymentToken) -> Result<MerchantCertData> {
        todo!()
    }
}

impl Detokenizer for ApplePay {
    type TokenData = ();

    fn extract(&self, data: String) -> Result<Self::TokenData> {
        let token = self.parse_token(data)?;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::apple::domain::{Header, Version};

    use super::*;

    #[test]
    fn successful_token_json_parse() {
        let token_ecc_raw = r#"
        {
            "data":"test_data",
            "signature":"test_sign",
            "version":"EC_v1",
            "header":{
                "applicationData":"test_data",
                "ephemeralPublicKey":"test_epk",
                "wrappedKey":"test_wk",
                "publicKeyHash":"test_pkh",
                "transactionId":"test_tid"
            }
        }"#.to_string();
        let token_rsa_raw = r#"
        {
            "data":"test_data",
            "signature":"test_sign",
            "version":"RSA_v1",
            "header":{
                "applicationData":"test_data",
                "ephemeralPublicKey":"test_epk",
                "wrappedKey":"test_wk",
                "publicKeyHash":"test_pkh",
                "transactionId":"test_tid"
            }
        }"#.to_string();
        let expected_ecc_token = PaymentToken {
            data: "test_data".to_string(),
            signature: "test_sign".to_string(),
            version: Version::Ecc,
            header: Header {
                application_data: Some("test_data".to_string()),
                ephemeral_public_key: Some("test_epk".to_string()),
                wrapped_key: Some("test_wk".to_string()),
                public_key_hash: "test_pkh".to_string(),
                transaction_id: "test_tid".to_string(),
            }
        };
        let expected_rsa_token = PaymentToken {
            data: "test_data".to_string(),
            signature: "test_sign".to_string(),
            version: Version::Rsa,
            header: Header {
                application_data: Some("test_data".to_string()),
                ephemeral_public_key: Some("test_epk".to_string()),
                wrapped_key: Some("test_wk".to_string()),
                public_key_hash: "test_pkh".to_string(),
                transaction_id: "test_tid".to_string(),
            }
        };
        let applepay_detokenizer = ApplePay { };
        match applepay_detokenizer.parse_token(base64::encode(token_ecc_raw)) {
            Ok(token) => assert_eq!(token, expected_ecc_token),
            Err(err) => assert!(false, "failed to parse token: {:?}", err),
        };
        match applepay_detokenizer.parse_token(base64::encode(token_rsa_raw)) {
            Ok(token) => assert_eq!(token, expected_rsa_token),
            Err(err) => assert!(false, "failed to parse token: {:?}", err),
        }
    }

    #[test]
    fn error_on_invalid_token() {
        let invalid_token = r#"
        {
            "data":"test_data",
            "signature":"test_sign",
            "version":"NoneExist",
            "header":{
                "applicationData":"test_data",
                "ephemeralPublicKey":"test_epk",
                "wrappedKey":"test_wk",
                "publicKeyHash":"test_pkh",
                "transactionId":"test_tid"
            }
        }"#.to_string();
        let applepay_detokenizer = ApplePay { };
        match applepay_detokenizer.parse_token(base64::encode(invalid_token)) {
            Ok(_) => assert!(false, "parse should return error on invalid token"),
            Err(err) => assert!(true),
        };
    }
}