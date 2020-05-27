use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
	RequestError(Box<dyn StdError>),
	ParseError(std::io::Error),
	CookieError(String),
	HtmlError(HtmlError),
}

impl From<HtmlError> for Error {
	fn from(value: HtmlError) -> Self {
		Self::HtmlError(value)
	}
}

#[derive(Debug)]
pub enum HtmlError {
	InvalidLineCount(String),
	AttributeNotFound(String, &'static str, u8),
	InvalidValue(String, &'static str, u8),
}