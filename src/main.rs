use clap::Parser;

mod cli;
mod pluss;
mod rdist;

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
        let mut chart = ChartBuilder::on(&canvas)
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..last.0 + 1, 0.0..last.1 + 1.0)?;
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
fn main() -> anyhow::Result<()> {
    use plotters::prelude::*;
    env_logger::init_from_env("TTC_LOG");
    let cli: cli::CliOpt = cli::CliOpt::parse();
    let data = std::fs::File::open(cli.input)?;
    let parsed = pluss::DumpedData::new(data)?;
    let dist = rdist::ReuseDist::from(&parsed);
    let mut ttc = Vec::new();
    for i in (2..=cli.max_cache_size).step_by(cli.cache_size_step) {
        if let Some(x) = dist.thread_tolerance(i) {
            ttc.push((i, x));
        } else {
            log::warn!("TTC is not well defined from cache size {}", i);
            break;
        }
    }
    let json = simd_json::to_string(&ttc)?;
    if let Some(path) = cli.output {
        std::fs::write(path, json)?;
    } else {
        log::info!("{}", json);
    }
    if let Some(path) = cli.plot {
        if cli.bitmap {
            draw_graph!(BitMapBackend, path, cli, ttc);
        } else {
            draw_graph!(SVGBackend, path, cli, ttc);
        }
    }
    Ok(())
}
