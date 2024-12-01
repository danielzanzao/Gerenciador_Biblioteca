use chrono::NaiveDate;
use chrono::Datelike;
use std::str::FromStr;
use std::fmt::{self, Display, Formatter};


// Validação do nome
pub fn validar_string(valor: &str, max_len: usize) -> Result<(), String> {
    if valor.trim().is_empty() {
        return Err("O título não pode ser vazio.".to_string());
    }
    if valor.len() > max_len {
        return Err(format!("O título não pode ter mais de {} caracteres.", max_len));
    }
    Ok(())
}

// numero de paginas
pub fn validar_numero(valor: u32, min: u32, max: u32) -> Result<(), String> {
    if valor < min || valor > max {
        return Err(format!("O número de páginas deve estar entre {} e {}.", min, max));
    }
    Ok(())
}

// Validação dos dias especificos
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
                        29 
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_validar_string_valida() {
        assert!(validar_string("Título válido", 20).is_ok());
    }

    #[test]
    fn test_validar_string_vazia() {
        assert!(validar_string("", 20).is_err());
    }

    #[test]
    fn test_validar_string_excede_tamanho() {
        assert!(validar_string("Um título muito longo que excede o limite", 10).is_err());
    }

    #[test]
    fn test_validar_numero_valido() {
        assert!(validar_numero(100, 1, 200).is_ok());
    }

    #[test]
    fn test_validar_numero_abaixo_minimo() {
        assert!(validar_numero(0, 1, 200).is_err());
    }

    #[test]
    fn test_validar_numero_acima_maximo() {
        assert!(validar_numero(201, 1, 200).is_err());
    }

    #[test]
    fn test_validar_data_valida() {
        assert!(validar_data("2023-12-25").is_ok());
    }

    #[test]
    fn test_validar_data_invalida() {
        assert!(validar_data("2023-13-01").is_err()); // Mês inválido
    }

    #[test]
    fn test_validar_data_dia_invalido() {
        assert!(validar_data("2023-02-30").is_err()); // Dia inválido para fevereiro
    }

    #[test]
    fn test_validar_obrigatorio_valor_presente() {
        assert!(validar_obrigatorio("Presente").is_ok());
    }

    #[test]
    fn test_validar_obrigatorio_valor_ausente() {
        assert!(validar_obrigatorio("").is_err());
    }
}


