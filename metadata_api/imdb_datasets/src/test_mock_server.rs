use tokio::runtime::Runtime;
use warp::Filter;

pub fn run() {
	let mut rt = Runtime::new()
		.unwrap();
	rt.block_on(async {
		let download = warp::path("imdb-datasets")
			.and(warp::fs::dir("../resources/test"));
		warp::serve(download).run(([127, 0, 0, 1], 12794)).await;
	})
}