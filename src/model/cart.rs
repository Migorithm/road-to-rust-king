#[derive(Clone, Debug)]
pub struct Cart {
    pub user_id: u32,
    pub total: u32,
    pub lines: Vec<CartLine>,
}

impl Cart {
    pub fn new(user_id: u32) -> Self {
        Self {
            user_id,
            total: 0,
            lines: Vec::<CartLine>::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CartLine {
    pub product_id: u32,
    pub quantity: u32,
}