mod pluss;
mod rdist;

pub(crate) type UnsharedDist = rdist::ReuseDist;
pub(crate) type UnsharedData = pluss::DumpedData;

pub fn routine(cli: &crate::CliOpt) -> anyhow::Result<()> {
    use plotters::prelude::*;
    let data = std::fs::File::open(&cli.input)?;
    let parsed = pluss::DumpedData::new(data)?;
    let dist = rdist::ReuseDist::from(parsed);
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
            crate::draw_graph!(BitMapBackend, path, cli, ttc_vec);
        } else {
            crate::draw_graph!(SVGBackend, path, cli, ttc_vec);
        }
    }
    Ok(())
}
