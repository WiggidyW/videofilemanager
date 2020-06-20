mod data;

pub mod source;
pub mod rows;
pub mod db_pipe;

pub use data::*;

#[cfg(test)]
mod tests {
    use crate::pipe::Pipe;
    use futures::stream::StreamExt;
    use super::*;
    use std::convert::TryFrom;
    use std::iter::FromIterator;
    use std::collections::{HashSet, HashMap};

    #[tokio::test(threaded_scheduler)]
    async fn test_local_files_correct_row_count() {
        let rows_pipe = rows::RowsPipe::new(source::LocalFilePipe::new("resources/test"));
        let mut stream = rows_pipe.get(()).await.unwrap();
        let mut counter: usize = 0;
        while let Some(rows) = stream.next().await {
            let rows = rows.unwrap();
            let num_rows = Vec::<Row>::try_from(&rows).unwrap().len();
            counter += num_rows;
        }
        assert_eq!(counter, 90_938_739);
    }

    #[tokio::test(threaded_scheduler)]
    async fn check_duplicates() {
        let rows_pipe = rows::RowsPipe::new(source::LocalFilePipe::new("resources/test"));
        let mut stream = rows_pipe.get(()).await.unwrap();
        let mut id_tracker: Vec<HashSet<u32>> = vec![
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        ];
        let mut dupe_counter: Vec<u32> = vec![
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        while let Some(rows) = stream.next().await {
            let rows = rows.unwrap();
            for row in Vec::<Row>::try_from(&rows).unwrap() {
                match row {
                    Row::TitlePrincipals {imdb_id, ordering, name_id, category, job, characters} =>
                    match id_tracker[0].contains(&imdb_id) {
                        true => dupe_counter[0] += 1,
                        false => {let _ = id_tracker[0].insert(imdb_id);},
                    },
                    Row::TitleAkas {imdb_id, ordering, title, region, language, types, attributes, is_original_title} =>
                    match id_tracker[1].contains(&imdb_id) {
                        true => dupe_counter[1] += 1,
                        false => {let _ = id_tracker[1].insert(imdb_id);},
                    },
                    Row::TitleBasics {imdb_id, title_type, primary_title, original_title, is_adult, start_year, end_year, runtime_minutes, genres} => 
                    match id_tracker[2].contains(&imdb_id) {
                        true => dupe_counter[2] += 1,
                        false => {let _ = id_tracker[2].insert(imdb_id);},
                    },
                    Row::TitleCrew {imdb_id, directors, writers} =>
                    match id_tracker[3].contains(&imdb_id) {
                        true => dupe_counter[3] += 1,
                        false => {let _ = id_tracker[3].insert(imdb_id);},
                    },
                    Row::TitleEpisode {imdb_id, series_id, season_number, episode_number} =>
                    match id_tracker[4].contains(&imdb_id) {
                        true => dupe_counter[4] += 1,
                        false => {let _ = id_tracker[4].insert(imdb_id);},
                    },
                    Row::TitleRatings {imdb_id, average_rating, num_votes} =>
                    match id_tracker[5].contains(&imdb_id) {
                        true => dupe_counter[5] += 1,
                        false => {let _ = id_tracker[5].insert(imdb_id);},
                    },
                    _ => (),
                }
            }
        }
        println!("---\nTitlePrincipals - {} Dupes\nTitleAkas - {} Dupes\nTitleBasics - {} Dupes\nTitleCrew - {} Dupes\nTitleEpisode - {} Dupes\nTitleRatings - {} Dupes\n---", dupe_counter[0], dupe_counter[1], dupe_counter[2], dupe_counter[3], dupe_counter[4], dupe_counter[5]);
    }
}