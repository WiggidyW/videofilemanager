use derive_more::From;
use either::Either;

#[derive(Debug, From)]
pub enum Error {
	RequestError(reqwest::Error),
	DeserializeError(csv::Error),
	ThreadError(crossbeam::crossbeam_channel::RecvError),
	#[from(ignore)]
	WriteError(Box<dyn std::error::Error + Send + 'static>),
	#[from(ignore)]
	GetTimestampError(Box<dyn std::error::Error + Send>),
	#[from(ignore)]
	SetTimestampError(Box<dyn std::error::Error + Send>),
	HeaderNotFoundError,
	InvalidHeaderError(Either<reqwest::header::ToStrError, chrono::format::ParseError>),
}

impl From<reqwest::header::ToStrError> for Error {
	fn from(value: reqwest::header::ToStrError) -> Self {
		Self::InvalidHeaderError(Either::Left(value))
	}
}

impl From<chrono::format::ParseError> for Error {
	fn from(value: chrono::format::ParseError) -> Self {
		Self::InvalidHeaderError(Either::Right(value))
	}
}

impl Error {
	pub fn write_error(err: impl std::error::Error + Send + 'static) -> Self {
		Self::WriteError(Box::new(err))
	}

	pub fn get_timestamp_error(err: impl std::error::Error + Send + 'static) -> Self {
		Self::GetTimestampError(Box::new(err))
	}

	pub fn set_timestamp_error(err: impl std::error::Error + Send + 'static) -> Self {
		Self::SetTimestampError(Box::new(err))
	}
}