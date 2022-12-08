use std::sync::atomic::{AtomicBool, Ordering};
use anyhow::anyhow;

#[derive(Debug)]
struct MissRatio {
    data: Vec<(usize, f64)>,
}

impl MissRatio {
    fn new<R>(mut input: R) -> anyhow::Result<Self>
    where
        R: std::io::Read,
    {
        use rayon::prelude::*;
        let mut buf = String::new();
        input.read_to_string(&mut buf)?;
        let flag = AtomicBool::new(true);
        let data = buf
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
        if flag.load(Ordering::Relaxed) {
            Ok(MissRatio { data })
        } else {
            Err(anyhow!("failed to parse input"))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::pluss::MissRatio;

    #[test]
    fn basic_deserialize() -> anyhow::Result<()> {
        let data = r#"
        0 , 1.0
        1 , 0.02
        2 , 0.98
        "#
        .to_string();
        let t = MissRatio::new(data.as_bytes())?;
        Ok(println!("{:?}", t))
    }
}
