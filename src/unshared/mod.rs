mod cli;
mod pluss;
mod rdist;

pub use cli::UnSharedCliOpt;

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

pub fn routine(cli: &cli::UnSharedCliOpt) -> anyhow::Result<()> {
    use plotters::prelude::*;
    let data = std::fs::File::open(&cli.input)?;
    let parsed = pluss::DumpedData::new(data)?;
    let dist = rdist::ReuseDist::from(&parsed);
    let mut ttc = Vec::new();
    for i in (2..=cli.max_cache_size).step_by(cli.cache_size_step) {
        match dist.thread_tolerance(i) {
            None => {
                log::warn!("TTC is not well defined from cache size {}", i);
                break;
            }
            Some((c1, c2)) => {
                ttc.push((i, c1, c2, c2 / c1));
            }
        }
    }
    let json = simd_json::to_string(&ttc)?;
    if let Some(path) = &cli.output {
        std::fs::write(path, json)?;
    } else {
        log::info!("{}", json);
    }

    // put the cache size and ttc element into the new vector for plotting
    let mut ttc_vec = Vec::new();
    for (i, _, _, ttc_val) in ttc {
        ttc_vec.push((i, ttc_val));
    }

    if let Some(path) = &cli.plot {
        if cli.bitmap {
            draw_graph!(BitMapBackend, path, cli, ttc_vec);
        } else {
            draw_graph!(SVGBackend, path, cli, ttc_vec);
        }
    }
    Ok(())
}