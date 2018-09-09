/// The database ID of mailing list.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MailingListID(pub u32);

/// The database ID of a member, not to be confused with their student ID. (A database ID might be
/// `12`, a student ID will be e.g. `5114284`.)
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MemberID(pub u32);

/// A tag applied to a member.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tag(pub String);

/// The database ID of mail template.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TemplateID(pub u32);
