mod data;

pub mod source;
pub mod rows;
pub mod db_pipe;

pub use data::*;
pub use Error as DataError;

#[cfg(test)]
mod tests {
    use crate::pipe::Pipe;
    use futures::stream::StreamExt;
    use super::*;
    use std::convert::TryFrom;

    // #[tokio::test(threaded_scheduler)]
    // async fn test_local_files_correct_row_count() {
    //     let rows_pipe = rows::RowsPipe::new(source::LocalFilePipe::new("resources/test"));
    //     let mut stream = rows_pipe.get(()).await.unwrap();
    //     let mut counter: usize = 0;
    //     while let Some(rows) = stream.next().await {
    //         let rows = rows.unwrap();
    //         let num_rows = Vec::<Row>::try_from(&rows).unwrap().len();
    //         counter += num_rows;
    //     }
    //     assert_eq!(counter, 90_938_739);
    // }

    #[tokio::test(threaded_scheduler)]
    async fn test_sqlite_pipe() {
        let sqlite_pipe = db_pipe::SqlitePipe::new(
            rows::RowsPipe::new(source::LocalFilePipe::new("resources/test")),
            sqlx::Pool::new("sqlite:resources/test/sqlite_test.db").await.unwrap(),
            reqwest::Client::new(),
        );
        sqlite_pipe.tables_down().await.unwrap();
        sqlite_pipe.tables_up().await.unwrap();
        let title_info = sqlite_pipe
            .get(crate::token::ImdbId::new(102975))
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();
        println!("{:?}", title_info);
    }

    #[test]
    fn test_json_string() {
        let vec = vec![1, 2, 3, 4, 5, 6];
        let string = serde_json::Value::from(vec).to_string();
        println!("{}", string);
    }
}