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
pub struct MandateResponse {
    /// The identifier for mandate
    #[serde(rename = "mandate_id")]
    pub mandate_id: String,
    #[serde(rename = "status")]
    pub status: crate::models::MandateStatus,
    /// The identifier for payment method
    #[serde(rename = "payment_method_id")]
    pub payment_method_id: String,
    /// The payment method
    #[serde(rename = "payment_method")]
    pub payment_method: String,
    #[serde(rename = "card", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub card: Option<Option<Box<crate::models::MandateCardDetails>>>,
    #[serde(rename = "customer_acceptance", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub customer_acceptance: Option<Option<Box<crate::models::CustomerAcceptance>>>,
}

impl MandateResponse {
    pub fn new(mandate_id: String, status: crate::models::MandateStatus, payment_method_id: String, payment_method: String) -> MandateResponse {
        MandateResponse {
            mandate_id,
            status,
            payment_method_id,
            payment_method,
            card: None,
            customer_acceptance: None,
        }
    }
}


