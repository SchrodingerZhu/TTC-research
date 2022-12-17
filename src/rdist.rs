use crate::pluss::DumpedData;

#[derive(Debug)]
pub struct ReuseDist<'a> {
    pub original: &'a [(usize, f64)],
    // probability density
    pub pdf: Vec<f64>,
    // cumulative density
    pub cdf: Vec<f64>,
    // accumulated expectation
    pub aexp: Vec<f64>,
    // complement cumulative density
    pub ccdf: Vec<f64>,
    // accumulated complement cumulative density
    pub accdf: Vec<f64>,
}

impl<'a> From<&'a DumpedData> for ReuseDist<'a> {
    fn from(value: &'a DumpedData) -> Self {
        use rayon::prelude::*;
        let data = value.data();
        let max: usize = data.last().map(|x| x.0).unwrap_or(0);
        let accesses: f64 = data.par_iter().map(|x| x.1).sum();
        let mut pdf = vec![0.0];
        let mut cdf = vec![0.0];
        let mut aexp = vec![0.0];
        let mut ccdf = vec![1.0];
        let mut accdf = vec![1.0];
        ccdf.reserve_exact(max);
        accdf.reserve_exact(max);
        pdf.reserve_exact(max);
        cdf.reserve_exact(max);
        aexp.reserve_exact(max);
        let mut j = 1;
        for i in 1..=max {
            // The histogram can be sparse;
            // The intermediate values are just the same as the previous one
            let mut update_j = false;
            {
                // update ccdf
                let prev = ccdf[i - 1];
                let current = data[j - 1];
                if current.0 > i {
                    ccdf.push(prev);
                } else {
                    ccdf.push(prev - current.1 / accesses);
                    update_j = true;
                }
                accdf.push(ccdf[i] + accdf[i - 1]);
            }
            {
                // update standard distribution
                let current = data[j - 1];
                if current.0 > i {
                    pdf.push(0.0);
                } else {
                    pdf.push(current.1 / accesses);
                    update_j = true;
                }
                cdf.push(pdf[i] + cdf[i - 1]);
                aexp.push(pdf[i] * (i as f64) + aexp[i - 1])
            }
            if update_j {
                j += 1;
            }
        }
        Self {
            original: data,
            pdf,
            cdf,
            aexp,
            ccdf,
            accdf,
        }
    }
}

impl<'a> ReuseDist<'a> {
    pub fn max_reuse_distance(&self) -> usize {
        self.ccdf.len() - 1
    }
    pub fn aet(&self, cache_size: usize) -> Option<usize> {
        let pt = self.accdf.partition_point(|x| (*x as usize) < cache_size);
        if pt < self.max_reuse_distance() {
            Some(pt)
        } else {
            None
        }
    }
    pub fn cond_exp(&self, reuse_interval: usize) -> Option<f64> {
        self.aexp.get(reuse_interval).cloned()
    }
    pub fn reuse_interval_boundaries(&self, cache_size: usize) -> Option<(usize, usize)> {
        self.aet(cache_size)
            .and_then(|aet| {
                let pt = self.original.partition_point(|x| x.0 < aet);
                pt.checked_sub(1)
            })
            .and_then(|idx| self.original.get(idx).map(|x| (idx, x.0)))
            .and_then(|(idx, lower)| self.original.get(idx + 1).map(|x| (lower, x.0)))
    }
    pub fn thread_tolerance(&self, cache_size: usize) -> Option<f64> {
        let (x, y) = self.reuse_interval_boundaries(cache_size)?;
        let ex = self.cond_exp(x)?;
        let ey = self.cond_exp(y)?;
        let e1 = self.cond_exp(1)?;
        let a = y as f64 * self.ccdf[y] + (ey - e1) as f64;
        let b = x as f64 * self.ccdf[x] + (ex - e1) as f64;
        Some(a / b)
    }
}

#[cfg(test)]
mod test {
    use crate::pluss::DumpedData;
    use crate::rdist::ReuseDist;

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
        let x = ReuseDist::from(&data);
        println!("{:#?}", x);
        assert_eq!(x.max_reuse_distance(), 32);
        assert_eq!(x.aet(2), Some(2));
        assert_eq!(x.aet(3), Some(3));
        assert_eq!(x.aet(4), Some(4));
        assert_eq!(x.aet(5), Some(6));

        // when cache is too large, the aet is undefined
        assert_eq!(x.aet(16), None);
        assert_eq!(x.aet(32), None);
        println!("{:?}", x.reuse_interval_boundaries(4));
        println!("{:?}", x.thread_tolerance(2));
        Ok(())
    }
}
