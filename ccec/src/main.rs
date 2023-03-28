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


use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::process::exit;

use clap::Parser as ClapParser;

use cce_ast::{Parser, ParseNode};
use cce_infer_ast::convert;
use cce_infer::Deducer;


#[derive(ClapParser)]
#[command(name = "CCEC")]
#[command(about = "The Circe Compiler", long_about = None)]
#[command(version = "0.1.0")]
#[command(author = "Carlos Kieliszewski")]
struct CLI {
  filename: String
}


fn main() {
  let cli = CLI::parse();

  let path: &Path = Path::new(&cli.filename);
  if !path.exists() {
    println!("File not found");
  }

  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut parser = Parser::from(contents);
  let mut nodes: Vec<ParseNode> = Vec::new();

  loop {
    match parser.next() {
      Ok(Some(node)) => nodes.push(node),
      Ok(None) => break,
      Err(err) => {
        println!("Error: {}", err);
        exit(1);
      }
    }
  }

  let ast = convert(nodes);
  let mut deducer = Deducer::new();
  for node in ast {
    deducer.add_node(node);
  }

  let result = deducer.deduce();
  println!("{:?}", result);
}