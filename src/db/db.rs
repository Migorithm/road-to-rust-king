use crate::model::cart::Cart;

// smart pointer => Arc, MutexGuard, Rc, Cell, Box, String
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
