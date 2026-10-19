# Segurança

- **Encontrou uma falha?** Não abra issue pública. Use o "Report a vulnerability" na aba Security deste repositório (reporte privado do GitHub).
- **Token.** A North lê o token só de `DISCORD_TOKEN`. Ele nunca entra no código, em log ou em commit (`.env`, `*.pem` e `*.key` estão no `.gitignore`). Se um token vazar, regere no Developer Portal na hora.
- **Sem `unsafe`.** O lint `unsafe_code = "forbid"` vale para o crate inteiro.
