/// The body of a 403 response.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AuthError {
    //
}
