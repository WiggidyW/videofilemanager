use tokio::runtime::Runtime;
use warp::Filter;

// #[get("/<name>")]
// fn download(file: PathBuf) -> rocket::Result<NamedFile, NotFound<String>> {
// 	let path = Path::new("../../resources/test/").join(file);
// 	NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
// }

pub fn run() {
	let mut rt = Runtime::new()
		.unwrap();
	rt.block_on(async {
		let download = warp::path("imdb-datasets")
			.and(warp::fs::dir("../resources/test"));
		warp::serve(download).run(([127, 0, 0, 1], 12794)).await;
	})
}