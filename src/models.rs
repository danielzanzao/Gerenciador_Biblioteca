#[derive(Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Genero {
    Ficcao,
    Biografia,
    Poesia,
    Infantil,
    Romance,
    Outro,
}

#[derive(Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Livro {
    pub titulo: String,
    pub numero_paginas: u32,
    pub data_publicacao: String,
    pub genero: Genero,
}
