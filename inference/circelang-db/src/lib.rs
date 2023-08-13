/*

Copyright (C) 2023 Carlos Kieliszewski

This file is part of the Circe Project.

Circe is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option)
any later version.

Circe is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
Circe. If not, see <https://www.gnu.org/licenses/>. 

*/

use std::collections::HashMap;
use circelang_hash::CirceHash;

mod impls;


pub struct Database {
    entries: Vec<(u64, String)>,
    lookup: HashMap<u64, Vec<u64>>
}

impl Database {
    pub fn new() -> Database {
        Database {
            entries: Vec::new(),
            lookup: HashMap::new()
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

pub trait DatabaseObject {
    fn get_lookups(&self) -> Vec<u64>;
    fn get_conversion(&self) -> String;
}
