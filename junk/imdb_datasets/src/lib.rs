#![feature(const_generics)]
#![allow(incomplete_features)]

mod generic;
pub(crate) use generic::Request;
pub use generic::{Metadata, Error};

mod imdb_dataset;
pub type TitleEpisode = Metadata<imdb_dataset::TitleEpisode>;