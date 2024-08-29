use anyhow::Result;
use data_explorer::{get_callbacks, ReplCommand, ReplContext};
use reedline_repl_rs::Repl;
const HISTORY_SIZE: usize = 1024;

fn main() -> Result<()> {
    let ctx = ReplContext::new();
    let callbacks = get_callbacks();

    let history_file = directories::BaseDirs::new()
        .expect("expect base dirs")
        .home_dir()
        .join(".data-explorer");
    let mut repl = Repl::new(ctx)
        .with_history(history_file, HISTORY_SIZE)
        .with_banner("Welcome to Data-explorer, your dataset exploration REPL!")
        .with_derived::<ReplCommand>(callbacks);

    repl.run()?;

    Ok(())
}
