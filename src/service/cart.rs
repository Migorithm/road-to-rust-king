use crate::db;
use crate::models::Cart;
use std::convert::From;
use std::fmt::Error;

#[derive(Debug)]
pub enum CartError {
    DbError,
    NotFound,
}

impl From<std::fmt::Error> for CartError {
    fn from(_: std::fmt::Error) -> Self {
        CartError::DbError
    }
}

pub struct CartController;

// repository

// #1 Code First Approach

// #2 DB First Approach

// TODO: repository가 정의된 부분과 구현된 부분이 어디에 있어야할지? think or research

impl CartController {
    // GET
    pub fn get_cart(user_id: u32) -> Result<Cart, CartError> {
        CartRepository::find(user_id, product_id, quantity)
    }

    // POST
    pub fn add_to_cart(user_id: u32, product_id: u32, quantity: u32) -> Result<(), CartError> {
        CartRepository::insert(user_id, product_id, quantity)
    }

    // DELETE
    pub fn remove_from_cart(user_id: u32, product_id: u32) -> Result<(), CartError> {
        CartRepository::delete(user_id, product_id)
    }

    pub fn update_cart_line(user_id: u32, product_id: u32, quantity: u32) -> Result<(), CartError> {
        CartRepository::update(user_id, product_id, quantity)
    }
}
