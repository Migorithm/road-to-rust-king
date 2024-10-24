use crate::db;

pub struct CartRepository;

impl CartRepository {
    pub fn find(user_id: u32) -> Option<Cart> {
        // 사용자 ID로 카트를 찾고, 없으면 NotFound 에러 반환
        db::find_cart_by_user_id(user_id).ok_or(CartError::NotFound)
    }

    pub fn insert(user_id: u32, product_id: u32, quantity: u32) -> Result<(), CartError> {
        // 사용자 ID로 카트를 찾고, 없으면 NotFound 에러 반환
        let mut cart = db::find_cart_by_user_id(user_id).unwrap_or_else(|| Cart {
            user_id,
            total: 0,
            lines: vec![],
        });

        // 카트에 상품을 추가
        cart.add_line(product_id, quantity); // modified cart quantity+1

        // 카트를 저장하고, 실패 시 DbError 반환
        db::save_cart(cart).map_err(|_| CartError::DbError)
    }

    pub fn delete(user_id: u32, product_id: u32) -> Result<(), CartError> {
        // 사용자 ID로 카트를 찾고, 없으면 NotFound 에러 반환
        let mut cart = db::find_cart_by_user_id(user_id).ok_or(CartError::NotFound)?;

        // 카트에서 상품을 제거
        cart.remove_line(product_id);

        // 카트를 저장하고, 실패 시 DbError 반환
        db::save_cart(cart).map_err(|_| CartError::DbError)
    }

    pub fn update(user_id: u32, product_id: u32, quantity: u32) -> Result<(), CartError> {
        let mut cart = db::find_cart_by_user_id(user_id).unwrap_or_else(|| Cart {
            user_id,
            total: 0,
            lines: vec![],
        });

        // 카트의 상품 수량을 업데이트
        cart.update_line(product_id, quantity);

        // 카트를 저장하고, 실패 시 DbError 반환
        db::save_cart(cart).map_err(|_| CartError::DbError)
    }
}
