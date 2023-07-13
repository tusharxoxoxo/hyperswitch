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
pub struct ApplepaySessionTokenResponse {
    #[serde(rename = "session_token_data")]
    pub session_token_data: Box<crate::models::ApplePaySessionResponse>,
    #[serde(rename = "payment_request_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_request_data: Option<Option<Box<crate::models::ApplePayPaymentRequest>>>,
    /// The session token is w.r.t this connector
    #[serde(rename = "connector")]
    pub connector: String,
    /// Identifier for the delayed session response
    #[serde(rename = "delayed_session_token")]
    pub delayed_session_token: bool,
    #[serde(rename = "sdk_next_action")]
    pub sdk_next_action: Box<crate::models::SdkNextAction>,
}

impl ApplepaySessionTokenResponse {
    pub fn new(session_token_data: crate::models::ApplePaySessionResponse, connector: String, delayed_session_token: bool, sdk_next_action: crate::models::SdkNextAction) -> ApplepaySessionTokenResponse {
        ApplepaySessionTokenResponse {
            session_token_data: Box::new(session_token_data),
            payment_request_data: None,
            connector,
            delayed_session_token,
            sdk_next_action: Box::new(sdk_next_action),
        }
    }
}


