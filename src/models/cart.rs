use std::{
    fmt::Error,
    sync::{LazyLock, RwLock},
};

use super::{
    command::AddCartLine, //? is this right?
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
        let product = Product {
            id: command.product_id,
            kind: JhProduct::ChocoDonut,
            price: 1000,
        };

        let line = CartLine {
            product,
            quantity: command.quantity,
        };

        self.lines.push(line);
    }
}

pub struct CartLine {
    pub(crate) product: Product,
    pub(crate) quantity: u32,
}
