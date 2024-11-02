use std::sync::{Arc, Mutex, MutexGuard};

use crate::model::cart::Cart;



pub struct CartDb {
    // TODO: 이거 없이 할 수 없나?
    cart_db: Vec<Option<Cart>>,
}

#[derive(Debug)]
pub enum CartDbError {
    DBError,
}

impl CartDb {
    pub fn new() -> Self {
        Self {
            cart_db: Vec::<Option<Cart>>::new(),
        }
    }
}

impl CartDb {
    // TODO: Result<Option<Cart>> 로 바꿀 수 있나?
    // 결과값이 없는데 에러로 반환?
    pub fn find(&self, user_id: u32) -> Result<Cart, CartDbError> {
        self.cart_db
            .iter()
            .find_map(|cart| cart.as_ref().filter(|c| c.user_id == user_id).cloned())
            .ok_or_else(|| {
                println!("Cart not found");
                CartDbError::DBError
            })
    }

    pub fn create(&mut self, user_id: u32) -> Result<Cart, CartDbError> {
        let new_cart = Cart::new(user_id);

        self.cart_db.push(Some(new_cart.clone())); // newCart를 복제하여 이동 방지

        Ok(new_cart)
    }

    pub fn update(&mut self, cart: &Cart) -> Result<Cart, CartDbError> {
        let cart_index = self.cart_db
            .iter()
            .position(|c| c.as_ref().unwrap().user_id == cart.user_id)
            .unwrap();

        self.cart_db[cart_index] = Some(cart.clone());
        
        Ok(cart.clone())
    }

    pub fn delete(&mut self, user_id: u32) -> Result<(), CartDbError> {
        let cart_index = self.cart_db
            .iter()
            .position(|cart| cart.as_ref().unwrap().user_id == user_id)
            .unwrap();

        self.cart_db.remove(cart_index);

        Ok(())
    }
}


