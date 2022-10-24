use crate::{games::{tarok::tarok::Tarok, table::table::Table}, models::user::NewUser};

use super::traits::CheckName;


pub struct GameAggregator {
    tarok: Tarok,
    table: Table,
}

impl GameAggregator {
    pub fn new() -> Self {
        Self {
            tarok: Tarok{},
            table: Table{},
        }
    }
 
    pub fn validate_user(&self, user: &mut NewUser) {
        if !self.table.is_valid_name(&user.name) {
            user.invalidate();
            return;
        }
        if !self.tarok.is_valid_name(&user.name) {
            user.invalidate();
            return;
        }
        user.validate();
    }
}