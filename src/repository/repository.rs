use crate::model::cart::Cart;
use crate::model::command::*;
use crate::DB;

pub struct CartRepository;
pub enum RepositoryError {
    TimeOut,
    BackEndDBError,
}

// From Trait? map_err ?

// Error? - network error, duplicate primary key error or other constraints, transaction error
impl CartRepository {
    pub fn get_cart(command: &GetCart) -> Result<Option<Cart>, RepositoryError> {
        let cart = DB
            .lock()
            .unwrap()
            .iter()
            .find(|cart| cart.user_id == command.user_id)
            .cloned();

        Ok(cart)
    }

    pub fn create_cart(command: &CreateCart) -> Result<(), RepositoryError> {
        DB.lock().unwrap().push(command.new_cart.clone());

        Ok(())
    }

    pub fn update_cart(command: &UpdateCart) -> Result<(), RepositoryError> {
        let mut db = DB.lock().unwrap();
        let existing_cart = db
            .iter_mut()
            .find(|c| c.user_id == command.cart.user_id)
            .unwrap();

        *existing_cart = command.cart.clone();

        Ok(())
    }

    pub fn delete_cart(command: &DeleteCart) -> Result<(), RepositoryError> {
        let mut db = DB.lock().unwrap();

        let cart_index = db
            .iter()
            .position(|c| c.user_id == command.user_id)
            .unwrap();

        db.swap_remove(cart_index);

        Ok(())
    }
}
