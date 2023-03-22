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


#[derive(Debug, Clone, PartialEq)]
pub enum RootInferNode {
  Command(Command)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
  pub root: GeneralCommand,
  pub modifiers: Vec<GeneralCommand>
}

#[derive(Debug, Clone, PartialEq)]
pub enum GeneralCommand {
  ActiveCommand(ActiveCommand),
  PassiveCommand(PassiveCommand)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveCommand {
  pub action: Action,
  pub target: Target
}

#[derive(Debug, Clone, PartialEq)]
pub enum PassiveCommand {
  PropertyDeclaration(PropertyDeclaration)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
  pub root: String
}

#[derive(Debug, Clone, PartialEq)]
pub struct Target {
  pub actor: Object,
  pub scene: Option<Object>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
  Property(ObjectProperty)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectProperty {
  pub root: String,
  pub property: Vec<String>
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDeclaration {
  pub object: Object,
  pub value: Object
}