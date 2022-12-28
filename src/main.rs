use clap::Parser;

mod unshared;

#[derive(Debug, Parser)]
enum Command {
    #[command(about = "Caculate AET for unshared data model")]
    Unshared(unshared::UnSharedCliOpt),
    #[command(about = "Caculate AET for shared data model")]
    Shared,
}

fn main() -> anyhow::Result<()> {
    let command: Command = Command::parse();
    match command {
        Command::Unshared(opt) => unshared::routine(&opt)?,
        Command::Shared => todo!(),
    }
    Ok(())
}
