pub type Triple = (usize, f64, f64);

#[derive(Debug, Clone)]
pub struct DumppedData {
    local: Vec<Triple>,
    shared: Vec<Triple>,
}

#[cfg(test)]
pub(in crate::shared) mod test {
    pub fn shared_data() -> super::DumppedData {
        super::DumppedData {
            local: vec![
                (512, 1.83501e+06, 0.225013),
                (256, 260096.0, 0.0318935),
                (4, 1.83501e+06, 0.225013),
                (2, 2.09715e+06, 0.257157),
                (1, 2.12787e+06, 0.260924),
            ],
            shared: vec![(32768, 253952.0, 1.0)],
        }
    }
}
