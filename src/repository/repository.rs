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
        let cart = DB.lock().unwrap().iter()
        .find_map(|cart| cart.as_ref().filter(|c| c.user_id == command.user_id).cloned())
        // TODO
        .ok_or_else(|| {
            println!("Cart not found");
                RepositoryError::CartNotFound { user_id: command.user_id }
            })?;

        Ok(cart)
    }

    pub fn create_cart(command: &CreateCart) -> Result<(), RepositoryError> {
        DB.lock().unwrap().push(Some(command.new_cart.clone()));

        Ok(())
    }   

    pub fn update_cart(command: &UpdateCart) -> Result<(), RepositoryError> {
        let mut db = DB.lock().unwrap();

        // let cart_index = db.iter()
        // .position(|c| c.as_ref().unwrap().user_id == command.cart.user_id)
        // .unwrap();

        let existing_cart = db.iter_mut().find(|c| c.as_ref().unwrap().user_id == command.cart.user_id).unwrap();

        *existing_cart = Some(command.cart.clone());

        Ok(())
    }

    pub fn delete_cart(command: &DeleteCart) -> Result<(), RepositoryError> {
        let mut db = DB.lock().unwrap();

        let cart_index = db.iter()
        .position(|c| c.as_ref().unwrap().user_id == command.user_id)
        .unwrap();

        db.swap_remove(cart_index);

        Ok(())
    }
}

