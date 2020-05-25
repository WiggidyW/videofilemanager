mod model;
mod request;
mod error;

use chrono::{DateTime, offset::{Utc, FixedOffset}};
use std::{thread, error::Error as StdError, time::{SystemTime, Duration}};
use crossbeam::crossbeam_channel as cbeam;
use serde::de::DeserializeOwned;
use request::ImdbDataset as ImdbDatasetTrait;
use model::*;
use error::Error;

pub fn run<F1, F2, F3, F4, E1, E2, E3>(
	interval: Duration,
	write: F1,
	get_timestamp: F2,
	set_timestamp: F3,
	log_err: F4)
where
	F1: Fn(ImdbDataset) -> Result<(), E1>,
	F2: Fn() -> Result<i64, E2>,
	F3: Fn(i64) -> Result<(), E3>,
	F4: Fn(&Error) -> (),
	E1: StdError + Send + 'static,
	E2: StdError + Send + 'static,
	E3: StdError + Send + 'static,
{
	loop {
		match get_timestamp() {
			Err(e) => log_err(&Error::get_timestamp_error(e)),
			Ok(timestamp) =>
		{ // Continue if get_timestamp was successful
			let results = [
				header_timestamp::<TitleRatings>(),
				header_timestamp::<TitleEpisode>(),
				header_timestamp::<TitleCrew>(),
				header_timestamp::<TitleBasics>(),
				header_timestamp::<TitleAkas>(),
				header_timestamp::<NameBasics>(),
				header_timestamp::<TitlePrincipals>(),
			];
			if results.iter().all(|r|
				match r {
					Err(e) => { log_err(e); false },
					Ok(i) => i > &timestamp,
			})
		{ // Continue if all header timestamps are ok and greater than timestamp
			match refresh(&write) {
				Err(e) => log_err(&e),
				Ok(()) =>
		{ // Continue if rows were written successfully
			let timestamp = DateTime::<Utc>::from(SystemTime::now())
				.timestamp();
			match set_timestamp(timestamp) {
				Err(e) => log_err(&Error::set_timestamp_error(e)),
				Ok(()) => (),
		}}}}}}
		thread::sleep(interval);
	}
}

pub fn refresh<F, E>(f: F) -> Result<(), Error>
where
	F: Fn(ImdbDataset) -> Result<(), E>,
	E: StdError + Send + 'static,
{
	let (tx1, rx) = cbeam::bounded(128);
	let (tx2, tx3, tx4, tx5, tx6, tx7) = (
		tx1.clone(),
		tx1.clone(),
		tx1.clone(),
		tx1.clone(),
		tx1.clone(),
		tx1.clone(),
	);

	thread::spawn(move || {
		iter::<TitlePrincipals>(tx1);
	});
	thread::spawn(move || {
		iter::<TitleAkas>(tx2);
		iter::<TitleBasics>(tx3);
	});
	thread::spawn(move || {
		iter::<NameBasics>(tx4);
		iter::<TitleCrew>(tx5);
		iter::<TitleEpisode>(tx6);
		iter::<TitleRatings>(tx7);
	});

	let mut count = 0;
	loop {
		if count == 7 {
			break;
		}
		match rx.recv() {
			Ok(Some(Ok(row))) => match f(row) {
				Ok(()) => (),
				Err(e) => return Err(Error::write_error(e)),
			}
			Ok(None) => count += 1,
			Ok(Some(Err(e))) => return Err(e),
			Err(e) => return Err(Error::from(e)),
		}
	}
	Ok(())
}

fn iter<T>(tx: cbeam::Sender<Option<Result<ImdbDataset, Error>>>)
where
	T: ImdbDatasetTrait,
	T: DeserializeOwned,
	T: Send,
{
	match T::request() {
		Err(e) => tx.send(Some(Err(e)))
			.unwrap_or_else(|e|
				panic!("Thread Panic sending Err: {}", e)
			),
		Ok(res) => for row_res in res.into_iter() {
			tx.send(Some(row_res))
				.unwrap_or_else(|e| 
					panic!("Thread Panic sending Row Result: {}", e)
				);
		},
	}
	tx.send(None)
		.unwrap_or_else(|e|
			panic!("Thread Panic sending None: {}", e)
		);
}

fn header_timestamp<T>() -> Result<i64, Error>
where
	T: ImdbDatasetTrait,
{
	let client = reqwest::blocking::Client::new();
	let res = client
		.head(T::url())
		.send()?
		.error_for_status()?;
	let time_str = res
		.headers()
		.get("Last-Modified")
		.ok_or_else(|| Error::HeaderNotFoundError)?
		.to_str()?;
	let time = DateTime::<FixedOffset>::parse_from_rfc2822(time_str)?
		.timestamp();
	Ok(time)
}

#[cfg(test)]
mod test_mock_server;

#[cfg(test)]
mod test {
	use std::{thread, time, convert::Infallible};
	use crate::test_mock_server;

	#[test]
	fn parses_ok() {
		thread::spawn(move || {
			test_mock_server::run()
		});
		thread::sleep(time::Duration::from_secs(2));
		assert!(crate::refresh(|_| Result::<(), Infallible>::Ok(())).is_ok());
	}
}