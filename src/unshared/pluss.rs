use anyhow::anyhow;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct DumpedData {
    pub(crate) data: Vec<(usize, f64)>,
}

impl DumpedData {
    pub fn new<R>(mut input: R) -> anyhow::Result<Self>
    where
        R: std::io::Read,
    {
        use rayon::prelude::*;
        let mut buf = String::new();
        input.read_to_string(&mut buf)?;
        let flag = AtomicBool::new(true);
        let mut data: Vec<(usize, f64)> = buf
            .par_lines()
            .filter(|x| x.contains(','))
            .map(|x: &str| {
                let mut split = x.trim().split(',');
                split
                    .next()
                    .ok_or_else(|| anyhow!("missing field"))
                    .and_then(|x| x.trim().parse().map_err(Into::into))
                    .and_then(|x| {
                        split
                            .next()
                            .ok_or_else(|| anyhow!("missing field"))
                            .and_then(|x| x.trim().parse().map_err(Into::into))
                            .map(|y| (x, y))
                    })
            })
            .filter_map(|x| {
                if let Err(ref e) = x {
                    flag.store(false, Ordering::SeqCst);
                    log::error!("error parsing input: {}", e)
                }
                x.ok()
            })
            .collect();
        data.par_sort_unstable_by_key(|x| x.0);
        data = crate::util::merge_overlapped(data);
        if flag.load(Ordering::Relaxed) {
            log::info!("{} data points loaded", data.len());
            Ok(DumpedData { data })
        } else {
            Err(anyhow!("failed to parse input"))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::unshared::pluss::DumpedData;

    #[test]
    fn basic_deserialize() -> anyhow::Result<()> {
        let data = r#"
1, 1059850.0
2, 1456590.0
4, 2437730.0
8, 4409820.0
16, 2588550.0
32, 141462.0
        "#
        .to_string();
        let t = DumpedData::new(data.as_bytes())?;
        Ok(println!("{:?}", t))
    }
}
