pub(crate) const ERR_REQWEST_TO_IO: std::io::ErrorKind = std::io::ErrorKind::TimedOut;

pub enum Error {
	RequestError(reqwest::Error),
	DecoderError(Box<dyn std::error::Error>),
	GzipError(std::io::Error),
	DeserializeError(Box<dyn std::error::Error>),
}

impl Error {
	pub(crate) fn stream_error(error: std::io::Error) -> Self {
		match error.kind() {
			ERR_REQWEST_TO_IO => Self::DecoderError(error.into_inner().unwrap()),
			_ => Self::GzipError(error),
		}
	}

	pub(crate) fn deser_error(error: impl std::error::Error + 'static) -> Self {
		Self::DeserializeError(Box::new(error))
	}
}