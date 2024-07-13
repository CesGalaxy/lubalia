use data::TokenData;

pub mod data;
pub mod metadata;
pub mod display;

#[derive(Debug, Clone)]
pub struct Token(pub TokenData);