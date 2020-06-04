#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![feature(proc_macro_hygiene, decl_macro)]

pub mod cache;
pub mod file_map;
pub mod database;

pub(crate) use {cache::Cache, file_map::FileMap, database::Database};

pub(crate) mod core;
pub(crate) mod media_mixer;

#[macro_use] extern crate rocket;
pub mod microservice;

pub(crate) mod microservice_responders;