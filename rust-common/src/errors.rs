use std::collections::HashSet;

/// The body of a 403 response.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum AuthError {
    /// The auth token was required but not present.
    AuthTokenRequired,

    /// The auth token was insufficient to grant the needed privileges.
    #[serde(with = "capabilities_in_struct")]
    CapabilitiesRequired(HashSet<String>),

    /// The auth token was expired.
    Expired,

    /// The auth token was invalid.
    Invalid,
}

mod capabilities_in_struct {
    use std::collections::HashSet;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D>(de: D) -> Result<HashSet<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct CapsDe {
            capabilities: HashSet<String>,
        }

        CapsDe::deserialize(de).map(|caps| caps.capabilities)
    }

    pub fn serialize<S>(capabilities: &HashSet<String>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        pub struct CapsSer<'a> {
            capabilities: &'a HashSet<String>,
        }

        let caps = CapsSer { capabilities };
        CapsSer::serialize(&caps, ser)
    }
}
