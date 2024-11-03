// use crate::model::cart::Cart;
// use crate::model::command::*;
// use crate::repository::repository::CartRepository;

// type Result<T> = std::result::Result<T, RepositoryError>;

// pub enum RepositoryError {
//     CartNotFound { user_id: u32 },
//     CartDbError,
// }

// // TOOD: feedback
// pub struct CartService;

// impl CartService {
//     pub fn get_cart(command: GetCart) -> Result<Cart> {
//         let cart =
//             CartRepository::get_cart(&command).map_err(|_| RepositoryError::CartNotFound {
//                 user_id: command.user_id,
//             })?;

//         Ok(cart)
//     }

//     pub fn add_to_cart(command: CreateCart) -> Result<()> {
//         CartRepository::get_cart(&GetCart {
//             user_id: command.user_id,
//         })
//         .map_err(|_| RepositoryError::CartNotFound {
//             user_id: command.user_id,
//         })?;

//         CartRepository::create_cart(&command).map_err(|_| RepositoryError::CartDbError)?;

//         Ok(())
//     }

//     pub fn remove_from_cart(command: DeleteCart) -> Result<()> {
//         // 이것만 From으로 할수 없나...
//         CartRepository::get_cart(&GetCart {
//             user_id: command.user_id,
//         })
//         .map_err(|_| RepositoryError::CartNotFound {
//             user_id: command.user_id,
//         })?;

//         CartRepository::delete_cart(&command).map_err(|_| RepositoryError::CartDbError)?;

//         Ok(())
//     }

//     pub fn update_cart_line(command: UpdateCart) -> Result<()> {
//         match CartRepository::get_cart(&GetCart {
//             user_id: command.cart.user_id,
//         }) {
//             Ok(_) => {
//                 CartRepository::update_cart(&command).map_err(|_| RepositoryError::CartDbError)?;
//             }
//             Err(_) => {
//                 CartRepository::create_cart(&CreateCart {
//                     user_id: command.cart.user_id,
//                 })
//                 .map_err(|_| RepositoryError::CartDbError)?;
//             }
//         }

//         Ok(())
//     }
// }
