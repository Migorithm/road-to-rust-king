// Donut store
// 내가 뭘 팔것인가? -> Product
// - Cheese Donut
// - Choco Donut
// - Glazed Donut
// Transaction -> Payment
// 판매가 된 것들은 어떻게 관리할것인가?  -> Service

use models::cart::{AddCartLine, FAKE_CART_DB};

mod models;

fn main() {
    // Given - user exists already...
    let account_id = 1;

    // choose donut and put in the cart - command
    let add_cart_line_command = AddCartLine {
        user_id: account_id,
        product_id: 1,
        quantity: 1,
    };

    let mut cart_db = FAKE_CART_DB.write().unwrap();

    let cart = cart_db.find_by_user_id(account_id);
    cart.add_line(add_cart_line_command);

    // Q. do we have to create a cart?
    // Q. how do we add cart line to the cart?

    // pay for donut

    // get donut
}
