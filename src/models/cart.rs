use std::sync::{LazyLock, RwLock};

use super::product::{JhProduct, Product};

//Command
pub struct AddCartLine {
    pub user_id: u32,
    pub product_id: u32,
    pub quantity: u32,
}

pub struct Cart {
    user_id: u32,
    total: u32,
    lines: Vec<CartLine>,
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
    product: Product,
    quantity: u32,
}

pub static FAKE_CART_DB: LazyLock<RwLock<CartDb>> = LazyLock::new(Default::default);

#[derive(Default)]
pub struct CartDb {
    carts: Vec<Cart>,
}

impl CartDb {
    pub fn find_by_user_id(&mut self, user_id: u32) -> &mut Cart {
        self.carts
            .iter_mut()
            .find(|c| c.user_id == user_id)
            .unwrap()
    }
}
