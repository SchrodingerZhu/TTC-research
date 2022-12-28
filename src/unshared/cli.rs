use std::path::PathBuf;

use clap::Parser;
#[derive(Debug, Parser)]
pub struct UnSharedCliOpt {
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
