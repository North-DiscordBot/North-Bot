//! North: bot oficial. Nesta fase só confirma que monta a configuração;
//! a conexão com o Discord ainda não existe.

use nortcod::Token;
use nortcod::gateway::pulse::{Next, Session};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let token = match Token::from_env("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(err) => {
            tracing::error!(%err, "defina DISCORD_TOKEN para rodar o North");
            std::process::exit(2);
        }
    };

    let session = Session::new(token, 0, "wss://gateway.discord.gg");
    if let Next::Identify { url } = session.next() {
        tracing::info!(%url, "North pronto; o transporte do Pulse ainda não foi implementado");
    }
}
