mod data;
pub use data::*;

pub mod db_pipe;
pub mod chunk_pipe;
pub mod chunk_row_pipe;

#[cfg(test)]
mod tests {
    use crate::Pipe;
    use futures::stream::StreamExt;
    use super::*;
    use std::convert::TryFrom;
    use std::sync::Arc;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test(threaded_scheduler)]
    async fn test_local_files_correct_row_count() {
        let rows_pipe = Arc::new(rows::ChunkRowPipe::new(Arc::new(
            chunk::LocalFilePipe::new( {
                let mut file_map = HashMap::new();                
                file_map.insert(DatasetKind::TitlePrincipals, "resources/test/datasets/title.principals.tsv.gz");
                file_map.insert(DatasetKind::NameBasics, "resources/test/datasets/name.basics.tsv.gz");
                file_map.insert(DatasetKind::TitleAkas, "resources/test/datasets/title.akas.tsv.gz");
                file_map.insert(DatasetKind::TitleBasics, "resources/test/datasets/title.basics.tsv.gz");
                file_map.insert(DatasetKind::TitleCrew, "resources/test/datasets/title.crew.tsv.gz");
                file_map.insert(DatasetKind::TitleEpisode, "resources/test/datasets/title.episode.tsv.gz");
                file_map.insert(DatasetKind::TitleRatings, "resources/test/datasets/title.ratings.tsv.gz");
                file_map
            })
        )));
        let counter = Arc::new(AtomicU32::new(0));
        let mut vec = Vec::with_capacity(7);
        for kind in DatasetKind::iter() {
            let rows_pipe = rows_pipe.clone();
            let counter = counter.clone();
            vec.push(tokio::spawn(async move {
                let mut stream = rows_pipe.get(kind).await.unwrap();
                while let Some(_) = stream.next().await {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }
        for fut in vec {
            fut.await.unwrap();
        }
        assert_eq!(counter.load(Ordering::Relaxed), 90_938_739);
    }
}
//     #[tokio::test(threaded_scheduler)]
//     async fn test_sqlite_pipe() {
//         let sqlite_pipe = db_pipe::SqlitePipe::new(
//             rows::RowsPipe::new(source::LocalFilePipe::new("resources/test")),
//             sqlx::Pool::new("sqlite:resources/test/sqlite_test.db").await.unwrap(),
//             reqwest::Client::new(),
//         );
//         sqlite_pipe.tables_down().await.unwrap();
//         sqlite_pipe.tables_up().await.unwrap();
//         let title_info = sqlite_pipe
//             .get(crate::token::ImdbId::new(102975))
//             .await
//             .unwrap()
//             .next()
//             .await
//             .unwrap()
//             .unwrap();
//         println!("{:?}", title_info);
//     }

//     #[tokio::test]
//     async fn test_asdf() {
//         let pool: sqlx::SqlitePool = sqlx::Pool::new("sqlite:resources/test/sqlite_test.db").await.unwrap();
//         println!("{}, {}", pool.min_size(), pool.max_size());
//     }

//     #[test]
//     fn test_json_string() {
//         let vec = vec![1, 2, 3, 4, 5, 6];
//         let string = serde_json::Value::from(vec).to_string();
//         println!("{}", string);
//     }
// }