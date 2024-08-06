Gerando Números Aleatórios em Rust com a Biblioteca "rand"
Entendendo o Problema:

Queremos criar um programa em Rust que:

Utilize a biblioteca externa "rand" para gerar números aleatórios.
Gere 10 números aleatórios entre 0 e 100.
Imprima esses números no console.

Explicação:

Importando a biblioteca:

use rand::Rng;: Importamos o trait Rng da biblioteca rand, que nos fornece métodos para gerar números aleatórios.
Criando um gerador de números aleatórios:

let mut rng = rand::thread_rng();: Criamos um gerador de números aleatórios associado à thread atual. O mut é necessário porque o gerador precisa ser mutável para gerar números sequencialmente.
Gerando e imprimindo os números:

for _ in 0..10: Iteramos 10 vezes para gerar 10 números.
let random_number: u32 = rng.gen_range(0..=100);: Geramos um número aleatório entre 0 e 100 (inclusive) e o armazenamos na variável random_number. O tipo u32 indica que estamos gerando um número inteiro sem sinal de 32 bits.
println!("{}", random_number);: Imprimimos o número gerado no console.
Como executar:

Criar um novo projeto Rust: Se você ainda não tiver um projeto Rust, crie um novo diretório para o seu projeto e execute o seguinte comando no terminal:
Bash
cargo new random_numbers
Use o código com cuidado.

Criar o arquivo main.rs: Crie um arquivo chamado main.rs dentro do diretório do seu projeto e cole o código acima nele.
Executar o programa:
Bash
cd random_numbers
cargo run
Use o código com cuidado.

Observações:

Biblioteca "rand": A biblioteca "rand" é uma biblioteca padrão para gerar números aleatórios em Rust. Ela oferece diversas funcionalidades para gerar números aleatórios de diferentes distribuições.
Rng trait: O trait Rng define métodos para gerar números aleatórios de vários tipos. O método gen_range é usado para gerar números aleatórios dentro de um intervalo específico.
Semente: A qualidade da aleatoriedade depende da semente utilizada para inicializar o gerador de números aleatórios. Por padrão, o rand::thread_rng() utiliza uma semente baseada no tempo, o que garante uma boa aleatoriedade para a maioria dos casos.
Outros tipos de números: Você pode gerar números aleatórios de outros tipos, como f64 para números de ponto flutuante, ajustando o tipo da variável random_number.
Com este código, você terá um programa simples e eficiente para gerar números aleatórios em Rust.
