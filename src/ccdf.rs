use crate::pluss::DumpedData;

#[derive(Debug)]
pub struct CCDF {
    pub distribution: Vec<f64>,
    pub accumulation: Vec<f64>,
}

impl<'a> From<&'a DumpedData> for CCDF  {
    fn from(value: &'a DumpedData) -> Self {
        use rayon::prelude::*;
        let data = value.data();
        let max: usize = data.last().map(|x| x.0).unwrap_or(0);
        let accesses: f64 = data.par_iter().map(|x| x.1).sum();
        let mut distribution = vec![1.0];
        let mut accumulation = vec![1.0];
        distribution.reserve_exact(max);
        accumulation.reserve_exact(max);
        let mut j = 1;
        for i in 1..=max {
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
            accumulation.push(distribution[i] + accumulation[i - 1]);
        }
        CCDF {
            distribution,
            accumulation,
        }
    }
}

impl CCDF {
    pub fn max_reuse_distance(&self) -> usize {
        self.distribution.len() - 1
    }
    pub fn aet(&self, cache_size: usize) -> Option<usize> {
        self.accumulation
            .binary_search_by_key(&cache_size, |x| *x as usize)
            .ok()
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
            ],
        };
        let x = CCDF::from(&data);
        println!("{:#?}", x);
        assert_eq!(x.max_reuse_distance(), 32);
        assert_eq!(x.aet(2), Some(2));
        assert_eq!(x.aet(3), Some(3));
        assert_eq!(x.aet(4), Some(4));
        assert_eq!(x.aet(5), Some(6));

        // when cache is too large, the aet is undefined
        assert_eq!(x.aet(16), None);
        assert_eq!(x.aet(32), None);
        Ok(())
    }
}
