use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AuthParams {
    pub(crate) email: String,
    #[serde(rename = "apiKey")]
    pub(crate) api_key: String,
    pub(crate) url: String,
}
