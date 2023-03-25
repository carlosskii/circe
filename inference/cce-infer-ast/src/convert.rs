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


use cce_ast as ast;
use cce_lowlevel as ll;
use crate::nodes::*;

pub fn convert(program: Vec<ast::ParseNode>) -> Vec<ProgramNode> {
  program.into_iter().map(|node| match node {
    ast::ParseNode::Command(command) => ProgramNode::Command(convert_command(command)),
    ast::ParseNode::HowToStatement(howto) => ProgramNode::HowTo(convert_howto(howto)),
    ast::ParseNode::WhatIsStatement(whatis) => ProgramNode::WhatIs(convert_whatis(whatis))
  }).collect()
}

fn convert_command(command: ast::Command) -> CommandNode {
  CommandNode {
    command: command.components.into_iter().map(convert_command_component).collect(),
    modifiers: command.modifiers.into_iter().map(|modifier| modifier.into_iter().map(convert_command_component).collect()).collect()
  }
}

fn convert_command_component(component: ast::CommandComponent) -> CommandComponent {
  match component {
    ast::CommandComponent::Literal(literal) => CommandComponent::Literal(literal),
    ast::CommandComponent::Keyword(keyword) => CommandComponent::Keyword(keyword)
  }
}

fn convert_howto(howto: ast::HowToStatement) -> HowToNode {
  HowToNode {
    signature: howto.signature.into_iter().map(convert_command_component).collect(),
    body: howto.body.into_iter().map(convert_howto_command).collect()
  }
}

fn convert_howto_command(command: ast::HowToCommand) -> HowToCommand {
  match command {
    ast::HowToCommand::HighLevel(command) => HowToCommand::HighLevel(convert_command(command)),
    ast::HowToCommand::LowLevel(lowlevel) => HowToCommand::LowLevel(convert_lowlevel(lowlevel))
  }
}

fn convert_whatis(whatis: ast::WhatIsStatement) -> WhatIsNode {
  WhatIsNode {
    signature: whatis.signature.into_iter().map(convert_command_component).collect(),
    body: whatis.body.into_iter().map(convert_command).collect()
  }
}

fn convert_lowlevel(lowlevel: Vec<ll::ParseNode>) -> Vec<LowLevelCommand> {
  lowlevel.into_iter().map(|node| match node {
    ll::ParseNode::CommandCall(command) => LowLevelCommand::CommandCall(convert_lowlevel_command_call(command)),
    ll::ParseNode::VariableAssignment(assignment) => LowLevelCommand::VariableAssignment(LLVariableAssignment {
      name: assignment.name,
      value: convert_lowlevel_variable_value(assignment.value)
    })
  }).collect()
}

fn convert_lowlevel_command_call(command: ll::CommandCall) -> CommandNode {
  match command {
    ll::CommandCall::HighLevelSequence(hls) => {
      let mut parser: ast::Parser = ast::Parser::from(hls);

      let cmd: ast::Command = match parser.next().unwrap() {
        Some(ast::ParseNode::Command(cmd)) => cmd,
        _ => panic!("Invalid high-level sequence")
      };

      convert_command(cmd)
    }
  }
}

fn convert_lowlevel_variable_value(value: ll::VariableValue) -> LLVariableValue {
  match value {
    ll::VariableValue::Number(number) => LLVariableValue::Number(number),
    ll::VariableValue::HighLevelSequence(hls) => {
      let mut parser: ast::Parser = ast::Parser::from(hls);

      let cmd: ast::Command = match parser.next().unwrap() {
        Some(ast::ParseNode::Command(cmd)) => cmd,
        _ => panic!("Invalid high-level sequence")
      };

      LLVariableValue::CommandResult(convert_command(cmd))
    }
  }
}