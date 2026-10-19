//! Slash commands da North. Cada comando é um arquivo; `all()` junta todos.

mod ping;
mod sobre;

use nortcod::Result;
use nortcod::interactions::Commands;

pub fn all() -> Result<Commands> {
    Commands::new().register(ping::Ping)?.register(sobre::Sobre)
}

#[cfg(test)]
mod tests {
    #[test]
    fn todos_os_comandos_sao_validos() {
        let defs = super::all().unwrap().definitions();
        let names: Vec<_> = defs.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(names, ["ping", "sobre"]);
    }
}
