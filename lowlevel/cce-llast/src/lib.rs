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

use syn;

pub mod ast;

pub fn parse(input: &str) -> syn::Result<Vec<ast::LLTopStatement>> {
  let file = syn::parse_file(input)?;

  let mut result = Vec::new();

  for item in file.items {
    result.push(ast::LLTopStatement::from(item));
  };

  Ok(result)
}