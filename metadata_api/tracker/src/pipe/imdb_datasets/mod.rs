mod data;

pub mod source;
pub mod rows;

pub use data::*;

#[cfg(test)]
mod tests {
    use crate::pipe::Pipe;
    use futures::stream::StreamExt;
    use super::*;
    use std::convert::TryFrom;

    #[tokio::test(threaded_scheduler)]
    async fn test_local_files_correct_row_count() {
        let rows_pipe = rows::RowsPipe::new(source::LocalFilePipe::new("resources/test"));
        let mut stream = rows_pipe.get(()).await.unwrap();
        let mut counter: usize = 0;
        while let Some(rows) = stream.next().await {
            let rows = rows.unwrap();
            if let Err(e) = Vec::<Row>::try_from(&rows) {
                println!("{}", e);
            }
            // counter += Vec::<Row>::try_from(&rows).unwrap().len();
            // println!("{:?}", counter);
        }
        assert_eq!(counter, 90_938_739);
    }

    // #[tokio::test]
    // async fn print_byte_len() {
    //     let file_pipe = source::LocalFilePipe::new("resources/test");
    //     let mut stream = file_pipe.get(DatasetKind::TitleRatings).await.unwrap();
    //     while let Some(Ok(bytes)) = stream.next().await {
    //         println!("{:?}", bytes.len());
    //     }
    // }
}