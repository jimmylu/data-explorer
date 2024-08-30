use clap::{ArgMatches, Parser};

use crate::{CmdExcutor, ReplContext, ReplDisplay, ReplMsg};

use super::ReplResult;

#[derive(Debug, Parser)]
pub struct HeadOpts {
    #[arg(long, help = "The name of dataset")]
    pub name: String,
    #[arg(long, help = "The number of rows to show")]
    pub n: Option<usize>,
}

pub fn head(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let name = args
        .get_one::<String>("name")
        .expect("expect name")
        .to_owned();
    let n = args.get_one::<usize>("n").copied();
    // let cmd = HeadOpts::new(name, n).into();
    let (msg, rx) = ReplMsg::new(HeadOpts::new(name, n));
    Ok(ctx.send(msg, rx))
}

// impl From<HeadOpts> for ReplCommand {
//     fn from(opts: HeadOpts) -> Self {
//         ReplCommand::Head(opts)
//     }
// }
impl HeadOpts {
    pub fn new(name: String, n: Option<usize>) -> Self {
        Self { name, n }
    }
}

impl CmdExcutor for HeadOpts {
    async fn execute<T: crate::Backend>(self, backend: &mut T) -> anyhow::Result<String> {
        let df = backend.head(&self.name, self.n.unwrap_or(5)).await?;
        df.display().await
    }
}
