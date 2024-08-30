use clap::Parser;
use enum_dispatch::enum_dispatch;

mod connect;
mod describe;
mod head;
mod list;
mod sql;

pub use self::{connect::connect, describe::describe, head::head, list::list, sql::sql};
pub use self::{
    connect::{ConnectOpts, DatasetConn},
    describe::DescribeOpts,
    head::HeadOpts,
    list::ListOpts,
    sql::SqlOpts,
};

type ReplResult = Result<Option<String>, reedline_repl_rs::Error>;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
pub enum ReplCommand {
    #[command(
        name = "connect",
        about = "Connect to a dataset and register it to Data-Explorer"
    )]
    Connect(ConnectOpts),

    #[command(name = "list", about = "List all datasets")]
    List(ListOpts),

    #[command(name = "describe", about = "Describe a dataset")]
    Describe(DescribeOpts),

    #[command(about = "Show the first few rows of a dataset")]
    Head(HeadOpts),

    #[command(about = "Query a dataset using given SQL")]
    Sql(SqlOpts),
}
