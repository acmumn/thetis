use std::collections::HashSet;

/// The body of a POST to `/api/auth/check`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthCheckRequest {
    /// The capabilities to require.
    pub capabilities: HashSet<String>,

    /// The authentication token to check.
    pub token: String,
}

/// The body of a POST to `/api/auth/login`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthLoginRequest {
    /// A URL as a string. Optional, defaults to `$BASE_URL`. The URL the magic link should
    /// redirect to after the user has logged in.
    pub redirect: Option<String>,

    /// The user's X.500.
    pub x500: String,
}
