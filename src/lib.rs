#![no_std]
extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod model;

#[cfg(test)]
#[global_allocator]
static A : std::alloc::System = std::alloc::System;

#[test]
fn test_collection_macro() {
    use alloc::vec::Vec;
    use alloc::format;
    use core::str::FromStr;

    let c = collection!(model::types::Color::Black.into(),
                        model::types::Move::from_str("W B2").unwrap().into());

    assert_eq!("B", format!("{}", c[0]));
    assert_eq!("W B2", format!("{}", c[1]));
}

#[test]
fn test_command() {
    use alloc::vec::Vec;
    use alloc::format;
    use core::str::FromStr;
    use alloc::string::String;
    use crate::model::command::Command;
    use crate::model::command::CommandName;
    use crate::model::command::Args;

    let command: Command = "name".parse().unwrap();
    assert_eq!(format!("{}", command), String::from("name \n"));

    let command_id: Command = "0 list_commands \n".parse().unwrap();
    assert_eq!(format!("{}", command_id), String::from("0 list_commands \n"));
}