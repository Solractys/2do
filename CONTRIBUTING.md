# Como contribuir

Obrigado por querer melhorar o 2do! Contribuições pequenas ou grandes são bem-vindas.

## Antes de começar

- Para erros, abra um relato de problema.
- Para novas ideias, abra uma sugestão de melhoria.
- Para mudanças maiores, converse conosco em uma issue antes de escrever o código.

## Preparando o projeto

Você precisa ter o Rust instalado.

```sh
git clone git@github.com:Solractys/2do.git
cd 2do
cargo run
```

## Enviando uma mudança

1. Crie uma branch a partir da `main`.
2. Faça uma mudança pequena e focada.
3. Adicione ou ajuste testes quando necessário.
4. Execute as verificações abaixo.
5. Abra um pull request explicando o motivo da mudança.

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Não se preocupe se for sua primeira contribuição. Abra o pull request e peça ajuda.

## Estilo do projeto

- Prefira nomes claros e funções pequenas.
- Preserve os atalhos de teclado já documentados.
- Mantenha a interface legível em terminais pequenos.
- Evite dependências quando a biblioteca padrão resolver bem o problema.

Ao participar, siga nosso [Código de Conduta](CODE_OF_CONDUCT.md).

