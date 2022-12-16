use crate::pluss::DumpedData;

#[derive(Debug)]
pub struct CCDF {
    pub distribution: Vec<f64>,
}


impl From<DumpedData> for CCDF {
    fn from(value: DumpedData) -> Self {
        use rayon::prelude::*;
        let data = value.data();
        let max : usize = data.last().map(|x|x.0).unwrap_or(0);
        let accesses: f64 = data.par_iter().map(|x| x.1).sum();
        let mut distribution = vec![1.0];
        distribution.reserve_exact(max);
        let mut j = 1;
        for i in 1..max {
            // The histogram can be sparse;
            // The intermediate values are just the same as the previous one
            let prev = distribution[i - 1];
            let current = data[j - 1];
            if current.0 > i {
                distribution.push(prev);
            } else {
                distribution.push(prev - current.1 / accesses);
                j += 1;
            }
        }
        CCDF {
            distribution
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ccdf::CCDF;
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
        Ok(println!("{:?}", CCDF::from(data)))
    }
}