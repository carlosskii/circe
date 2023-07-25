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

/****************************************
* LLTopStatement
****************************************/
#[derive(Debug, PartialEq, Clone)]
pub enum LLTopStatement {
  LLFunction(LLFunction),
  LLStruct(LLStruct)
}

impl From<syn::Item> for LLTopStatement {
  fn from(item: syn::Item) -> Self {
    match item {
      syn::Item::Fn(f) => LLTopStatement::LLFunction(LLFunction::from(f)),
      syn::Item::Struct(s) => LLTopStatement::LLStruct(LLStruct::from(s)),
      _ => panic!("Not implemented")
    }
  }
}

/****************************************
* LLFunction
****************************************/
#[derive(Debug, PartialEq, Clone)]
pub struct LLFunction {
  pub name: String,
  pub args: Vec<LLArgument>,
  pub ret: LLType
}

impl From<syn::ItemFn> for LLFunction {
  fn from(item: syn::ItemFn) -> Self {
    let name = item.sig.ident.to_string();
    let args = item.sig.inputs.iter().map(LLArgument::from).collect();
    let ret = LLType::from(&item.sig.output);

    LLFunction { name, args, ret }
  }
}

/****************************************
* LLStruct
****************************************/
#[derive(Debug, PartialEq, Clone)]
pub struct LLStruct {
  pub name: String,
  pub fields: Vec<LLStructField>
}

impl From<syn::ItemStruct> for LLStruct {
  fn from(item: syn::ItemStruct) -> Self {
    let name = item.ident.to_string();
    let fields: Vec<LLStructField> = item.fields.iter().map(LLStructField::from).collect();

    if fields.is_empty() {
      panic!("Structs must have at least one field");
    }

    LLStruct { name, fields }
  }
}

/****************************************
* LLArgument
****************************************/
#[derive(Debug, PartialEq, Clone)]
pub struct LLArgument {
  pub name: String,
  pub ty: LLType
}

impl From<&syn::FnArg> for LLArgument {
  fn from(arg: &syn::FnArg) -> Self {
    match arg {
      syn::FnArg::Typed(t) => {
        let name = match &*t.pat {
          syn::Pat::Ident(i) => i.ident.to_string(),
          _ => panic!("Not implemented")
        };

        let ty = LLType::from(t.ty.as_ref());

        LLArgument { name, ty }
      },
      _ => panic!("Not implemented")
    }
  }
}

/****************************************
* LLType
****************************************/
#[derive(Debug, PartialEq, Clone)]
pub struct LLType {
  pub name: String
}

impl From<&syn::ReturnType> for LLType {
  fn from(ret: &syn::ReturnType) -> Self {
    match ret {
      syn::ReturnType::Default => LLType { name: "void".to_string() },
      syn::ReturnType::Type(_, t) => LLType::from(t.as_ref())
    }
  }
}

impl From<&syn::Type> for LLType {
  fn from(ty: &syn::Type) -> Self {
    match ty {
      syn::Type::Path(p) => {
        let name = p.path.segments.iter().map(|seg| seg.ident.to_string()).collect::<Vec<String>>().join("::");

        LLType { name }
      },
      _ => panic!("Not implemented")
    }
  }
}

/****************************************
* LLStructField
****************************************/
#[derive(Debug, PartialEq, Clone)]
pub struct LLStructField {
  pub name: String,
  pub ty: LLType
}

impl From<&syn::Field> for LLStructField {
  fn from(field: &syn::Field) -> Self {
    let name = field.ident.as_ref().unwrap().to_string();
    let ty = LLType::from(&field.ty);

    LLStructField { name, ty }
  }
}