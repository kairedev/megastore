use crate::grafo::Grafo;
use crate::produto::Produto;
use crate::recomendacao::{recomendar_para_cliente, recomendar_para_cliente_dfs};
use std::time::{Duration, Instant};

struct Lcg(u64);

impl Lcg {
    fn proximo(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}


pub fn gerar_grafo(
    n_clientes: u32,
    n_produtos: u32,
    compras_por_cliente: u32,
    semente: u64,
) -> Grafo {
    let categorias = ["Eletrônicos", "Livros", "Casa", "Esporte", "Moda"];
    let mut g = Grafo::novo();
    let mut rng = Lcg(semente);

    for id in 1..=n_produtos {
        let cat = categorias[(id as usize) % categorias.len()];
        g.cadastrar_produto(Produto::novo(id, &format!("Produto {}", id), cat, 10.0 + (id % 100) as f64))
            .unwrap();
    }
    for id in 1..=n_clientes {
        g.cadastrar_cliente(id, &format!("Cliente {}", id)).unwrap();
        for _ in 0..compras_por_cliente {
            let produto = (rng.proximo() % n_produtos as u64) as u32 + 1;
            let nota = (rng.proximo() % 5) as u8 + 1; // nota de 1 a 5
            // Se o sorteio repetir um produto, a compra duplicada é ignorada.
            let _ = g.registrar_compra(id, produto, nota);
        }
    }
    g
}


pub struct Medicao {
    pub clientes: u32,
    pub produtos: u32,
    pub compras: usize,
    pub tempo_construcao: Duration,
    pub tempo_bfs: Duration,
    pub tempo_dfs: Duration,
}


pub fn medir(n_clientes: u32, n_produtos: u32, compras_por_cliente: u32) -> Medicao {
    let inicio = Instant::now();
    let g = gerar_grafo(n_clientes, n_produtos, compras_por_cliente, 42);
    let tempo_construcao = inicio.elapsed();

    let amostras = 100u32.min(n_clientes);
    let passo = (n_clientes / amostras).max(1);
    let inicio = Instant::now();
    for i in 0..amostras {
        let cliente = 1 + i * passo;
        let _ = recomendar_para_cliente(&g, cliente, 5);
    }
    let tempo_bfs = inicio.elapsed() / amostras;

    let inicio = Instant::now();
    for i in 0..amostras {
        let cliente = 1 + i * passo;
        let _ = recomendar_para_cliente_dfs(&g, cliente, 5);
    }
    let tempo_dfs = inicio.elapsed() / amostras;

    Medicao {
        clientes: n_clientes,
        produtos: n_produtos,
        compras: g.total_compras(),
        tempo_construcao,
        tempo_bfs,
        tempo_dfs,
    }
}


pub fn executar_e_imprimir() {
    let cenarios = [
        (1_000, 500, 5),
        (10_000, 2_000, 5),
        (100_000, 10_000, 5),
        (500_000, 50_000, 5),
    ];
    println!(
        "{:>10} {:>10} {:>10} {:>16} {:>14} {:>14}",
        "Clientes", "Produtos", "Compras", "Construção (ms)", "BFS (µs)", "DFS (µs)"
    );
    for (c, p, k) in cenarios {
        let m = medir(c, p, k);
        println!(
            "{:>10} {:>10} {:>10} {:>16.1} {:>14.1} {:>14.1}",
            m.clientes,
            m.produtos,
            m.compras,
            m.tempo_construcao.as_secs_f64() * 1000.0,
            m.tempo_bfs.as_secs_f64() * 1_000_000.0,
            m.tempo_dfs.as_secs_f64() * 1_000_000.0
        );
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn gera_grafo_com_os_totais_esperados() {
        let g = gerar_grafo(100, 50, 3, 1);
        assert_eq!(g.total_clientes(), 100);
        assert_eq!(g.total_produtos(), 50);
        assert!(g.total_compras() <= 300);
        assert!(g.total_compras() > 0);
    }

    #[test]
    fn mesma_semente_gera_mesmo_grafo() {
        let a = gerar_grafo(50, 20, 3, 7);
        let b = gerar_grafo(50, 20, 3, 7);
        assert_eq!(a.total_compras(), b.total_compras());
    }
}
