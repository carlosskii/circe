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
fn test_fn_basic() {
  let input = r#"
    fn foo() -> u32 {
      42
    }
  "#;

  let result = parse(input);

  assert_eq!(result.unwrap(), vec![
    LLTopStatement::LLFunction(LLFunction {
      name: "foo".to_string(),
      args: vec![],
      ret: LLType {
        name: "u32".to_string(),
      }
    })
  ])
}

#[test]
fn test_fn_args() {
  let input = r#"
    fn foo(bar: u32, baz: u64) -> u32 {
      42
    }
  "#;

  let result = parse(input);

  assert_eq!(result.unwrap(), vec![
    LLTopStatement::LLFunction(LLFunction {
      name: "foo".to_string(),
      args: vec![
        LLArgument {
          name: "bar".to_string(),
          ty: LLType {
            name: "u32".to_string(),
          }
        },
        LLArgument {
          name: "baz".to_string(),
          ty: LLType {
            name: "u64".to_string(),
          }
        }
      ],
      ret: LLType {
        name: "u32".to_string(),
      }
    })
  ])
}