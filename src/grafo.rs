use crate::produto::Produto;
use std::collections::HashMap;
use std::fmt;


#[derive(Debug, PartialEq)]
pub enum Erro {
    ProdutoJaExiste(u32),
    ProdutoNaoEncontrado(u32),
    ClienteJaExiste(u32),
    ClienteNaoEncontrado(u32),
    CompraJaRegistrada { cliente: u32, produto: u32 },
    NotaInvalida(u8),
}

impl fmt::Display for Erro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Erro::ProdutoJaExiste(id) => write!(f, "Já existe um produto com id {}", id),
            Erro::ProdutoNaoEncontrado(id) => write!(f, "Produto {} não encontrado", id),
            Erro::ClienteJaExiste(id) => write!(f, "Já existe um cliente com id {}", id),
            Erro::ClienteNaoEncontrado(id) => write!(f, "Cliente {} não encontrado", id),
            Erro::NotaInvalida(n) => write!(f, "Nota {} inválida (use de 1 a 5)", n),
            Erro::CompraJaRegistrada { cliente, produto } => write!(
                f,
                "Cliente {} já comprou o produto {} (conexão já existe)",
                cliente, produto
            ),
        }
    }
}


pub struct Grafo {
    produtos: HashMap<u32, Produto>,
    clientes: HashMap<u32, String>,
    compras_do_cliente: HashMap<u32, HashMap<u32, u8>>,
    compradores_do_produto: HashMap<u32, HashMap<u32, u8>>,
    total_arestas: usize,
}

impl Grafo {
    pub fn novo() -> Grafo {
        Grafo {
            produtos: HashMap::new(),
            clientes: HashMap::new(),
            compras_do_cliente: HashMap::new(),
            compradores_do_produto: HashMap::new(),
            total_arestas: 0,
        }
    }




    pub fn cadastrar_produto(&mut self, produto: Produto) -> Result<(), Erro> {
        if self.produtos.contains_key(&produto.id) {
            return Err(Erro::ProdutoJaExiste(produto.id));
        }
        self.compradores_do_produto.insert(produto.id, HashMap::new());
        self.produtos.insert(produto.id, produto);
        Ok(())
    }


    pub fn buscar_produto(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }


    pub fn listar_produtos(&self) -> Vec<&Produto> {
        let mut lista: Vec<&Produto> = self.produtos.values().collect();
        lista.sort_by_key(|p| p.id);
        lista
    }




    pub fn cadastrar_cliente(&mut self, id: u32, nome: &str) -> Result<(), Erro> {
        if self.clientes.contains_key(&id) {
            return Err(Erro::ClienteJaExiste(id));
        }
        self.clientes.insert(id, nome.to_string());
        self.compras_do_cliente.insert(id, HashMap::new());
        Ok(())
    }

    pub fn buscar_cliente(&self, id: u32) -> Option<&String> {
        self.clientes.get(&id)
    }


    pub fn registrar_compra(&mut self, cliente: u32, produto: u32, nota: u8) -> Result<(), Erro> {
        if !(1..=5).contains(&nota) {
            return Err(Erro::NotaInvalida(nota));
        }
        if !self.clientes.contains_key(&cliente) {
            return Err(Erro::ClienteNaoEncontrado(cliente));
        }
        if !self.produtos.contains_key(&produto) {
            return Err(Erro::ProdutoNaoEncontrado(produto));
        }
        let compras = self.compras_do_cliente.get_mut(&cliente).unwrap();
        if compras.contains_key(&produto) {
            return Err(Erro::CompraJaRegistrada { cliente, produto });
        }
        compras.insert(produto, nota);
        self.compradores_do_produto
            .get_mut(&produto)
            .unwrap()
            .insert(cliente, nota);
        self.total_arestas += 1;
        Ok(())
    }


    pub fn produtos_do_cliente(&self, cliente: u32) -> Option<&HashMap<u32, u8>> {
        self.compras_do_cliente.get(&cliente)
    }


    pub fn compradores_do_produto(&self, produto: u32) -> Option<&HashMap<u32, u8>> {
        self.compradores_do_produto.get(&produto)
    }



    pub fn total_produtos(&self) -> usize {
        self.produtos.len()
    }
    pub fn total_clientes(&self) -> usize {
        self.clientes.len()
    }
    pub fn total_compras(&self) -> usize {
        self.total_arestas
    }
    pub fn ids_clientes(&self) -> Vec<u32> {
        let mut ids: Vec<u32> = self.clientes.keys().copied().collect();
        ids.sort();
        ids
    }
}

impl Default for Grafo {
    fn default() -> Self {
        Grafo::novo()
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn grafo_basico() -> Grafo {
        let mut g = Grafo::novo();
        g.cadastrar_produto(Produto::novo(1, "Mouse", "Periféricos", 50.0)).unwrap();
        g.cadastrar_produto(Produto::novo(2, "Teclado", "Periféricos", 120.0)).unwrap();
        g.cadastrar_cliente(10, "Ana").unwrap();
        g
    }

    #[test]
    fn cadastra_e_consulta_produto() {
        let g = grafo_basico();
        assert_eq!(g.buscar_produto(1).unwrap().nome, "Mouse");
        assert!(g.buscar_produto(99).is_none());
        assert_eq!(g.total_produtos(), 2);
    }

    #[test]
    fn recusa_produto_duplicado() {
        let mut g = grafo_basico();
        let r = g.cadastrar_produto(Produto::novo(1, "Outro", "X", 1.0));
        assert_eq!(r, Err(Erro::ProdutoJaExiste(1)));
    }

    #[test]
    fn recusa_cliente_duplicado() {
        let mut g = grafo_basico();
        assert_eq!(g.cadastrar_cliente(10, "Ana 2"), Err(Erro::ClienteJaExiste(10)));
    }

    #[test]
    fn registra_compra_nos_dois_sentidos_com_a_nota() {
        let mut g = grafo_basico();
        g.registrar_compra(10, 1, 4).unwrap();
        assert_eq!(g.produtos_do_cliente(10).unwrap().get(&1), Some(&4));
        assert_eq!(g.compradores_do_produto(1).unwrap().get(&10), Some(&4));
        assert_eq!(g.total_compras(), 1);
    }

    #[test]
    fn recusa_compra_duplicada() {
        let mut g = grafo_basico();
        g.registrar_compra(10, 1, 4).unwrap();
        let r = g.registrar_compra(10, 1, 5);
        assert_eq!(r, Err(Erro::CompraJaRegistrada { cliente: 10, produto: 1 }));
        assert_eq!(g.total_compras(), 1);
        // a nota original não é alterada
        assert_eq!(g.produtos_do_cliente(10).unwrap().get(&1), Some(&4));
    }

    #[test]
    fn recusa_compra_com_vertice_inexistente() {
        let mut g = grafo_basico();
        assert_eq!(g.registrar_compra(99, 1, 3), Err(Erro::ClienteNaoEncontrado(99)));
        assert_eq!(g.registrar_compra(10, 99, 3), Err(Erro::ProdutoNaoEncontrado(99)));
    }

    #[test]
    fn recusa_nota_fora_do_intervalo() {
        let mut g = grafo_basico();
        assert_eq!(g.registrar_compra(10, 1, 0), Err(Erro::NotaInvalida(0)));
        assert_eq!(g.registrar_compra(10, 1, 6), Err(Erro::NotaInvalida(6)));
        assert_eq!(g.total_compras(), 0);
    }
}
