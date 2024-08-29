use clap::{ArgMatches, Parser};

use crate::ReplContext;

use super::{ReplCommand, ReplResult};

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
    let cmd = DescribeOpts::new(name).into();
    ctx.send(cmd);

    Ok(None)
}

impl From<DescribeOpts> for ReplCommand {
    fn from(opts: DescribeOpts) -> Self {
        ReplCommand::Describe(opts)
    }
}

impl DescribeOpts {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
