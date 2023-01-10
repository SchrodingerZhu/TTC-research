#![feature(iter_collect_into)]
use clap::Parser;

mod shared;
mod unshared;
mod util;
use std::path::PathBuf;

#[macro_export]
macro_rules! draw_graph {
    ($backend:ident, $path:expr, $cli:expr, $ttc:expr) => {
        let canvas = $backend::new(
            &$path,
            (
                $cli.plot_width.unwrap_or(640),
                $cli.plot_height.unwrap_or(480),
            ),
        )
        .into_drawing_area();
        canvas.fill(&WHITE)?;
        let last = $ttc.last().cloned().unwrap_or((0, 0.0));
        let max = $ttc.iter().map(|x| x.1 as usize + 1).max().unwrap_or(2) as f64;
        let mut chart = ChartBuilder::on(&canvas)
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..last.0 + 1, 0.0..max + 1.0)?;
        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new($ttc, RED))?
            .label("TTC")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;

        canvas.present()?;
    };
}

#[derive(Debug, Parser)]
pub struct CliOpt {
    #[arg(long, short = 'p', help = "Render the TTC curve")]
    pub plot: Option<PathBuf>,
    #[arg(long, short = 'W', help = "Plot width")]
    pub plot_width: Option<u32>,
    #[arg(long, short = 'H', help = "Plot height")]
    pub plot_height: Option<u32>,
    #[arg(long, short, default_value = "64", help = "Maximum cache size")]
    pub max_cache_size: usize,
    #[arg(
        long,
        short,
        default_value = "1",
        help = "Increment step of cache size"
    )]
    pub cache_size_step: usize,
    #[arg(long, short, help = "Path to the input file")]
    pub input: PathBuf,
    #[arg(long, short, help = "Output path")]
    pub output: Option<PathBuf>,
    #[arg(long, short, help = "Use bitmap instead of SVG")]
    pub bitmap: bool,
}

#[derive(Debug, Parser)]
enum Command {
    #[command(about = "Caculate AET for unshared data model")]
    Unshared(CliOpt),
    #[command(about = "Caculate AET for shared data model")]
    Shared(CliOpt),
}

fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("TTC_LOG");
    let command: Command = Command::parse();
    match command {
        Command::Unshared(opt) => unshared::routine(&opt)?,
        Command::Shared(opt) => shared::routine(&opt)?,
    }
    Ok(())
}
