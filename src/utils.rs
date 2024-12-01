use chrono::NaiveDate;
use std::str::FromStr;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

// Validação de nome
pub fn validar_string(valor: &str, max_len: usize) -> Result<(), String> {
    if valor.is_empty() {
        return Err("O campo não pode ser vazio.".to_string());
    }
    if valor.len() > max_len {
        return Err(format!("O campo não pode ter mais de {} caracteres.", max_len));
    }
    Ok(())
}

// Validação de num de pagina
pub fn validar_numero(valor: u32, min: u32, max: u32) -> Result<(), String> {
    if valor < min || valor > max {
        return Err(format!("O valor deve estar entre {} e {}.", min, max));
    }
    Ok(())
}

// Validação de data
pub fn validar_data(data_str: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(data_str, "%Y-%m-%d").map_err(|_| "Data inválida. O formato deve ser AAAA-MM-DD.".to_string())
}

// Validação de tudo confere
pub fn validar_obrigatorio(valor: &str) -> Result<(), String> {
    if valor.trim().is_empty() {
        return Err("Este campo é obrigatório.".to_string());
    }
    Ok(())
}

pub fn carregar_livros(arquivo: &str) -> io::Result<Vec<Livro>> {
    let file = File::open(arquivo)?;
    let reader = BufReader::new(file);

    let mut livros = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let campos: Vec<&str> = line.split(',').collect();

        if campos.len() != 4 {
            continue;
        }

        let titulo = campos[0].to_string();
        let numero_paginas = campos[1].parse::<u32>().unwrap_or(0);
        let genero = campos[2].parse::<Genero>().unwrap_or(Genero::Outro);
        let data_publicacao = NaiveDate::parse_from_str(campos[3], "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd(1900, 1, 1));

        livros.push(Livro {
            titulo,
            numero_paginas,
            data_publicacao,
            genero,
        });
    }

    Ok(livros)
}

