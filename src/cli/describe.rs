use clap::{ArgMatches, Parser};

use crate::{CmdExcutor, ReplContext, ReplDisplay, ReplMsg};

use super::ReplResult;

#[derive(Debug, Parser)]
pub struct DescribeOpts {
    #[arg(short, long, help = "The name of the dataset")]
    pub name: String,
}

pub fn describe(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let name = args
        .get_one::<String>("name")
        .expect("expect name")
        .to_owned();
    // let cmd = DescribeOpts::new(name).into();
    let (msg, rx) = ReplMsg::new(DescribeOpts::new(name));
    Ok(ctx.send(msg, rx))
}

// impl From<DescribeOpts> for ReplCommand {
//     fn from(opts: DescribeOpts) -> Self {
//         ReplCommand::Describe(opts)
//     }
// }

impl DescribeOpts {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl CmdExcutor for DescribeOpts {
    async fn execute<T: crate::Backend>(self, backend: &mut T) -> anyhow::Result<String> {
        let df = backend.describe(&self.name).await?;
        df.display().await
    }
}
