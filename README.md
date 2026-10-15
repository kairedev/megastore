# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

Projeto acadêmico (Estruturas de Dados) desenvolvido em **Rust puro**, sem nenhuma crate externa.

* **Aluno:** *Kairê Gustavo de Oliveira  RA 186339*
* **Curso:** Análise e Desenvolvimento de Sistemas — UniFECAF
* **Vídeo pitch:** https://youtu.be/7r96ltPw7y8

## 1\. Objetivo

A MegaStore recomenda produtos apenas por categoria ou popularidade. O ConectaStore usa as **relações entre clientes e produtos** para recomendar: se dois clientes compraram produtos em comum, o que um comprou pode interessar ao outro ("quem comprou isso também comprou aquilo").

## 2\. Como funciona

O sistema modela a loja como um **grafo**:

* **Vértices:** clientes e produtos.
* **Arestas:** uma compra liga um cliente a um produto.
* **Peso da aresta:** a **nota (1 a 5)** que o cliente deu ao produto.
* **Tipo:** grafo **bipartido, não direcionado e ponderado** (arestas só ligam cliente ↔ produto).

Para recomendar produtos a um cliente, uma **BFS (busca em largura)** anda 3 níveis:

```
Nível 0: Cliente escolhido
Nível 1: produtos que ele já comprou
Nível 2: outros clientes que compraram esses produtos (clientes parecidos)
Nível 3: produtos desses clientes parecidos  -> candidatos à recomendação
```

A **pontuação** de cada candidato é a **soma das notas** que os clientes parecidos deram a ele. Só notas **3 ou mais** contam (nota baixa = o cliente não gostou, então não é recomendado). Produtos que o cliente já tem são descartados. O resultado sai ordenado por pontuação.

O projeto também traz uma versão com **DFS** (`recomendar\_para\_cliente\_dfs`) para comparação: a única diferença é usar **pilha** em vez de **fila**. Como a busca é limitada a 3 níveis, o resultado é igual, e os testes confirmam isso. Escolhemos a BFS porque ela processa os níveis em ordem, que é exatamente o que o algoritmo precisa.

## 3\. Tecnologias

* Rust (edição 2021) e Cargo
* Somente a biblioteca padrão (`std`). Nenhuma dependência externa.

## 4\. Estruturas de dados utilizadas

|Estrutura|Onde|Por quê|
|-|-|-|
|`HashMap<u32, Produto>`|cadastro de produtos|consulta por id em O(1) médio|
|`HashMap<u32, String>`|cadastro de clientes|consulta por id em O(1) médio|
|`HashMap<u32, HashMap<u32, u8>>` (x2)|**lista de adjacência ponderada** (cliente→{produto: nota} e produto→{cliente: nota})|percorrer vizinhos nos dois sentidos, guardar a nota (peso) e impedir compras duplicadas em O(1)|
|`VecDeque`|fila da BFS|inserir no fim e remover do início em O(1)|
|`Vec` (como pilha)|pilha da DFS|`push` e `pop` no fim em O(1)|
|`HashSet<Vertice>`|vértices visitados|cada vértice entra na fila uma única vez|
|`HashMap<u32, u32>`|pontuação dos candidatos|uma entrada por produto = **sem recomendações duplicadas**|
|`Vec`|resultado final|ordenação e limite (top N)|

**Por que lista e não matriz de adjacência?** O grafo é esparso (cada cliente compra poucos produtos entre milhões). A lista ocupa O(V + E); a matriz ocuparia O(V²). Detalhes e números no relatório técnico.

## 5\. Arquitetura

```
megastore/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs         # menu no terminal
│   ├── lib.rs          # declara os módulos (permite os testes de integração)
│   ├── produto.rs      # struct Produto
│   ├── grafo.rs        # grafo, cadastros e conexões (compras)
│   ├── recomendacao.rs # BFS e DFS de recomendação, produtos relacionados
│   ├── exemplo.rs      # dados pequenos para demonstração/testes
│   └── benchmark.rs    # gerador de dados e medição de tempo
└── tests/
    └── integracao.rs   # testes de integração
```

## 6\. Como compilar

