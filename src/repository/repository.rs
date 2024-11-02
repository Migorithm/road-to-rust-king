use crate::DB;
use crate::model::cart::Cart;
use crate::model::command::*;

pub struct CartRepository;
pub enum RepositoryError {
    CartNotFound{ user_id: u32 },
    CartCreationFailed,
    CartUpdateFailed,
    CartDeletionFailed,
}

// From Trait? map_err ?

// CRUD
impl CartRepository {
    pub fn get_cart(command: &GetCart) -> Result<Cart, RepositoryError> {
        let cart = DB.lock().unwrap().find(command.user_id).map_err(|_| RepositoryError::CartNotFound{ user_id: command.user_id })?;

        Ok(cart)
    }

    pub fn create_cart(command: &CreateCart) -> Result<(), RepositoryError> {
        DB.lock().unwrap().create(command.user_id).map_err(|_| RepositoryError::CartCreationFailed)?;

        // TODO: return new cart?
        Ok(())
    }   

    pub fn update_cart(command: &UpdateCart) -> Result<(), RepositoryError> {
        DB.lock().unwrap().update(&command.cart).map_err(|_| RepositoryError::CartUpdateFailed)?;

        Ok(())
    }

    pub fn delete_cart(command: &DeleteCart) -> Result<(), RepositoryError> {
        DB.lock().unwrap().delete(command.user_id).map_err(|_| RepositoryError::CartDeletionFailed)?;

        Ok(())
    }
}
