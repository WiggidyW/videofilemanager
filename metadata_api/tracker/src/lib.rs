#![allow(dead_code)]
#![feature(type_alias_impl_trait)]
#![feature(const_generics)]

pub struct BoxError(Box<dyn std::error::Error + Send + Sync>);

pub mod pipe;
pub mod token;