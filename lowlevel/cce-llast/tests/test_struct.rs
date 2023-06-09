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

use cce_llast::{parse, ast::*};


#[test]
fn test_struct_basic() {
  let input = r#"
    struct Foo {
      bar: u32
    }
  "#;

  let result = parse(input);

  assert_eq!(result.unwrap(), vec![
    LLTopStatement::LLStruct(LLStruct {
      name: "Foo".to_string(),
      fields: vec![
        LLStructField {
          name: "bar".to_string(),
          ty: LLType {
            name: "u32".to_string(),
          }
        }
      ]
    })
  ])
}

#[test]
#[should_panic]
fn test_struct_no_fields() {
  let input = r#"
    struct Foo {}
  "#;

  parse(input).unwrap();
}

#[test]
#[should_panic]
fn test_struct_no_name() {
  let input = r#"
    struct {}
  "#;

  parse(input).unwrap();
}