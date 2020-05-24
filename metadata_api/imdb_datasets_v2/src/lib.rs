mod model;

use serde::de::DeserializeOwned;
use std::thread;
use crossbeam::crossbeam_channel::{Sender, Receiver, bounded, select, RecvError, never};
use either::Either;

type ThreadError = Either<reqwest::Error, csv::Error>;
type Error = Either<RecvError, ThreadError>;

fn spawn_iter<T>(tx: Sender<Option<Result<T, ThreadError>>>) -> thread::JoinHandle<()>
where
	T: model::ImdbDataset,
	T: DeserializeOwned,
	T: Send,
	T: 'static,
{
	thread::spawn(move || { match T::request() {
		Err(e) => {
			tx.send(Some(Err(ThreadError::Left(e))))
				.expect("Thread Panicked");
			tx.send(None)
				.expect("Thread Panicked");
		},
		Ok(res) => {
			for row_result in res.into_iter() {
				tx.send(Some(row_result
					.map_err(|e| ThreadError::Right(e))))
					.expect("Thread Panicked");
			}
			tx.send(None)
				.expect("Thread Panicked");
		},
	}})
}

pub fn run() -> Result<(), Error> {
	let (tx1, mut rx1) = bounded(1);
	let (tx2, mut rx2) = bounded(1);
	let (tx3, mut rx3) = bounded(1);
	let (tx4, mut rx4) = bounded(1);
	let (tx5, mut rx5) = bounded(1);
	let (tx6, mut rx6) = bounded(1);
	let (tx7, mut rx7) = bounded(1);

	let _ = spawn_iter::<model::TitlePrincipals>(tx1);
	let _ = spawn_iter::<model::NameBasics>(tx2);
	let _ = spawn_iter::<model::TitleAkas>(tx3);
	let _ = spawn_iter::<model::TitleBasics>(tx4);
	let _ = spawn_iter::<model::TitleCrew>(tx5);
	let _ = spawn_iter::<model::TitleEpisode>(tx6);
	let _ = spawn_iter::<model::TitleRatings>(tx7);

	let mut count = 0;
	loop {
		if count == 7 {
			return Ok(());
		}
		select! {
			recv(rx1) -> msg => match msg {
				Ok(Some(Ok(row))) => (),
				Ok(Some(Err(e))) => return Err(Error::Right(e)),
				Err(e) => return Err(Error::Left(e)),
				Ok(None) => {
					count += 1;
					rx1 = never();
				},
			},
			recv(rx2) -> msg => match msg {
				Ok(Some(Ok(row))) => (),
				Ok(Some(Err(e))) => return Err(Error::Right(e)),
				Err(e) => return Err(Error::Left(e)),
				Ok(None) => {
					count += 1;
					rx2 = never();
				},
			},
			recv(rx3) -> msg => match msg {
				Ok(Some(Ok(row))) => (),
				Ok(Some(Err(e))) => return Err(Error::Right(e)),
				Err(e) => return Err(Error::Left(e)),
				Ok(None) => {
					count += 1;
					rx3 = never();
				},
			},
			recv(rx4) -> msg => match msg {
				Ok(Some(Ok(row))) => (),
				Ok(Some(Err(e))) => return Err(Error::Right(e)),
				Err(e) => return Err(Error::Left(e)),
				Ok(None) => {
					count += 1;
					rx4 = never();
				},
			},
			recv(rx5) -> msg => match msg {
				Ok(Some(Ok(row))) => (),
				Ok(Some(Err(e))) => return Err(Error::Right(e)),
				Err(e) => return Err(Error::Left(e)),
				Ok(None) => {
					count += 1;
					rx5 = never();
				},
			},
			recv(rx6) -> msg => match msg {
				Ok(Some(Ok(row))) => (),
				Ok(Some(Err(e))) => return Err(Error::Right(e)),
				Err(e) => return Err(Error::Left(e)),
				Ok(None) => {
					count += 1;
					rx6 = never();
				},
			},
			recv(rx7) -> msg => match msg {
				Ok(Some(Ok(row))) => (),
				Ok(Some(Err(e))) => return Err(Error::Right(e)),
				Err(e) => return Err(Error::Left(e)),
				Ok(None) => {
					count += 1;
					rx7 = never();
				},
			},
		}
	}
}

#[cfg(test)]
mod test_mock_server;

#[cfg(test)]
mod test {
	use std::{thread, time};
	use crate::test_mock_server;

	#[test]
	fn test_ok() {
		thread::spawn(move || {
			test_mock_server::run()
		});
		thread::sleep(time::Duration::from_secs(2));
		crate::run().unwrap();
	}
}