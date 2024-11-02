use crate::db::db::CartDbError;

pub type Result<T> = std::result::Result<T, Error>;

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
