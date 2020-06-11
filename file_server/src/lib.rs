#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]

pub(crate) mod core;

#[macro_use] extern crate rocket;
pub mod microservice;