Pré-requisito: Rust instalado (https://rustup.rs). Confira com `rustc --version`.

```bash
cd megastore
cargo build --release
```

## 7\. Como executar

```bash
cargo run --release
```

Abre o menu:

```
1 - Cadastrar produto          6 - Recomendar produtos para um cliente
2 - Consultar produto          7 - Produtos relacionados a um produto
3 - Listar produtos            8 - Carregar dados de exemplo
4 - Cadastrar cliente          9 - Teste de desempenho
5 - Registrar compra           0 - Sair
```

Teste de desempenho direto, sem menu:

```bash
cargo run --release -- benchmark
```

## 8\. Como rodar os testes

```bash
cargo test
```

São **29 testes**: 20 unitários (dentro de `src/`) e 9 de integração (`tests/integracao.rs`). Cobrem cadastro, consulta, conexões, notas, erros, prevenção de duplicatas, ordenação, volume grande e a igualdade entre BFS e DFS.

## 9\. Exemplo de uso

Dados de exemplo (opção 8), com a nota de cada compra:

|Cliente|Compras (produto: nota)|
|-|-|
|Ana (1)|Mouse (1): 5, Mousepad (6): 4|
|Bruno (2)|Mouse (1): 4, Teclado (2): 5, Headset (3): 4|
|Carla (3)|Mouse (1): 5, Headset (3): 5, Monitor (4): 2|
|Diego (4)|Livro de Rust (5): 5|
|Elisa (5)|Teclado (2): 4, Monitor (4): 5|

Opção 6, cliente 1 (Ana):

```
Recomendações para Ana:
1. \[3] Headset - Áudio - R$ 179.90 (pontuação: 9)
2. \[2] Teclado Mecânico - Periféricos - R$ 249.90 (pontuação: 5)
```

O Headset lidera porque **Bruno (nota 4) e Carla (nota 5)**, que compraram o mesmo Mouse que a Ana, também compraram o Headset: 4 + 5 = 9. O Teclado vem só do Bruno (nota 5). O **Monitor não aparece**, porque a Carla deu nota 2 (abaixo de 3). O Livro de Rust também não: Diego não tem ligação com a Ana.

Opção 5 com compra repetida ou nota inválida:

```
Erro: Cliente 1 já comprou o produto 1 (conexão já existe)
Erro: Nota 9 inválida (use de 1 a 5)
```

## 10\. Resultados dos testes de desempenho

Compilado com `--release`. Cada cenário usa 5 compras por cliente, com notas e produtos pseudoaleatórios (semente fixa). Os tempos de recomendação são a **média de 100 consultas** (top 5), medidas separadamente para BFS e DFS.

|Clientes|Produtos|Compras (arestas)|Construção do grafo|BFS (por consulta)|DFS (por consulta)|
|-:|-:|-:|-:|-:|-:|
|1.000|500|4.977|1,1 ms|16,7 µs|13,8 µs|
|10.000|2.000|49.951|11,2 ms|36,4 µs|38,2 µs|
|100.000|10.000|499.891|117,2 ms|106,1 µs|97,1 µs|
|500.000|50.000|2.499.886|1.038,9 ms|126,1 µs|116,5 µs|

Medido neste PC, com Windows, pelo comando `cargo run --release -- benchmark`. Entre uma rodada e outra há variação de 10% a 20%.

**Leitura dos resultados:**

* Os dados cresceram 500 vezes (de \~5 mil para \~2,5 milhões de compras), mas o tempo de **uma recomendação** cresceu só cerca de 8 vezes. A busca explora apenas a vizinhança do cliente (3 níveis), e não o grafo inteiro.
* A **construção** cresce de forma aproximadamente linear com o número de compras.
* **BFS e DFS têm tempos praticamente iguais** (as diferenças ficam dentro da variação entre rodadas). Isso é esperado: com profundidade limitada, as duas visitam exatamente os mesmos vértices, e só muda se o container é fila ou pilha.

## 11\. Limitações

* Os dados ficam só na memória (sem banco de dados, de propósito).
* Produtos muito populares têm muitos compradores e deixam a busca mais lenta.
* A nota mínima (3) é uma constante fixa no código; um sistema real poderia ajustá-la por categoria.

