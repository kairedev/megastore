use crate::grafo::Grafo;
use crate::produto::Produto;


pub fn grafo_exemplo() -> Grafo {
    let mut g = Grafo::novo();

    let produtos = [
        (1, "Mouse Gamer", "Periféricos", 89.90),
        (2, "Teclado Mecânico", "Periféricos", 249.90),
        (3, "Headset", "Áudio", 179.90),
        (4, "Monitor 24 pol", "Monitores", 799.00),
        (5, "Livro de Rust", "Livros", 99.90),
        (6, "Mousepad", "Periféricos", 29.90),
    ];
    for (id, nome, cat, preco) in produtos {
        g.cadastrar_produto(Produto::novo(id, nome, cat, preco)).unwrap();
    }

    let clientes = [(1, "Ana"), (2, "Bruno"), (3, "Carla"), (4, "Diego"), (5, "Elisa")];
    for (id, nome) in clientes {
        g.cadastrar_cliente(id, nome).unwrap();
    }

    
    let compras = [
        (1, 1, 5), (1, 6, 4),
        (2, 1, 4), (2, 2, 5), (2, 3, 4),
        (3, 1, 5), (3, 3, 5), (3, 4, 2),
        (4, 5, 5),
        (5, 2, 4), (5, 4, 5),
    ];
    for (cliente, produto, nota) in compras {
        g.registrar_compra(cliente, produto, nota).unwrap();
    }
    g
}
