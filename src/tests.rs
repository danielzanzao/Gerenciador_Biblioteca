#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn test_adicionar_livro() {
        let mut biblioteca = Biblioteca::new();
        biblioteca.adicionar_livro("Livro 1".into(), 100, "2025-01-01".into(), Genero::Ficcao);
        assert_eq!(biblioteca.listar_livros().len(), 1);
    }
}
