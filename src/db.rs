use lazy_static::lazy_static;

use std::{
    collections::HashMap,
    fmt::Error,
    sync::{LazyLock, Mutex, RwLock},
};

use crate::models::cart::{Cart, CartLine};

// 가라로 DB를 위한 전역 변수
lazy_static! {
    static ref CART_DB: Mutex<HashMap<u32, Cart>> = Mutex::new(HashMap::new());
}

pub fn find_cart_by_user_id(user_id: u32) -> Option<Cart> {
    // CART_DB의 Mutex를 잠금
    let db = CART_DB.lock().unwrap();

    // user_id로 Cart를 검색하고, 없으면 None을 반환
    db.get(&user_id).cloned()
}

pub fn save_cart(cart: Cart) -> Result<(), Error> {
    // CART_DB의 Mutex를 잠금
    let mut db = CART_DB.lock().unwrap();

    // user_id를 키로 하여 Cart를 삽입
    db.insert(cart.user_id, cart);

    Ok(())
}

// test: 같은 상품에 대한 cartline이 추가되었을 때, 수량만 올라가야 한다
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::command::AddCartLine;

    #[test]
    fn test_add_same_product_increases_quantity() {
        let mut cart = Cart {
            user_id: 1,
            total: 0,
            lines: vec![],
        };

        let command1 = AddCartLine {
            user_id: 1,
            product_id: 101,
            quantity: 2,
        };

        let command2 = AddCartLine {
            user_id: 1,
            product_id: 101,
            quantity: 3,
        };

        cart.add_line(command1);
        cart.add_line(command2);

        assert_eq!(cart.lines.len(), 1);
        assert_eq!(cart.lines[0].quantity, 5);
    }
}

// advanced: order가 만들어지면, cart를 비운다
pub fn clear_cart(user_id: u32) -> Result<(), Error> {
    let mut db = CART_DB.lock().unwrap();

    if let Some(cart) = db.get_mut(&user_id) {
        cart.lines.clear();
    }

    Ok(())
}
// -> 근데 만약에 pay를 하고 cancel이 되면???
