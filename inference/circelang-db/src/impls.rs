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

use crate::DatabaseObject;

use cce_ast::ast::*;
use circelang_hash::CirceHash;


impl DatabaseObject for CommandComponent {
    fn get_lookups(&self) -> Vec<u64> {
        match self {
            CommandComponent::Literal(s) => vec![s.hash()],
            CommandComponent::Keyword(k) => vec![k.hash()]
        }
    }

    fn get_conversion(&self) -> String {
        match self {
            CommandComponent::Literal(s) => format!("'{}'", s),
            CommandComponent::Keyword(k) => k.clone()
        }
    }
}

impl DatabaseObject for Command {
    fn get_lookups(&self) -> Vec<u64> {
        let mut lookups = Vec::new();

        for component in &self.components {
            lookups.extend(component.get_lookups());
        }

        for modifier in &self.modifiers {
            for component in modifier {
                lookups.extend(component.get_lookups());
            }
        }

        lookups
    }

    fn get_conversion(&self) -> String {
        let mut conversions = String::new();

        for component in &self.components {
            conversions.push_str(&component.get_conversion());
            conversions.push(' ');
        }

        for modifier in &self.modifiers {
            conversions.push_str("| ");
            for component in modifier {
                conversions.push_str(&component.get_conversion());
                conversions.push(' ');
            }
        }

        conversions
    }
}
