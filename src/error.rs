use crate::db::db::CartDbError;

#[derive(Debug)]
pub enum Error {
    // -- Model errors
    DBError,
}

impl From<CartDbError> for Error {
    fn from(error: CartDbError) -> Self {
        println!("DBError: {:?}", error);

        Error::DBError
    }
}
