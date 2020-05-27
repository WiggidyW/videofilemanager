#[derive(Debug)]
pub enum Error {
	RequestError(reqwest::Error),
	ReqwestDecoderError(Box<dyn std::error::Error>),
	GzipDecoderError(std::io::Error),
	Utf8Error(std::string::FromUtf8Error),
	DeserializeError(Box<dyn std::error::Error>),
	SinkError(Box<dyn std::error::Error>),
}

impl Error {
	pub(crate) fn request_error(error: reqwest::Error) -> Self {
		Self::RequestError(error)
	}

	pub(crate) fn decoder_error(error: std::io::Error) -> Self {
		match &error.kind() {
			std::io::ErrorKind::TimedOut => Self::ReqwestDecoderError(error.into_inner().unwrap()),
			_ => Self::GzipDecoderError(error),
		}
	}

	pub(crate) fn utf8_error(error: std::string::FromUtf8Error) -> Self {
		Self::Utf8Error(error)
	}

	pub(crate) fn deser_error(error: impl std::error::Error + 'static) -> Self {
		Self::DeserializeError(Box::new(error))
	}

	pub(crate) fn sink_error(error: impl std::error::Error + 'static) -> Self {
		Self::WriteError(Box::new(error))
	}

	pub(crate) fn reqwest_to_io(error: reqwest::Error) -> std::io::Error {
		std::io::Error::new(std::io::ErrorKind::TimedOut, error)
	}
}