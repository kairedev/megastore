use crate::grafo::{Erro, Grafo};
use std::collections::{HashMap, HashSet, VecDeque};


pub const NOTA_MINIMA: u8 = 3;


#[derive(Debug, Clone, PartialEq)]
pub struct Recomendacao {
    pub produto_id: u32,
    pub pontuacao: u32,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Vertice {
    Cliente(u32),
    Produto(u32),
}


fn expandir(
    grafo: &Grafo,
    vertice: Vertice,
    nivel: u32,
    ja_comprados: &HashMap<u32, u8>,
    visitados: &mut HashSet<Vertice>,
    pontos: &mut HashMap<u32, u32>,
) -> Vec<(Vertice, u32)> {
    let mut novos = Vec::new();
    match vertice {
        Vertice::Cliente(c) => {
            if let Some(compras) = grafo.produtos_do_cliente(c) {
                for (&p, &nota) in compras {
                    if nivel == 0 {
                        // cliente inicial -> seus produtos (nível 1)
                        if visitados.insert(Vertice::Produto(p)) {
                            novos.push((Vertice::Produto(p), 1));
                        }
                    } else if nota >= NOTA_MINIMA && !ja_comprados.contains_key(&p) {

                        *pontos.entry(p).or_insert(0) += nota as u32;
                    }
                }
            }
        }
        Vertice::Produto(p) => {

            if let Some(compradores) = grafo.compradores_do_produto(p) {
                for &c in compradores.keys() {
                    if visitados.insert(Vertice::Cliente(c)) {
                        novos.push((Vertice::Cliente(c), 2));
                    }
                }
            }
        }
    }
    novos
}


pub fn recomendar_para_cliente(
    grafo: &Grafo,
    cliente_id: u32,
    limite: usize,
) -> Result<Vec<Recomendacao>, Erro> {
    let ja_comprados = grafo
        .produtos_do_cliente(cliente_id)
        .ok_or(Erro::ClienteNaoEncontrado(cliente_id))?;

    let mut fila: VecDeque<(Vertice, u32)> = VecDeque::new();
    let mut visitados: HashSet<Vertice> = HashSet::new();
    let mut pontos: HashMap<u32, u32> = HashMap::new();

    let inicio = Vertice::Cliente(cliente_id);
    fila.push_back((inicio, 0));
    visitados.insert(inicio);

    while let Some((vertice, nivel)) = fila.pop_front() {
        let novos = expandir(grafo, vertice, nivel, ja_comprados, &mut visitados, &mut pontos);
        for item in novos {
            fila.push_back(item);
        }
    }

    Ok(ordenar(pontos, limite))
}


pub fn recomendar_para_cliente_dfs(
    grafo: &Grafo,
    cliente_id: u32,
    limite: usize,
) -> Result<Vec<Recomendacao>, Erro> {
    let ja_comprados = grafo
        .produtos_do_cliente(cliente_id)
        .ok_or(Erro::ClienteNaoEncontrado(cliente_id))?;

    let mut pilha: Vec<(Vertice, u32)> = Vec::new();
    let mut visitados: HashSet<Vertice> = HashSet::new();
    let mut pontos: HashMap<u32, u32> = HashMap::new();

    let inicio = Vertice::Cliente(cliente_id);
    pilha.push((inicio, 0));
    visitados.insert(inicio);

    while let Some((vertice, nivel)) = pilha.pop() {
        let novos = expandir(grafo, vertice, nivel, ja_comprados, &mut visitados, &mut pontos);
        for item in novos {
            pilha.push(item);
        }
    }

    Ok(ordenar(pontos, limite))
}


pub fn produtos_relacionados(
    grafo: &Grafo,
    produto_id: u32,
    limite: usize,
) -> Result<Vec<Recomendacao>, Erro> {
    let compradores = grafo
        .compradores_do_produto(produto_id)
        .ok_or(Erro::ProdutoNaoEncontrado(produto_id))?;

    let mut pontos: HashMap<u32, u32> = HashMap::new();
    for &c in compradores.keys() {
        if let Some(produtos) = grafo.produtos_do_cliente(c) {
            for (&p, &nota) in produtos {
                if p != produto_id && nota >= NOTA_MINIMA {
                    *pontos.entry(p).or_insert(0) += nota as u32;
                }
            }
        }
    }
    Ok(ordenar(pontos, limite))
}


fn ordenar(pontos: HashMap<u32, u32>, limite: usize) -> Vec<Recomendacao> {
    let mut lista: Vec<Recomendacao> = pontos
        .into_iter()
        .map(|(produto_id, pontuacao)| Recomendacao { produto_id, pontuacao })
        .collect();
    lista.sort_by(|a, b| {
        b.pontuacao
            .cmp(&a.pontuacao)
            .then(a.produto_id.cmp(&b.produto_id))
    });
    lista.truncate(limite);
    lista
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::exemplo::grafo_exemplo;

    #[test]
    fn recomenda_para_ana_ponderando_pelas_notas() {
        let g = grafo_exemplo();
        let r = recomendar_para_cliente(&g, 1, 10).unwrap();

        assert_eq!(
            r,
            vec![
                Recomendacao { produto_id: 3, pontuacao: 9 },
                Recomendacao { produto_id: 2, pontuacao: 5 },
            ]
        );
    }

    #[test]
    fn nota_baixa_nao_gera_recomendacao() {
        let g = grafo_exemplo();
        let r = recomendar_para_cliente(&g, 1, 10).unwrap();
        assert!(r.iter().all(|x| x.produto_id != 4));
    }

    #[test]
    fn nao_recomenda_o_que_o_cliente_ja_comprou() {
        let g = grafo_exemplo();
        let r = recomendar_para_cliente(&g, 1, 10).unwrap();
        assert!(r.iter().all(|x| x.produto_id != 1 && x.produto_id != 6));
    }

    #[test]
    fn nao_gera_recomendacoes_duplicadas() {
        let g = grafo_exemplo();
        let r = recomendar_para_cliente(&g, 1, 10).unwrap();
        let ids: HashSet<u32> = r.iter().map(|x| x.produto_id).collect();
        assert_eq!(ids.len(), r.len());
    }

    #[test]
    fn respeita_o_limite() {
        let g = grafo_exemplo();
        let r = recomendar_para_cliente(&g, 1, 1).unwrap();
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].produto_id, 3);
    }

    #[test]
    fn cliente_inexistente_gera_erro() {
        let g = grafo_exemplo();
        assert_eq!(
            recomendar_para_cliente(&g, 999, 5),
            Err(Erro::ClienteNaoEncontrado(999))
        );
        assert_eq!(
            recomendar_para_cliente_dfs(&g, 999, 5),
            Err(Erro::ClienteNaoEncontrado(999))
        );
    }

    #[test]
    fn cliente_sem_compras_nao_recebe_recomendacao() {
        let mut g = grafo_exemplo();
        g.cadastrar_cliente(50, "Novo").unwrap();
        assert!(recomendar_para_cliente(&g, 50, 5).unwrap().is_empty());
    }

    #[test]
    fn dfs_e_bfs_dao_o_mesmo_resultado() {
        let g = grafo_exemplo();
        for cliente in 1..=5 {
            assert_eq!(
                recomendar_para_cliente(&g, cliente, 10).unwrap(),
                recomendar_para_cliente_dfs(&g, cliente, 10).unwrap()
            );
        }
    }

    #[test]
    fn produtos_relacionados_ao_mouse() {
        let g = grafo_exemplo();
        let r = produtos_relacionados(&g, 1, 10).unwrap();

        assert_eq!(
            r,
            vec![
                Recomendacao { produto_id: 3, pontuacao: 9 },
                Recomendacao { produto_id: 2, pontuacao: 5 },
                Recomendacao { produto_id: 6, pontuacao: 4 },
            ]
        );
    }

    #[test]
    fn produto_inexistente_gera_erro() {
        let g = grafo_exemplo();
        assert_eq!(
            produtos_relacionados(&g, 999, 5),
            Err(Erro::ProdutoNaoEncontrado(999))
        );
    }
}
