use super::cart::Cart;

#[derive(Debug, Clone)]
pub struct CreateCart {
    pub user_id: u32,
}

#[derive(Debug, Clone)]
pub struct GetCart {
    pub user_id: u32,
}

#[derive(Debug, Clone)]
pub struct UpdateCart {
    pub cart: Cart,
}

#[derive(Debug, Clone)]
pub struct DeleteCart {
    pub user_id: u32,
}
