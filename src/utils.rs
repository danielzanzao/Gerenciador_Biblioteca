use chrono::NaiveDate;
use chrono::Datelike;
use std::str::FromStr;
use std::fmt::{self, Display, Formatter};


// Validação de nome
pub fn validar_string(valor: &str, max_len: usize) -> Result<(), String> {
    if valor.trim().is_empty() {
        return Err("O título não pode ser vazio.".to_string());
    }
    if valor.len() > max_len {
        return Err(format!("O título não pode ter mais de {} caracteres.", max_len));
    }
    Ok(())
}

// Validação de número de páginas
pub fn validar_numero(valor: u32, min: u32, max: u32) -> Result<(), String> {
    if valor < min || valor > max {
        return Err(format!("O número de páginas deve estar entre {} e {}.", min, max));
    }
    Ok(())
}

// Validação detalhada da data
pub fn validar_data(data_str: &str) -> Result<NaiveDate, String> {
    match NaiveDate::parse_from_str(data_str, "%Y-%m-%d") {
        Ok(data) => {
            let ano = data.year();
            let mes = data.month();
            let dia = data.day();

            if mes > 12 {
                return Err("O mês não pode ser superior a 12.".to_string());
            }

            let dias_no_mes = match mes {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => {
                    if (ano % 4 == 0 && ano % 100 != 0) || ano % 400 == 0 {
                        29 // Ano bissexto
                    } else {
                        28
                    }
                }
                _ => return Err("Mês inválido.".to_string()),
            };

            if dia > dias_no_mes {
                return Err(format!(
                    "O dia {} não é válido para o mês {}.",
                    dia, mes
                ));
            }

            Ok(data)
        }
        Err(_) => Err("Data inválida. O formato deve ser AAAA-MM-DD.".to_string()),
    }
}

// Validação de tudo confere
pub fn validar_obrigatorio(valor: &str) -> Result<(), String> {
    if valor.trim().is_empty() {
        return Err("Este campo é obrigatório.".to_string());
    }
    Ok(())
}



