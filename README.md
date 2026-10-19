# 2do

Uma lista de tarefas rápida que vive no terminal.

## Instalação

Você precisa do Rust instalado. Dentro da pasta do projeto:

```sh
cargo install --path .
```

Depois, execute:

```sh
2do
```

## Atalhos

| Tecla | Ação |
|---|---|
| `j` / `↓` | Selecionar a próxima tarefa |
| `k` / `↑` | Selecionar a tarefa anterior |
| `g` / `G` | Ir para a primeira / última tarefa |
| `a` | Adicionar uma tarefa |
| `r` | Renomear a tarefa selecionada |
| `J` / `K` | Mover a tarefa para baixo / cima na prioridade |
| `dd` | Apagar a tarefa selecionada |
| `Espaço` | Marcar como concluída ou reabrir |
| `?` | Mostrar ajuda |
| `q` | Salvar e sair |

As tarefas são salvas automaticamente em `~/.local/share/2do/tasks.json`.
Para escolher outro arquivo, defina a variável `TODO_TUI_DATA`.
