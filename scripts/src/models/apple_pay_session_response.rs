/*
 * Hyperswitch - API Documentation
 *
 *  ## Get started  Hyperswitch provides a collection of APIs that enable you to process and manage payments. Our APIs accept and return JSON in the HTTP body, and return standard HTTP response codes.  You can consume the APIs directly using your favorite HTTP/REST library.  We have a testing environment referred to \"sandbox\", which you can setup to test API calls without affecting production data. Currently, our sandbox environment is live while our production environment is under development and will be available soon. You can sign up on our Dashboard to get API keys to access Hyperswitch API.  ### Environment  Use the following base URLs when making requests to the APIs:  | Environment   |  Base URL                          | |---------------|------------------------------------| | Sandbox       | <https://sandbox.hyperswitch.io>   | | Production    | <https://api.hyperswitch.io>       |  ## Authentication  When you sign up on our [dashboard](https://app.hyperswitch.io) and create a merchant account, you are given a secret key (also referred as api-key) and a publishable key. You may authenticate all API requests with Hyperswitch server by providing the appropriate key in the request Authorization header.  | Key             |  Description                                                                                  | |-----------------|-----------------------------------------------------------------------------------------------| | api-key         | Private key. Used to authenticate all API requests from your merchant server                  | | publishable key | Unique identifier for your account. Used to authenticate API requests from your app's client  |  Never share your secret api keys. Keep them guarded and secure. 
 *
 * The version of the OpenAPI document: 0.2.0
 * Contact: hyperswitch@juspay.in
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplePaySessionResponse {
    #[serde(rename = "secrets")]
    pub secrets: Box<crate::models::SecretInfoToInitiateSdk>,
    /// Timestamp at which session is requested
    #[serde(rename = "epoch_timestamp")]
    pub epoch_timestamp: i64,
    /// Timestamp at which session expires
    #[serde(rename = "expires_at")]
    pub expires_at: i64,
    /// The identifier for the merchant session
    #[serde(rename = "merchant_session_identifier")]
    pub merchant_session_identifier: String,
    /// Apple pay generated unique ID (UUID) value
    #[serde(rename = "nonce")]
    pub nonce: String,
    /// The identifier for the merchant
    #[serde(rename = "merchant_identifier")]
    pub merchant_identifier: String,
    /// The domain name of the merchant which is registered in Apple Pay
    #[serde(rename = "domain_name")]
    pub domain_name: String,
    /// The name to be displayed on Apple Pay button
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A string which represents the properties of a payment
    #[serde(rename = "signature")]
    pub signature: String,
    /// The identifier for the operational analytics
    #[serde(rename = "operational_analytics_identifier")]
    pub operational_analytics_identifier: String,
    /// The number of retries to get the session response
    #[serde(rename = "retries")]
    pub retries: i32,
    /// The identifier for the connector transaction
    #[serde(rename = "psp_id")]
    pub psp_id: String,
}

impl ApplePaySessionResponse {
    pub fn new(secrets: crate::models::SecretInfoToInitiateSdk, epoch_timestamp: i64, expires_at: i64, merchant_session_identifier: String, nonce: String, merchant_identifier: String, domain_name: String, display_name: String, signature: String, operational_analytics_identifier: String, retries: i32, psp_id: String) -> ApplePaySessionResponse {
        ApplePaySessionResponse {
            secrets: Box::new(secrets),
            epoch_timestamp,
            expires_at,
            merchant_session_identifier,
            nonce,
            merchant_identifier,
            domain_name,
            display_name,
            signature,
            operational_analytics_identifier,
            retries,
            psp_id,
        }
    }
}


