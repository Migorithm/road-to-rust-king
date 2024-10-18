use std::{
    fmt::Error,
    sync::{LazyLock, RwLock},
};

use super::{
    command::{AddCartLine, UpdateCartLine}, //? is this right?
    product::{JhProduct, Product},
};

#[derive(Clone, Debug)]
pub struct Cart {
    pub(crate) user_id: u32,
    pub(crate) total: u32,
    pub(crate) lines: Vec<CartLine>,
}

impl Cart {
    pub fn add_line(&mut self, command: AddCartLine) {
        let line = CartLine {
            product_id: command.product_id,
            quantity: command.quantity,
        };

        self.lines.push(line);
    }

    pub fn update_line(&mut self, command: UpdateCartLine) {
        let line = CartLine {
            product_id: command.product_id,
            quantity: command.quantity,
        };

        self.lines.push(line);
    }
}

#[derive(Clone, Debug)]
pub struct CartLine {
    pub(crate) product_id: u32,
    pub(crate) quantity: u32,
}

impl CartLine {
    pub fn new(product_id: u32, quantity: u32) -> Self {
        Self {
            product_id,
            quantity,
        }
    }

    pub fn update_quantity(&mut self, quantity: u32) {
        self.quantity = quantity;
    }
}
