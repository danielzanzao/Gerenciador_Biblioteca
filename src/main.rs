use chrono::NaiveDate;
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::str::FromStr;
mod utils;
use utils::{validar_data, validar_string, validar_obrigatorio, validar_numero};

#[derive(Debug)]
pub enum Genero {
    Ficcao,
    NaoFiccao,
    Ciencia,
    Tecnologia,
    Historia,
    Outro,
}

impl FromStr for Genero {
    type Err = String;

    fn from_str(input: &str) -> Result<Genero, Self::Err> {
        match input.to_lowercase().as_str() {
            "ficcao" => Ok(Genero::Ficcao),
            "não ficção" | "nao ficcao" => Ok(Genero::NaoFiccao),
            "ciencia" => Ok(Genero::Ciencia),
            "tecnologia" => Ok(Genero::Tecnologia),
            "historia" => Ok(Genero::Historia),
            "outro" => Ok(Genero::Outro),
            _ => Err(format!("Gênero inválido: {}", input)),
        }
    }
}

#[derive(Debug)]
pub struct Livro {
    pub titulo: String,
    pub numero_paginas: u32,
    pub data_publicacao: NaiveDate,
    pub genero: Genero,
}

impl Livro {
    pub fn novo(titulo: &str, numero_paginas: u32, data_publicacao: &str, genero: &str) -> Result<Livro, String> {
        // Validações
        validar_string(titulo, 100)?;
        validar_numero(numero_paginas, 1, 10000)?;
        let data = validar_data(data_publicacao)?; //Faz validação e converte para NaiveDate
        validar_string(genero, 50)?;
        validar_obrigatorio(titulo)?;
        validar_obrigatorio(genero)?;

        //String pra Genero
        let genero_enum = genero.parse::<Genero>()?;

        Ok(Livro {
            titulo: titulo.to_string(),
            numero_paginas,
            data_publicacao: data,
            genero: genero_enum,
        })
    }

    pub fn salvar(&self, arquivo: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(arquivo)?;

        writeln!(
            file,
            "{},{},{:?},{}",
            self.titulo,
            self.numero_paginas,
            self.genero,
            self.data_publicacao.format("%Y-%m-%d")
        )?;
        Ok(())
    }
}

fn main() {
    loop {
        println!("=== Cadastro de Livros ===");
        let mut titulo = String::new();
        let mut numero_paginas = String::new();
        let mut data_publicacao = String::new();
        let mut genero = String::new();

        print!("Digite o título do livro: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut titulo).unwrap();
        let titulo = titulo.trim();

        print!("Digite o número de páginas: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut numero_paginas).unwrap();
        let numero_paginas: u32 = match numero_paginas.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Erro: Número inválido.");
                continue;
            }
        };

        print!("Digite a data de publicação (AAAA-MM-DD): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut data_publicacao).unwrap();
        let data_publicacao = data_publicacao.trim();
        println!("Escolha o gênero do livro:");
        println!("1. Ficção");
        println!("2. Não Ficção");
        println!("3. Ciência");
        println!("4. Tecnologia");
        println!("5. História");
        println!("6. Outro");
        print!("Digite a sua escolha: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut genero).unwrap();
        let genero = match genero.trim() {
            "1" => "Ficcao",
            "2" => "Nao Ficcao",
            "3" => "Ciencia",
            "4" => "Tecnologia",
            "5" => "Historia",
            "6" => "Outro",
            _ => {
                println!("Erro: Opção de gênero inválida.");
                continue;
            }
        };

        match Livro::novo(titulo, numero_paginas, data_publicacao, genero) {
            Ok(livro) => {
                println!("Livro criado com sucesso: {:?}", livro);

                if let Err(e) = livro.salvar("livros.txt") {
                    println!("Erro ao salvar o livro: {}", e);
                } else {
                    println!("Livro salvo com sucesso no arquivo.");
                }
            }
            Err(e) => {
                println!("Erro ao criar o livro: {}", e);
            }
        }

        println!("Deseja cadastrar outro livro? (s/n)");
        let mut resposta = String::new();
        io::stdin().read_line(&mut resposta).unwrap();
        if resposta.trim().to_lowercase() != "s" {
            break;
        }
    }
}
