use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Generate files in public directory
    Build,
    /// Useless
    Test,
    /// start localhost server
    Server,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "blogr", about = "A Rust static sites generator")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
