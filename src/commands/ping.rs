use std::time::{SystemTime, UNIX_EPOCH};

use nortcod::Result;
use nortcod::interactions::{BoxFuture, Command, CommandDefinition, Context};

pub struct Ping;

impl Command for Ping {
    fn definition(&self) -> CommandDefinition {
        CommandDefinition::new("ping", "Mostra se a North está respondendo e quanto demorou")
    }

    fn run<'a>(&'a self, ctx: &'a Context) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            // O ID da interação carrega o momento em que o Discord a criou.
            let created = ctx.interaction().id.timestamp_ms();
            ctx.reply(&message(now_ms().saturating_sub(created))).await
        })
    }
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
}

fn message(elapsed_ms: u64) -> String {
    format!("Pong! Cheguei em {elapsed_ms} ms.")
}

#[cfg(test)]
mod tests {
    #[test]
    fn texto() {
        assert_eq!(super::message(87), "Pong! Cheguei em 87 ms.");
    }
}
