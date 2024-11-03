use std::sync::{Arc, Mutex};

use db::db::CartDb;

// Donut store
// 내가 뭘 팔것인가? -> Product
// - Cheese Donut
// - Choco Donut
// - Glazed Donut
// Transaction -> Payment
// 판매가 된 것들은 어떻게 관리할것인가?  -> Service
#[macro_use]
extern crate lazy_static;

pub mod db;
mod error;
pub mod model;
pub mod repository;
pub mod service;

lazy_static! {
    static ref DB: Arc<Mutex<CartDb>> = Arc::new(Mutex::new(CartDb::new()));
}

fn main() {
    todo!()
}
