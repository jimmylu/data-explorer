use std::{ops::Deref, thread};

use cli::{connect, describe, head, list, sql};
use crossbeam_channel as mpsc;
use reedline_repl_rs::CallBackMap;
mod cli;
pub use cli::ReplCommand;
pub struct ReplContext {
    pub tx: mpsc::Sender<ReplCommand>,
}

pub type ReplCallBacks = CallBackMap<ReplContext, reedline_repl_rs::Error>;

pub fn get_callbacks() -> ReplCallBacks {
    let mut map = CallBackMap::new();
    map.insert("connect".to_string(), connect);
    map.insert("describe".to_string(), describe);
    map.insert("sql".to_string(), sql);
    map.insert("head".to_string(), head);
    map.insert("list".to_string(), list);
    map
}
impl ReplContext {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded();
        thread::Builder::new()
            .name("ReplBackend".to_string())
            .spawn(move || {
                while let Ok(cmd) = rx.recv() {
                    println!("!!! cmd: {:?}", cmd);
                }
            })
            .unwrap();
        Self { tx }
    }

    pub fn send(&self, cmd: ReplCommand) {
        if let Err(e) = self.tx.send(cmd) {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
impl Default for ReplContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for ReplContext {
    type Target = mpsc::Sender<ReplCommand>;

    fn deref(&self) -> &Self::Target {
        &self.tx
    }
}
