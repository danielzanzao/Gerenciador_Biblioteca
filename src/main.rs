use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};

#[derive(Debug, Serialize, Deserialize)]
pub enum Genero {
    Ficcao,
    Biografia,
    Poesia,
    Infantil,
    Romance,
    Outro,
}

impl std::str::FromStr for Genero {
    type Err = String;

    fn from_str(input: &str) -> Result<Genero, Self::Err> {
        match input.to_lowercase().as_str() {
            "ficcao" => Ok(Genero::Ficcao),
            "biografia" => Ok(Genero::Biografia),
            "poesia" => Ok(Genero::Poesia),
            "infantil" => Ok(Genero::Infantil),
            "romance" => Ok(Genero::Romance),
            "outro" => Ok(Genero::Outro),
            _ => Err(format!("Gênero inválido: {}", input)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Livro {
    pub titulo: String,
    pub numero_paginas: u32,
    pub data_publicacao: NaiveDate,
    pub genero: Genero,
}

impl Livro {
    pub fn novo(titulo: &str, numero_paginas: u32, data_publicacao: &str, genero: &str) -> Result<Livro, String> {
        let data = NaiveDate::parse_from_str(data_publicacao, "%Y-%m-%d")
            .map_err(|_| "Data inválida, use o formato AAAA-MM-DD".to_string())?;
        let genero_enum = genero.parse::<Genero>()?;

        Ok(Livro {
            titulo: titulo.to_string(),
            numero_paginas,
            data_publicacao: data,
            genero: genero_enum,
        })
    }
}

fn salvar_livros(arquivo: &str, livros: &Vec<Livro>) -> io::Result<()> {
    let serialized = bincode::serialize(livros).expect("Erro ao serializar os dados.");
    fs::write(arquivo, serialized)?;
    Ok(())
}

fn carregar_livros(arquivo: &str) -> io::Result<Vec<Livro>> {
    let data = fs::read(arquivo).unwrap_or_else(|_| Vec::new());
    if data.is_empty() {
        return Ok(Vec::new());
    }
    let livros: Vec<Livro> = bincode::deserialize(&data).expect("Erro ao deserializar os dados.");
    Ok(livros)
}

fn listar_livros(arquivo: &str) -> io::Result<()> {
    let livros = carregar_livros(arquivo)?;

    if livros.is_empty() {
        println!("Não há livros cadastrados.");
    } else {
        println!("=== Lista de Livros ===");
        for (i, livro) in livros.iter().enumerate() {
            println!(
                "{}. Título: {}, Páginas: {}, Publicação: {}, Gênero: {:?}",
                i + 1,
                livro.titulo,
                livro.numero_paginas,
                livro.data_publicacao,
                livro.genero
            );
        }
    }
    Ok(())
}

fn deletar_livro(arquivo: &str) -> io::Result<()> {
    let mut livros = carregar_livros(arquivo)?;

    if livros.is_empty() {
        println!("Não há livros cadastrados para deletar.");
        return Ok(());
    }

    println!("=== Lista de Livros ===");
    for (i, livro) in livros.iter().enumerate() {
        println!(
            "{}. Título: {}, Páginas: {}, Publicação: {}, Gênero: {:?}",
            i + 1,
            livro.titulo,
            livro.numero_paginas,
            livro.data_publicacao,
            livro.genero
        );
    }

    print!("Digite o número do livro que deseja deletar (ou 0 para cancelar): ");
    io::stdout().flush().unwrap();
    let mut escolha = String::new();
    io::stdin().read_line(&mut escolha).unwrap();
    let escolha: usize = match escolha.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada inválida.");
            return Ok(());
        }
    };

    if escolha == 0 {
        println!("Operação de deleção cancelada.");
        return Ok(());
    }

    if escolha > livros.len() {
        println!("Número inválido.");
        return Ok(());
    }

    let livro_removido = livros.remove(escolha - 1);
    println!("Livro removido: {}", livro_removido.titulo);

    salvar_livros(arquivo, &livros)?;

    println!("Lista atualizada salva com sucesso.");
    Ok(())
}

fn main() {
    let arquivo = "livros.bin";

    loop {
        println!("=== Sistema da Biblioteca ===");
        println!("1. Adicionar Livro");
        println!("2. Listar Livros");
        println!("3. Deletar Livro");
        println!("0. Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).unwrap();

        match escolha.trim() {
"1" => {
    let mut titulo = String::new();
    let mut numero_paginas = String::new();
    let mut data_publicacao = String::new();
    let mut genero_opcao = String::new();

    // Entrada para título
    print!("Digite o título: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut titulo).unwrap();

    // Entrada para número de páginas
    print!("Digite o número de páginas: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut numero_paginas).unwrap();
    let numero_paginas: u32 = match numero_paginas.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Número inválido! Tente novamente.");
            continue;
        }
    };

    // Entrada para data de publicação
    print!("Digite a data de publicação (AAAA-MM-DD): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut data_publicacao).unwrap();

    // Menu para escolha do gênero
    println!("Escolha o gênero do livro:");
    println!("1. Ficção");
    println!("2. Biografia");
    println!("3. Poesia");
    println!("4. Infantil");
    println!("5. Romance");
    println!("6. Outro");

    print!("Digite o número correspondente ao gênero: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut genero_opcao).unwrap();

    let genero = match genero_opcao.trim() {
        "1" => Genero::Ficcao,
        "2" => Genero::Biografia,
        "3" => Genero::Poesia,
        "4" => Genero::Infantil,
        "5" => Genero::Romance,
        "6" => Genero::Outro,
        _ => {
            println!("Opção inválida! Tente novamente.");
            continue;
        }
    };

    // Criação do livro
    match Livro::novo(&titulo.trim(), numero_paginas, &data_publicacao.trim(), &format!("{:?}", genero)) {
        Ok(livro) => {
            let mut livros = carregar_livros(arquivo).unwrap_or_else(|_| Vec::new());
            livros.push(livro);
            salvar_livros(arquivo, &livros).unwrap();
            println!("Livro adicionado com sucesso!");
        }
        Err(e) => println!("Erro: {}", e),
    }
}


            "2" => listar_livros(arquivo).unwrap(),
            "3" => deletar_livro(arquivo).unwrap(),
            "0" => break,
            _ => println!("Opção inválida."),
        }
    }
}
