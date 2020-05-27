pub enum Error {
	RedirectError(u16),
	ClientError(u16),
	ServerError(u16),
	ParseError(std::io::Error),
	CookieError,
}