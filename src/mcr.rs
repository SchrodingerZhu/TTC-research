use crate::ccdf::CCDF;

#[derive(Debug)]
pub struct MRC {
    pub ratios: Vec<f64>
}

impl MRC {
    fn new(ccdf: &CCDF, cache_limit: usize) -> Self {
        let mut integration = 0.0;
        let mut t = 0;
        let mut ratios = vec![1.0];
        for c in 1..cache_limit {
            while (integration as usize) < c && t < ccdf.distribution.len() {
                integration += ccdf.distribution[t];
                t += 1;
            }
            ratios.push(ccdf.distribution[t - 1]);
        }
        MRC {
            ratios
        }
    }
}


#[cfg(test)]
mod test {
    use crate::ccdf::CCDF;
    use crate::mcr::MRC;
    use crate::pluss::DumpedData;

    #[test]
    fn basic() -> anyhow::Result<()> {
        let data = DumpedData {
            data: vec![
                (1, 1059850.0),
                (2, 1456590.0),
                (4, 2437730.0),
                (8, 4409820.0),
                (16, 2588550.0),
                (32, 141462.0),
            ]
        };
        Ok(println!("{:?}", MRC::new(&CCDF::from(data), 8)))
    }
}