use std::{
    fmt::Error,
    sync::{LazyLock, RwLock},
};

use crate::models::cart::Cart;

pub static FAKE_CART_DB: LazyLock<RwLock<CartDb>> = LazyLock::new(Default::default);

#[derive(Default)]
pub struct CartDb {
    carts: Vec<Cart>,
}

impl CartDb {
    /// 유저의 장바구니를 찾는다
    pub fn find_by_user_id(&mut self, user_id: u32) -> &mut Cart {
        self.carts
            .iter_mut()
            .find(|c| c.user_id == user_id)
            .unwrap()
    }

    // TODO: DB Error 구현하기
    // TODO: Application 레벨에서 Error 구현하기
    // TODO: DB단에 Business Logic 넣지 말기 (진짜 DB Operation에서 예상되는 것만!! -- Model이랑 분리!!)

    pub fn insert(&mut self, user_id: u32) -> Result<(), Error> {
        let cart = Cart {
            user_id,
            total: 0,
            lines: vec![],
        };

        self.carts.push(cart); //? erorr handling?

        Ok(())
    }
}

// test: 같은 상품에 대한 cartline이 추가되었을 때, 수량만 올라가야 한다
// advanced: order가 만들어지면, cart를 비운다
// -> 근데 만약에 pay를 하고 cancel이 되면???
