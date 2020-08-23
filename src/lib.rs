#![no_std]
extern crate alloc;
#[cfg(test)]
extern crate std;


pub mod model;
#[cfg(test)]
#[global_allocator]
static A : std::alloc::System = std::alloc::System;

pub mod prelude {
    pub use crate::model::Command;
    pub use crate::model::Response;
    pub use crate::model::Failure;
}


#[test]
fn test_collection_macro() {
    use alloc::vec::Vec;
    use alloc::format;
    use core::str::FromStr;

    let c = collection!(model::Color::Black.into(),
                        model::Move::from_str("W B2").unwrap().into());

    assert_eq!("B", format!("{}", c[0]));
    assert_eq!("W B2", format!("{}", c[1]));
}

#[test]
fn test_command() {
    use alloc::string::String;
    use crate::model::Command;
    use alloc::string::ToString;
    scrub_log::init_with_filter_string("debug").unwrap();
    
    let command: Command = "name".parse().unwrap();
    assert_eq!(command.to_string(), String::from("name\n"));
    
    let command_id: Command = "0 list_commands \n".parse().unwrap();
    assert_eq!(command_id.to_string(), String::from("0 list_commands\n"));
}

#[test]
fn test_response() {
    use crate::prelude::*;
    use alloc::string::ToString;

    let response: Response = "=0".parse().unwrap();
    assert_eq!("=0\n\n", response.to_string());

    let response = "=".parse::<Response>().unwrap();
    assert_eq!("=\n\n", response.to_string());
    
    let response = "= c15 d12 a13\nj3 f3 g3 h4 h5\n\n".parse::<Response>().unwrap();
    assert_eq!("= C15 D12 A13\nJ3 F3 G3 H4 H5\n\n", response.to_string());

    let response = "=\n\n".parse::<Response>().unwrap();
    assert_eq!("=\n\n", response.to_string());
}
