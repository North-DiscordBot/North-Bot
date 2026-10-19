//! North: bot oficial. Por enquanto busca o Gateway pelo REST, conecta pelo
//! Pulse e registra os eventos; comandos entram depois.

use nortcod::Token;
use nortcod::events::Event;
use nortcod::gateway::intents;
use nortcod::gateway::pulse::{Pulse, PulseConfig, Session};
use nortcod::rest::RestClient;
use tracing_subscriber::EnvFilter;

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

    let rest = match RestClient::new(token.clone()) {
        Ok(rest) => rest,
        Err(err) => fail("não deu para montar o cliente REST", err),
    };
    let gateway = match rest.gateway_bot().await {
        Ok(gateway) => gateway,
        Err(err) => fail("GET /gateway/bot falhou (token certo?)", err),
    };
    let limit = &gateway.session_start_limit;
    tracing::info!(shards = gateway.shards, sessoes_restantes = limit.remaining, "gateway encontrado");
    if limit.remaining == 0 {
        tracing::error!(reset_ms = limit.reset_after, "limite diário de sessões esgotado");
        std::process::exit(1);
    }
    if gateway.shards > 1 {
        tracing::warn!(shards = gateway.shards, "Discord sugere sharding; o North ainda roda com 1 shard");
    }

    let session = Session::new(token, intents::GUILDS, gateway.url);
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

fn fail(what: &str, err: nortcod::Error) -> ! {
    tracing::error!(%err, "{what}");
    std::process::exit(1);
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
