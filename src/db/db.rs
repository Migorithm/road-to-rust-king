use crate::model::cart::Cart;

// TODO: field 없이 접근을 쉽게 할 수 있나? -> smart pointer => Arc, MutexGuard, Rc, Cell, Box, String
pub struct CartDb(Vec<Option<Cart>>);

#[derive(Debug)]
pub enum CartDbError {
    DBError,
}

impl CartDb {
    pub fn new() -> Self {
        Self(Vec::<Option<Cart>>::new())
    }
}

impl CartDb {
    // TODO: Result<Option<Cart>> 로 바꿀 수 있나?
    // 결과값이 없는데 에러로 반환?
    pub fn find(&self, user_id: u32) -> Result<Cart, CartDbError> {
        self.iter()
            .find_map(|cart| cart.as_ref().filter(|c| c.user_id == user_id).cloned())
            .ok_or_else(|| {
                println!("Cart not found");
                CartDbError::DBError
            })
    }

    pub fn create(&mut self, user_id: u32) -> Result<Cart, CartDbError> {
        let new_cart = Cart::new(user_id);
        self.push(Some(new_cart.clone())); // newCart를 복제하여 이동 방지

        Ok(new_cart)
    }

    pub fn update(&mut self, cart: &Cart) -> Result<Cart, CartDbError> {
        let cart_index = self
            .iter()
            .position(|c| c.as_ref().unwrap().user_id == cart.user_id)
            .unwrap();

        self.0[cart_index] = Some(cart.clone());

        Ok(cart.clone())
    }

    pub fn delete(&mut self, user_id: u32) -> Result<(), CartDbError> {
        let cart_index = self
            .iter()
            .position(|cart| cart.as_ref().unwrap().user_id == user_id)
            .unwrap();

        self.remove(cart_index);

        Ok(())
    }
}

impl std::ops::Deref for CartDb {
    type Target = Vec<Option<Cart>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CartDb {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
