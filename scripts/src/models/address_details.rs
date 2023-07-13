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
pub struct AddressDetails {
    /// The address city
    #[serde(rename = "city", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    #[serde(rename = "country", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub country: Option<Option<crate::models::CountryAlpha2>>,
    /// The first line of the address
    #[serde(rename = "line1", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub line1: Option<Option<String>>,
    /// The second line of the address
    #[serde(rename = "line2", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub line2: Option<Option<String>>,
    /// The third line of the address
    #[serde(rename = "line3", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub line3: Option<Option<String>>,
    /// The zip/postal code for the address
    #[serde(rename = "zip", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub zip: Option<Option<String>>,
    /// The address state
    #[serde(rename = "state", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// The first name for the address
    #[serde(rename = "first_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Option<String>>,
    /// The last name for the address
    #[serde(rename = "last_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Option<String>>,
}

impl AddressDetails {
    pub fn new() -> AddressDetails {
        AddressDetails {
            city: None,
            country: None,
            line1: None,
            line2: None,
            line3: None,
            zip: None,
            state: None,
            first_name: None,
            last_name: None,
        }
    }
}


