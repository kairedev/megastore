use megastore::benchmark;
use megastore::exemplo::grafo_exemplo;
use megastore::grafo::Grafo;
use megastore::produto::Produto;
use megastore::recomendacao::{produtos_relacionados, recomendar_para_cliente, Recomendacao};
use std::io::{self, Write};


fn ler(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap();
    let mut texto = String::new();
    let bytes = io::stdin().read_line(&mut texto).unwrap();
    if bytes == 0 {
        std::process::exit(0);
    }
    texto.trim().to_string()
}

fn ler_numero(mensagem: &str) -> Option<u32> {
    let n = ler(mensagem).parse::<u32>().ok();
    if n.is_none() {
        println!("Valor inválido. Digite um número inteiro positivo.");
    }
    n
}

fn mostrar_recomendacoes(grafo: &Grafo, recs: &[Recomendacao]) {
    if recs.is_empty() {
        println!("Nenhuma recomendação encontrada.");
        return;
    }
    for (i, r) in recs.iter().enumerate() {
        let p = grafo.buscar_produto(r.produto_id).unwrap();
        println!(
            "{}. [{}] {} - {} - R$ {:.2} (pontuação: {})",
            i + 1, p.id, p.nome, p.categoria, p.preco, r.pontuacao
        );
    }
}

fn menu() {
    println!("\n===== ConectaStore =====");
    println!("1 - Cadastrar produto");
    println!("2 - Consultar produto");
    println!("3 - Listar produtos");
    println!("4 - Cadastrar cliente");
    println!("5 - Registrar compra");
    println!("6 - Recomendar produtos para um cliente");
    println!("7 - Produtos relacionados a um produto");
    println!("8 - Carregar dados de exemplo");
    println!("9 - Teste de desempenho");
    println!("0 - Sair");
}

fn main() {
    // Atalho: `cargo run --release -- benchmark`
    if std::env::args().nth(1).as_deref() == Some("benchmark") {
        benchmark::executar_e_imprimir();
        return;
    }

    let mut grafo = Grafo::novo();

    loop {
        menu();
        match ler("Escolha: ").as_str() {
            "1" => {
                let Some(id) = ler_numero("Id do produto: ") else { continue };
                let nome = ler("Nome: ");
                let categoria = ler("Categoria: ");
                let Ok(preco) = ler("Preço: ").replace(',', ".").parse::<f64>() else {
                    println!("Preço inválido.");
                    continue;
                };
                match grafo.cadastrar_produto(Produto::novo(id, &nome, &categoria, preco)) {
                    Ok(()) => println!("Produto cadastrado!"),
                    Err(e) => println!("Erro: {}", e),
                }
            }
            "2" => {
                let Some(id) = ler_numero("Id do produto: ") else { continue };
                match grafo.buscar_produto(id) {
                    Some(p) => println!("[{}] {} - {} - R$ {:.2}", p.id, p.nome, p.categoria, p.preco),
                    None => println!("Produto não encontrado."),
                }
            }
            "3" => {
                let lista = grafo.listar_produtos();
                if lista.is_empty() {
                    println!("Nenhum produto cadastrado.");
                }
                for p in lista {
                    println!("[{}] {} - {} - R$ {:.2}", p.id, p.nome, p.categoria, p.preco);
                }
            }
            "4" => {
                let Some(id) = ler_numero("Id do cliente: ") else { continue };
                let nome = ler("Nome: ");
                match grafo.cadastrar_cliente(id, &nome) {
                    Ok(()) => println!("Cliente cadastrado!"),
                    Err(e) => println!("Erro: {}", e),
                }
            }
            "5" => {
                let Some(cliente) = ler_numero("Id do cliente: ") else { continue };
                let Some(produto) = ler_numero("Id do produto: ") else { continue };
                let Some(nota) = ler_numero("Nota do cliente para o produto (1 a 5): ") else { continue };
                match grafo.registrar_compra(cliente, produto, u8::try_from(nota).unwrap_or(0)) {
                    Ok(()) => println!("Compra registrada (conexão criada no grafo)!"),
                    Err(e) => println!("Erro: {}", e),
                }
            }
            "6" => {
                let Some(id) = ler_numero("Id do cliente: ") else { continue };
                match recomendar_para_cliente(&grafo, id, 5) {
                    Ok(recs) => {
                        println!("Recomendações para {}:", grafo.buscar_cliente(id).unwrap());
                        mostrar_recomendacoes(&grafo, &recs);
                    }
                    Err(e) => println!("Erro: {}", e),
                }
            }
            "7" => {
                let Some(id) = ler_numero("Id do produto: ") else { continue };
                match produtos_relacionados(&grafo, id, 5) {
                    Ok(recs) => {
                        println!("Quem comprou este produto também comprou:");
                        mostrar_recomendacoes(&grafo, &recs);
                    }
                    Err(e) => println!("Erro: {}", e),
                }
            }
            "8" => {
                grafo = grafo_exemplo();
                println!(
                    "Dados de exemplo carregados: {} clientes, {} produtos, {} compras.",
                    grafo.total_clientes(),
                    grafo.total_produtos(),
                    grafo.total_compras()
                );
            }
            "9" => benchmark::executar_e_imprimir(),
            "0" => {
                println!("Até logo!");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }
}
