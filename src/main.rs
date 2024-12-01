mod utils;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use utils::{validar_string, validar_numero, validar_data, validar_obrigatorio};
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
        validar_string(titulo, 100)?;
        validar_numero(numero_paginas, 1, 2000)?;
        validar_data(data_publicacao)?; // Validação detalhada da data
        validar_obrigatorio(titulo)?;
        validar_obrigatorio(genero)?;

        let genero_enum = genero.parse::<Genero>()?;

        Ok(Livro {
            titulo: titulo.to_string(),
            numero_paginas,
            data_publicacao: validar_data(data_publicacao)?, // Garantia de formato e validade
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

fn alterar_livro(arquivo: &str) -> io::Result<()> {
    let mut livros = carregar_livros(arquivo)?;

    if livros.is_empty() {
        println!("Não há livros cadastrados para alterar.");
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

    print!("Digite o número do livro que deseja alterar (ou 0 para cancelar): ");
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
        println!("Operação de alteração cancelada.");
        return Ok(());
    }

    if escolha > livros.len() {
        println!("Número inválido.");
        return Ok(());
    }

    let livro = &mut livros[escolha - 1];

    println!("=== Alterando informações do livro: {} ===", livro.titulo);

    // Alterar título
    let mut novo_titulo = String::new();
    print!("Novo título (ou pressione Enter para manter '{}'): ", livro.titulo);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut novo_titulo).unwrap();
    if !novo_titulo.trim().is_empty() {
        livro.titulo = novo_titulo.trim().to_string();
    }

    // Alterar número de páginas
    let mut novo_numero_paginas = String::new();
    print!(
        "Novo número de páginas (ou pressione Enter para manter '{}'): ",
        livro.numero_paginas
    );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut novo_numero_paginas).unwrap();
    if !novo_numero_paginas.trim().is_empty() {
        match novo_numero_paginas.trim().parse::<u32>() {
            Ok(num) => livro.numero_paginas = num,
            Err(_) => println!("Número inválido. Mantendo o valor atual."),
        }
    }

    // Alterar data de publicação
    let mut nova_data_publicacao = String::new();
    print!(
        "Nova data de publicação (AAAA-MM-DD) (ou pressione Enter para manter '{}'): ",
        livro.data_publicacao
    );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nova_data_publicacao).unwrap();
    if !nova_data_publicacao.trim().is_empty() {
        match NaiveDate::parse_from_str(nova_data_publicacao.trim(), "%Y-%m-%d") {
            Ok(data) => livro.data_publicacao = data,
            Err(_) => println!("Data inválida. Mantendo o valor atual."),
        }
    }

    // Alterar gênero
    println!("Escolha o novo gênero do livro:");
    println!("1. Ficção");
    println!("2. Biografia");
    println!("3. Poesia");
    println!("4. Infantil");
    println!("5. Romance");
    println!("6. Outro");
    print!("Digite o número correspondente ao novo gênero (ou pressione Enter para manter '{:?}'): ", livro.genero);
    io::stdout().flush().unwrap();
    let mut nova_opcao_genero = String::new();
    io::stdin().read_line(&mut nova_opcao_genero).unwrap();
    if !nova_opcao_genero.trim().is_empty() {
        match nova_opcao_genero.trim() {
            "1" => livro.genero = Genero::Ficcao,
            "2" => livro.genero = Genero::Biografia,
            "3" => livro.genero = Genero::Poesia,
            "4" => livro.genero = Genero::Infantil,
            "5" => livro.genero = Genero::Romance,
            "6" => livro.genero = Genero::Outro,
            _ => println!("Opção inválida. Mantendo o gênero atual."),
        }
    }

    salvar_livros(arquivo, &livros)?;
    println!("Livro alterado com sucesso!");
    Ok(())
}


fn main() {
    let arquivo = "livros.bin";

    loop {
        println!("=== Sistema da Biblioteca ===");
        println!("1. Adicionar Livro");
        println!("2. Listar Livros");
        println!("3. Deletar Livro");
        println!("4. Alterar Livro");
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

    print!("Digite o título: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut titulo).unwrap();
    if let Err(err) = validar_string(titulo.trim(), 100) {
        println!("{}", err);
    continue;
    }


    print!("Digite o número de páginas (máx 2000): ");
io::stdout().flush().unwrap();
io::stdin().read_line(&mut numero_paginas).unwrap();
let numero_paginas: u32 = match numero_paginas.trim().parse() {
    Ok(num) if validar_numero(num, 1, 2000).is_ok() => num,
        _ => {
            println!("Número inválido! Deve estar entre 1 e 2000.");
            continue;
        }
    };


    print!("Digite a data de publicação (AAAA-MM-DD): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut data_publicacao).unwrap();
    if let Err(err) = validar_data(data_publicacao.trim()) {
        println!("{}", err);
        continue;
    }


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

    // Criação 
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
            "4" => alterar_livro(arquivo).unwrap(),
            "0" => break,
            _ => println!("Opção inválida."),
        }
    }
}
