use chrono::NaiveDate;
use std::str::FromStr;
use std::fmt::{self, Display, Formatter};

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
