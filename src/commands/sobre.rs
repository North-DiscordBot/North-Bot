use nortcod::Result;
use nortcod::interactions::{BoxFuture, Command, CommandDefinition, Context};

pub struct Sobre;

impl Command for Sobre {
    fn definition(&self) -> CommandDefinition {
        CommandDefinition::new("sobre", "Conta o que é o North")
    }

    fn run<'a>(&'a self, ctx: &'a Context) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move { ctx.reply(&message()).await })
    }
}

fn message() -> String {
    format!(
        "**North** {}\nFeito em Rust, do zero, com a NortCod (a biblioteca da casa para falar com o Discord).",
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn mostra_a_versao() {
        assert!(super::message().starts_with(concat!("**North** ", env!("CARGO_PKG_VERSION"))));
    }
}
