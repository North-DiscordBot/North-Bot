//! North: bot oficial. Por enquanto conecta pelo Pulse e registra os eventos;
//! comandos entram depois.

use nortcod::Token;
use nortcod::events::Event;
use nortcod::gateway::intents;
use nortcod::gateway::pulse::{Pulse, PulseConfig, Session};
use tracing_subscriber::EnvFilter;

/// Base do Gateway enquanto o REST não busca GET /gateway/bot.
const GATEWAY_URL: &str = "wss://gateway.discord.gg";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let token = match Token::from_env("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(err) => {
            tracing::error!(%err, "defina DISCORD_TOKEN para rodar o North");
            std::process::exit(2);
        }
    };

    let session = Session::new(token, intents::GUILDS, GATEWAY_URL);
    let (pulse, handle, mut events) = Pulse::new(session, PulseConfig::default());
    let pulse = tokio::spawn(pulse.run());

    let stop = shutdown_signal();
    tokio::pin!(stop);
    loop {
        tokio::select! {
            Some(event) = events.recv() => match event {
                Event::Ready(_) => tracing::info!("North online"),
                Event::Resumed => tracing::info!("sessão retomada"),
                Event::Unknown { name, .. } => tracing::debug!(%name, "evento"),
            },
            _ = &mut stop => {
                handle.shutdown();
                break;
            }
            else => break,
        }
    }

    match pulse.await {
        Ok(Ok(())) => tracing::info!("North encerrado"),
        Ok(Err(err)) => {
            tracing::error!(%err, "Pulse parou");
            std::process::exit(1);
        }
        Err(err) => {
            tracing::error!(%err, "Pulse travou");
            std::process::exit(1);
        }
    }
}

/// Ctrl+C no terminal ou SIGTERM do systemd.
async fn shutdown_signal() {
    use tokio::signal::unix::{SignalKind, signal};
    let mut term = signal(SignalKind::terminate()).expect("registrar SIGTERM");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = term.recv() => {}
    }
}
