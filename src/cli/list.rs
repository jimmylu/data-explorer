use clap::{ArgMatches, Parser};

use crate::{CmdExcutor, ReplContext, ReplDisplay, ReplMsg};

use super::ReplResult;

#[derive(Debug, Parser)]
pub struct ListOpts;
pub fn list(_args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let (msg, rx) = ReplMsg::new(ListOpts);
    Ok(ctx.send(msg, rx))
}

impl CmdExcutor for ListOpts {
    async fn execute<T: crate::Backend>(self, backend: &mut T) -> anyhow::Result<String> {
        let df = backend.list().await?;
        df.display().await
    }
}

// impl From<ListOpts> for ReplCommand {
//     fn from(opts: ListOpts) -> Self {
//         ReplCommand::List(opts)
//     }
// }
