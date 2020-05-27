#![allow(dead_code)]
#![allow(unused_variables)]

const CSV_DELIMITER: u8 = b'\t';
const CSV_TERMINATOR: u8 = b'\n';

mod core;

pub(crate) mod deser;

pub(crate) mod model;

pub mod error;

// #[cfg(test)]
// mod tests {
// 	use futures_core::stream::Stream;
// 	use futures_util::stream::StreamExt;
// 	use bytes::Bytes;
// 	use std::io::Write;

// 	#[tokio::test]
// 	async fn my_test() {
// 		let res = crate::Dataset::TitleEpisode
// 			.request()
// 			.await
// 			.unwrap();
// 		let mut file = std::fs::File::create("test.csv")
// 			.unwrap();
// 		let mut stream = res
// 			.into_raw_stream()
// 			.into_decoded_stream();
// 		while let Some(byte) = stream.next().await {
// 			println!("{}", byte.unwrap().len());
// 			// file.write(&byte.unwrap())
// 			// 	.unwrap();
// 			// file.write(b"what the fuck?!")
// 			// 	.unwrap();
// 		}
// 	}
// }