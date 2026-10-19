//! North: bot oficial. Busca o Gateway pelo REST, conecta pelo Pulse,
//! registra os slash commands no READY e responde às interações.
//!
//! Ambiente: `DISCORD_TOKEN` (obrigatório) e `NORTH_DEV_GUILD_ID` (opcional:
//! registra os comandos só nesse servidor, que atualiza na hora; sem ele,
//! registra global).

mod commands;

use std::sync::Arc;

use nortcod::events::Event;
use nortcod::interactions::Commands;
use nortcod::{Id, Token};
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
            tracing::error!(%err, "defina DISCORD_TOKEN para rodar a North");
            std::process::exit(2);
        }
    };

    let dev_guild = match std::env::var("NORTH_DEV_GUILD_ID") {
        Ok(raw) => match raw.trim().parse() {
            Ok(id) => Some(Id(id)),
            Err(_) => {
                tracing::error!("NORTH_DEV_GUILD_ID precisa ser o ID numérico do servidor");
                std::process::exit(2);
            }
        },
        Err(_) => None,
    };
    let commands = match commands::all() {
        Ok(commands) => Arc::new(commands),
        Err(err) => fail("comandos inválidos", err),
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
        tracing::warn!(shards = gateway.shards, "Discord sugere sharding; a North ainda roda com 1 shard");
    }

    let session = Session::new(token, intents::GUILDS, gateway.url);
    let (pulse, handle, mut events) = Pulse::new(session, PulseConfig::default());
    let pulse = tokio::spawn(pulse.run());

    let mut registered = false;
    let stop = shutdown_signal();
    tokio::pin!(stop);
    loop {
        tokio::select! {
            Some(event) = events.recv() => match event {
                Event::Ready(ready) => {
                    let name = ready.user.as_ref().map(|u| u.username.as_str()).unwrap_or("?");
                    tracing::info!(%name, "North online");
                    // Um registro por processo: um READY depois de cair não precisa de outro.
                    match ready.application {
                        Some(app) if !registered => {
                            registered = true;
                            tokio::spawn(register(rest.clone(), app.id, dev_guild, commands.clone()));
                        }
                        Some(_) => {}
                        None => tracing::warn!("READY sem application.id, comandos não registrados"),
                    }
                }
                Event::Resumed => tracing::info!("sessão retomada"),
                Event::InteractionCreate(interaction) => {
                    // Cada uma na sua tarefa: o Discord só espera 3 s pela resposta.
                    let (rest, commands) = (rest.clone(), commands.clone());
                    tokio::spawn(async move {
                        if let Err(err) = commands.dispatch(rest, *interaction).await {
                            tracing::warn!(%err, "interação terminou com erro");
                        }
                    });
                }
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
        Ok(Ok(())) => tracing::info!("North encerrada"),
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

async fn register(rest: RestClient, app: Id, guild: Option<Id>, commands: Arc<Commands>) {
    let defs = commands.definitions();
    let result = match guild {
        Some(guild) => rest.set_guild_commands(app, guild, &defs).await,
        None => rest.set_global_commands(app, &defs).await,
    };
    match result {
        Ok(done) => tracing::info!(total = done.len(), servidor = ?guild.map(|g| g.0), "comandos registrados"),
        Err(err) => tracing::error!(%err, "falha ao registrar os comandos"),
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
