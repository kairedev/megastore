#[derive(Debug, Clone, PartialEq)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

impl Produto {
    pub fn novo(id: u32, nome: &str, categoria: &str, preco: f64) -> Produto {
        Produto {
            id,
            nome: nome.to_string(),
            categoria: categoria.to_string(),
            preco,
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn cria_produto_com_os_campos_corretos() {
        let p = Produto::novo(1, "Mouse", "Periféricos", 50.0);
        assert_eq!(p.id, 1);
        assert_eq!(p.nome, "Mouse");
        assert_eq!(p.categoria, "Periféricos");
        assert_eq!(p.preco, 50.0);
    }
}
