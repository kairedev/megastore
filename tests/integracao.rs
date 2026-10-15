use megastore::benchmark::gerar_grafo;
use megastore::exemplo::grafo_exemplo;
use megastore::grafo::{Erro, Grafo};
use megastore::produto::Produto;
use megastore::recomendacao::{
    produtos_relacionados, recomendar_para_cliente, recomendar_para_cliente_dfs,
};
use std::collections::HashSet;

#[test]
fn fluxo_completo_cadastro_compra_recomendacao() {
    let mut g = Grafo::novo();
    g.cadastrar_produto(Produto::novo(1, "Caneta", "Papelaria", 3.0)).unwrap();
    g.cadastrar_produto(Produto::novo(2, "Caderno", "Papelaria", 15.0)).unwrap();
    g.cadastrar_produto(Produto::novo(3, "Mochila", "Papelaria", 90.0)).unwrap();
    g.cadastrar_cliente(1, "Lia").unwrap();
    g.cadastrar_cliente(2, "Davi").unwrap();

    g.registrar_compra(1, 1, 5).unwrap();
    g.registrar_compra(2, 1, 4).unwrap();
    g.registrar_compra(2, 2, 5).unwrap();
    g.registrar_compra(2, 3, 3).unwrap();

    let recs = recomendar_para_cliente(&g, 1, 10).unwrap();
    let ids: Vec<u32> = recs.iter().map(|r| r.produto_id).collect();
    assert_eq!(ids, vec![2, 3]);
}

#[test]
fn recomendacao_do_exemplo_nunca_inclui_produto_isolado() {

    let g = grafo_exemplo();
    let recs = recomendar_para_cliente(&g, 1, 10).unwrap();
    assert!(recs.iter().all(|r| r.produto_id != 5));
}

#[test]
fn nota_baixa_nao_vira_recomendacao() {
    // Carla deu nota 2 ao Monitor: ele não pode ser recomendado à Ana.
    let g = grafo_exemplo();
    let recs = recomendar_para_cliente(&g, 1, 10).unwrap();
    assert!(recs.iter().all(|r| r.produto_id != 4));
}

#[test]
fn erros_sao_retornados_corretamente() {
    let mut g = grafo_exemplo();
    assert_eq!(g.registrar_compra(999, 1, 3), Err(Erro::ClienteNaoEncontrado(999)));
    assert_eq!(g.registrar_compra(1, 999, 3), Err(Erro::ProdutoNaoEncontrado(999)));
    assert!(matches!(g.registrar_compra(1, 1, 3), Err(Erro::CompraJaRegistrada { .. })));
    assert_eq!(g.registrar_compra(1, 2, 9), Err(Erro::NotaInvalida(9)));
}

#[test]
fn nova_compra_muda_a_recomendacao() {
    let mut g = grafo_exemplo();

    g.registrar_compra(4, 6, 4).unwrap();
    let recs = recomendar_para_cliente(&g, 1, 10).unwrap();
    assert!(recs.iter().any(|r| r.produto_id == 5));
}

#[test]
fn produtos_relacionados_funcionam_com_dados_de_exemplo() {
    let g = grafo_exemplo();
    let recs = produtos_relacionados(&g, 3, 10).unwrap();
    assert!(!recs.is_empty());
    assert!(recs.iter().all(|r| r.produto_id != 3));
}

#[test]
fn volume_grande_sem_duplicatas_nem_produtos_ja_comprados() {
    let g = gerar_grafo(5_000, 1_000, 5, 99);
    for cliente in [1u32, 100, 2_500, 5_000] {
        let recs = recomendar_para_cliente(&g, cliente, 20).unwrap();
        let ids: HashSet<u32> = recs.iter().map(|r| r.produto_id).collect();
        assert_eq!(ids.len(), recs.len(), "há recomendações duplicadas");
        let comprados = g.produtos_do_cliente(cliente).unwrap();
        assert!(recs.iter().all(|r| !comprados.contains_key(&r.produto_id)));
        assert!(recs.len() <= 20);
    }
}

#[test]
fn recomendacoes_saem_ordenadas_por_pontuacao() {
    let g = gerar_grafo(2_000, 300, 6, 5);
    let recs = recomendar_para_cliente(&g, 10, 50).unwrap();
    for par in recs.windows(2) {
        assert!(par[0].pontuacao >= par[1].pontuacao);
    }
}

#[test]
fn bfs_e_dfs_concordam_em_volume_grande() {
    let g = gerar_grafo(3_000, 500, 5, 11);
    for cliente in [1u32, 50, 1_000, 2_999] {
        assert_eq!(
            recomendar_para_cliente(&g, cliente, 10).unwrap(),
            recomendar_para_cliente_dfs(&g, cliente, 10).unwrap()
        );
    }
}