#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![feature(proc_macro_hygiene, decl_macro)]

pub(crate) mod core;

#[macro_use] extern crate rocket;
pub mod microservice;