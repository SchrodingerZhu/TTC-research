mod dist;
mod triple;

pub fn routine(cli: &crate::CliOpt) -> anyhow::Result<()> {
    use plotters::prelude::*;
    let data = std::fs::File::open(&cli.input)?;
    let parsed = triple::DumppedData::new(data);
    let dist = dist::DistWithSharing::new(parsed);
    let mut ttc = Vec::new();
    let nonshared_weight = 1.0 - dist.shared_samples / dist.total_samples;
    let shared_weight = dist.shared_samples / dist.total_samples;
    for i in (2..=cli.max_cache_size).step_by(cli.cache_size_step) {
        match dist.thread_tolerance_bound(i) {
            None => {
                log::warn!("TTC is not well defined from cache size {}", i);
                break;
            }
            Some((c1, c2)) => {
                ttc.push((i, c1, c2, c1 / c2, 1.0 - (1.0 - dist.local_dist.ccdf[i]) * nonshared_weight - shared_weight));
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
    for (i, _, _, ttc_val, _) in ttc {
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
