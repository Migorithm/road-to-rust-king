use std::{
    fmt::Error,
    sync::{LazyLock, RwLock},
};

use super::{
    command::{AddCartLine, UpdateCartLine}, //? is this right?
    product::{JhProduct, Product},
};

#[derive(Clone, Debug)]
pub struct Cart {
    pub(crate) user_id: u32,
    pub(crate) total: u32,
    pub(crate) lines: Vec<CartLine>,
}

//? total의 처리를 Controller에서 해야할지 Model에서 해야할지 잘 모르겠음...

impl Cart {
    pub fn add_line(&mut self, command: AddCartLine) -> Result<(), String> {
        // 만약 카트 라인 중에 이미 같은 product_id를 가진 라인이 존재하면 에러 반환
        if self
            .lines
            .iter()
            .any(|line| line.product_id == command.product_id)
        {
            return Err("Product Already exists".to_string());
        }

        // 새로운 CartLine을 생성하여 라인에 추가
        let line = CartLine {
            product_id: command.product_id,
            quantity: command.quantity,
        };

        self.lines.push(line);

        Ok(())
    }

    pub fn update_line(&mut self, command: UpdateCartLine) -> Result<(), String> {
        // 만약 command의 quantity가 0이면, 해당 product_id를 가진 라인을 제거
        if command.quantity == 0 {
            self.lines
                .retain(|line| line.product_id != command.product_id);
        } else {
            // quantity > 0인 경우 새로운 CartLine을 생성하여 라인에 추가
            let line = CartLine {
                product_id: command.product_id,
                quantity: command.quantity,
            };

            self.lines.push(line);
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CartLine {
    pub(crate) product_id: u32,
    pub(crate) quantity: u32,
}

impl CartLine {
    pub fn new(product_id: u32, quantity: u32) -> Self {
        Self {
            product_id,
            quantity,
        }
    }

    pub fn update_quantity(&mut self, quantity: u32) {
        self.quantity = quantity;
    }
}
