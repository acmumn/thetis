/// The database ID of a member, not to be confused with their student ID. (A database ID might be
/// `12`, a student ID will be e.g. `5114284`.)
#[derive(Clone, Copy, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MemberID(pub u32);
