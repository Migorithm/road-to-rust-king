pub enum JhProduct {
    CheeseDonut,
    ChocoDonut,
    GlazedDonut,
    Americano,
    Latte,
    Cappuccino,
}

pub struct Product {
    pub id: u32,
    pub kind: JhProduct,
    pub price: u32,
}
