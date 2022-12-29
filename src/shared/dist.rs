use super::triple::DumppedData;
pub type Dist = crate::unshared::UnsharedDist;
pub type Data = crate::unshared::UnsharedData;

pub struct DistWithSharing {
    local_dist: Dist,
    total_dist: Dist,
    shared_cardinality: usize,
    total_samples: f64,
    shared_samples: f64
}

impl DistWithSharing {
    pub fn new(dumped_data: DumppedData) -> Self {
        let shared_cardinality = dumped_data.shared.len();
        let shared_samples = dumped_data.shared.iter().map(|x|x.1).sum::<f64>();
        let total_samples = dumped_data.local.iter().map(|x|x.1).sum::<f64>() + shared_samples;
        let mut data: Vec<(usize, f64)> =
            dumped_data.local.into_iter().map(|x| (x.0, x.1)).collect();
        data.sort_by_key(|x| x.0);
        let local_dist = Dist::from(Data { data: data.clone() });
        dumped_data
            .shared
            .into_iter()
            .map(|x| (x.0, x.1))
            .collect_into(&mut data);
        let total_dist = Dist::from(Data { data });
        Self {
            local_dist,
            total_dist,
            shared_cardinality,
            total_samples,
            shared_samples
        }
    }

    pub fn total_aet(&self, cache_size: usize) -> Option<usize> {
        self.total_dist.aet(cache_size)
    }

    pub fn shared_aet(&self, aet: usize) -> Option<usize> {
        // now, assume that all reuse in the total distribution is moved to the private reuse range.
        // we scan over the PRI cdf to see if it can reaches the boundary.
        let local_weight = (self.total_samples - self.shared_samples) / self.total_samples;
        let shared_weight = 1.0 - local_weight;
        let target_cdf = self.total_dist.cdf.get(aet)?; // miss ratio = 1 - target_cdf
        let shared_aet = self.local_dist.cdf.partition_point(|x| x * local_weight + shared_weight < *target_cdf); // first index i with CDF_NBD(i) >= target cdf (CDF_RaceTrack(R_N))
        if shared_aet >= self.local_dist.max_reuse_distance() || shared_aet >= aet {
            None
        } else {
            Some(shared_aet)
        }
    }

    pub fn thread_tolerance_bound(&self, cache_size: usize) -> Option<(f64, f64)> {
        let aet = self.total_aet(cache_size)?;
        let shared_aet = self.shared_aet(aet);
        match shared_aet {
            // consider shared reuse does not give a change
            None => {
                let index = self
                    .total_dist
                    .original
                    .partition_point(|x| x.0 < aet)
                    .checked_sub(1)?;
                let x = self.total_dist.original.get(index)?.0;
                let y = self.total_dist.original.get(index + 1)?.0;
                let ex = self.total_dist.cond_exp(x)?;
                let ey = self.total_dist.cond_exp(y)?;
                let e1 = self.total_dist.cond_exp(1)?;
                let a = y as f64 * self.total_dist.ccdf[y] + (ey - e1);
                let b = x as f64 * self.total_dist.ccdf[x] + (ex - e1);
                Some((b, a))
            }
            Some(shared_aet) => {
                let index_y = self.total_dist.original.partition_point(|x| x.0 < aet);
                let index_x = self
                    .total_dist
                    .original
                    .partition_point(|x| x.0 < shared_aet)
                    .checked_sub(1)?;
                let x = self.total_dist.original.get(index_x)?.0;
                let y = self.total_dist.original.get(index_y)?.0;
                let ey = self.total_dist.cond_exp(y)?;
                let e1 = self.total_dist.cond_exp(1)?;
                let a = x as f64 * self.total_dist.ccdf[x] + self.shared_cardinality as f64;
                let b = y as f64 * self.total_dist.ccdf[y] + (ey - e1);
                Some((b, a))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::shared::triple::test::shared_data;

    #[test]
    fn test() {
        let data = shared_data();
        let dist = super::DistWithSharing::new(data);
        for i in 0..128 {
            let bnd = dist.thread_tolerance_bound(i);
            println!(
                "cache = {}, (b, a) = {:?}, ttc = {:?}",
                i,
                bnd,
                bnd.map(|(x, y)| x / y)
            );
        }
    }
}
