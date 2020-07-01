use derive_more::Display;

#[derive(Debug, Display)]
pub struct GuessitDict(serde_json::Value);

impl std::convert::TryFrom<&str> for GuessitDict {
    type Error = serde_json::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            serde_json::from_str(value)?
        ))
    }
}