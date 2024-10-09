use std::{
    fmt::Error,
    sync::{LazyLock, RwLock},
};

use super::{
    command::{AddCartLine, UpdateCartLine}, //? is this right?
    product::{JhProduct, Product},
};

// Command

pub struct Cart {
    pub(crate) user_id: u32,
    pub(crate) total: u32,
    pub(crate) lines: Vec<CartLine>,
}

impl Cart {
    pub fn add_line(&mut self, command: AddCartLine) {
        let line = CartLine {
            productId: command.product_id,
            quantity: command.quantity,
        };

        self.lines.push(line);
    }

    pub fn update_line(&mut self, command: UpdateCartLine) {
        let line = CartLine {
            productId: command.product_id,
            quantity: command.quantity,
        };

        self.lines.push(line);
    }
}

pub struct CartLine {
    pub(crate) productId: u32,
    pub(crate) quantity: u32,
}
