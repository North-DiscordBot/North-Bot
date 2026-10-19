# North

A North é uma bot do Discord feita em Rust, do zero.

O código da North é aberto. A comunicação com o Discord (Gateway, REST, interações) fica na **NortCod**, uma biblioteca da equipe que é **privada**. Por isso, quem não tem acesso à NortCod consegue ler e estudar a North, mas não consegue compilá-la.

## Comandos

| Comando | O que faz |
|---|---|
| `/ping` | Mostra se a North está respondendo e quanto demorou. |
| `/sobre` | Conta quem é a North. |

## Rodar

Precisa de acesso ao repositório `North-DiscordBot/NortCod` e do git configurado com as suas credenciais do GitHub. O `.cargo/config.toml` já manda o Cargo usar o git do sistema.

```bash
DISCORD_TOKEN=... cargo run --release
```

| Variável | |
|---|---|
| `DISCORD_TOKEN` | obrigatória |
| `NORTH_DEV_GUILD_ID` | opcional: registra os comandos só nesse servidor, que atualiza na hora. Sem ela, o registro é global. |
| `RUST_LOG` | opcional, padrão `info` |

Para os comandos aparecerem, o convite do bot precisa dos escopos `bot` e `applications.commands`. A North encerra limpa com Ctrl+C ou SIGTERM.

## Estrutura

```
src/
├── main.rs        configuração, conexão, registro dos comandos e despacho
└── commands/      um arquivo por comando
```

Cada comando implementa o trait `Command` da NortCod: a definição (nome, descrição, opções) e o `run`. Para criar um comando novo, basta um arquivo em `src/commands/` e uma linha em `commands::all()`.

## Licença

[GNU AGPL-3.0](LICENSE), com uma [permissão adicional](LICENSE-EXCEPTION) para combinar com a NortCod.

Na prática: você pode usar, estudar e modificar a North, mas se rodar uma versão modificada para outras pessoas (um bot público, por exemplo), precisa abrir o código das suas mudanças. A permissão adicional existe porque a NortCod é fechada: ela deixa a North ser combinada com a NortCod sem que a biblioteca precise ser aberta. Contribuições são aceitas sob esses mesmos termos.
