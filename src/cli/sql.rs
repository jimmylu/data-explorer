use clap::{ArgMatches, Parser};

use crate::{CmdExcutor, ReplContext, ReplDisplay, ReplMsg};

use super::ReplResult;

#[derive(Debug, Parser)]
pub struct SqlOpts {
    #[arg(short, long, help = "The SQL query to run")]
    pub query: String,
}

pub fn sql(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let query = args
        .get_one::<String>("query")
        .expect("expect query")
        .to_owned();
    // let cmd = SqlOpts::new(query).into();
    let (msg, rx) = ReplMsg::new(SqlOpts::new(query));
    Ok(ctx.send(msg, rx))
}

impl SqlOpts {
    pub fn new(query: String) -> Self {
        Self { query }
    }
}

// impl From<SqlOpts> for ReplCommand {
//     fn from(opts: SqlOpts) -> Self {
//         ReplCommand::Sql(opts)
//     }
// }

impl CmdExcutor for SqlOpts {
    async fn execute<T: crate::Backend>(self, backend: &mut T) -> anyhow::Result<String> {
        let df = backend.sql(&self.query).await?;
        df.display().await
    }
}
