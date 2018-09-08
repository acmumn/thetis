use std::collections::HashSet;

/// The body of a POST to `/api/auth/check`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthCheckRequest {
    /// The authentication token to check.
    pub token: String,

    /// The capabilities to require.
    pub capabilities: HashSet<String>,
}
