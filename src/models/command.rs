pub struct AddCartLine {
    pub user_id: u32,
    pub product_id: u32,
    pub quantity: u32,
}

pub struct UpdateCartLine {
    pub user_id: u32,
    pub product_id: u32,
    pub quantity: u32,
}
