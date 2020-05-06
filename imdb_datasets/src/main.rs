use std::ops::Deref;

fn main() {
	let mut biggest_ep: u32 = 0;
	let mut biggest_se: u32 = 0;
	let t = imdb_datasets::TitleEpisode::new().unwrap();
	for entry in t.data().deref() {
		if let Some(i) = entry.episode {
			if i > biggest_ep {
				biggest_ep = i;
			}
		}
		if let Some(i) = entry.season {
			if i > biggest_se {
				biggest_se = i;
			}
		}
	}
	println!("Biggest Episode: {}\nBiggest Season: {}", biggest_ep, biggest_se);
}