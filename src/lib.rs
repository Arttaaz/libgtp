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
