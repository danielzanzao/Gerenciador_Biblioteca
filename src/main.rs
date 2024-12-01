use chrono::NaiveDate;
use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead};

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

impl std::str::FromStr for Genero {
    type Err = String;

    fn from_str(input: &str) -> Result<Genero, Self::Err> {
        match input.to_lowercase().as_str() {
            "ficcao" => Ok(Genero::Ficcao),
            "nao ficcao" | "não ficção" => Ok(Genero::NaoFiccao),
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
        validar_string(titulo, 100)?;
        validar_numero(numero_paginas, 1, 10000)?;
        let data = validar_data(data_publicacao)?;
        validar_string(genero, 50)?;
        validar_obrigatorio(titulo)?;
        validar_obrigatorio(genero)?;

        let genero_enum = genero.parse::<Genero>()?;

        Ok(Livro {
            titulo: titulo.to_string(),
            numero_paginas,
            data_publicacao: data,
            genero: genero_enum,
        })
    }

    pub fn salvar(&self, arquivo: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(arquivo)?;
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

// Função para listar os livros
fn listar_livros(arquivo: &str) -> io::Result<()> {
    let file = File::open(arquivo)?;
    let reader = BufReader::new(file);
    println!("=== Lista de Livros ===");
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        println!("{}. {}", i + 1, line);
    }
    Ok(())
}

// Função para alterar um livro
fn alterar_livro(arquivo: &str) -> io::Result<()> {
    println!("Funcionalidade de alteração de livros ainda não implementada.");
    Ok(())
}

// Função para deletar um livro
fn deletar_livro(arquivo: &str) -> io::Result<()> {
    println!("Funcionalidade de deleção de livros ainda não implementada.");
    Ok(())
}

fn main() {
    let arquivo = "livros.txt";

    loop {
        println!("=== Sistema da Biblioteca Libri Mendes ===");
        println!("Escolha a operação que deseja fazer:");
        println!("1. Adicionar Livros");
        println!("2. Listar os Livros");
        println!("3. Alterar algum livro");
        println!("4. Deletar algum livro");
        println!("0. Sair");
        print!("Digite a sua escolha: ");
        io::stdout().flush().unwrap();

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).unwrap();

        match escolha.trim() {
            "1" => {
                println!("=== Adicionar Livro ===");
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

                        if let Err(e) = livro.salvar(arquivo) {
                            println!("Erro ao salvar o livro: {}", e);
                        } else {
                            println!("Livro salvo com sucesso no arquivo.");
                        }
                    }
                    Err(e) => {
                        println!("Erro ao criar o livro: {}", e);
                    }
                }
            }
            "2" => {
                if let Err(e) = listar_livros(arquivo) {
                    println!("Erro ao listar os livros: {}", e);
                }
            }
            "3" => {
                if let Err(e) = alterar_livro(arquivo) {
                    println!("Erro ao alterar o livro: {}", e);
                }
            }
            "4" => {
                if let Err(e) = deletar_livro(arquivo) {
                    println!("Erro ao deletar o livro: {}", e);
                }
            }
            "0" => {
                println!("Saindo do sistema. Até logo!");
                break;
            }
            _ => println!("Opção inválida! Tente novamente."),
        }
    }
}
