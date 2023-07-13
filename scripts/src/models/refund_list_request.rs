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
pub struct RefundListRequest {
    /// The identifier for the payment
    #[serde(rename = "payment_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<Option<String>>,
    /// Limit on the number of objects to return
    #[serde(rename = "limit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub limit: Option<Option<i64>>,
    /// The starting point within a list of objects
    #[serde(rename = "offset", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub offset: Option<Option<i64>>,
    #[serde(rename = "time_range", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub time_range: Option<Option<Box<crate::models::TimeRange>>>,
    /// The list of connectors to filter refunds list
    #[serde(rename = "connector", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub connector: Option<Option<Vec<String>>>,
    /// The list of currencies to filter refunds list
    #[serde(rename = "currency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Option<Vec<crate::models::Currency>>>,
    /// The list of refund statuses to filter refunds list
    #[serde(rename = "refund_status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<Option<Vec<crate::models::RefundStatus>>>,
}

impl RefundListRequest {
    pub fn new() -> RefundListRequest {
        RefundListRequest {
            payment_id: None,
            limit: None,
            offset: None,
            time_range: None,
            connector: None,
            currency: None,
            refund_status: None,
        }
    }
}


