pub type Triple = (usize, f64, f64);

#[derive(Debug, Clone)]
pub struct DumppedData {
    pub local: Vec<Triple>,
    pub shared: Vec<Triple>,
}

impl DumppedData {
    pub fn new<R>(mut input: R) -> anyhow::Result<Self>
    where
        R: std::io::Read,
    {
        let mut buf = String::new();
        let mut shared = false;
        input.read_to_string(&mut buf)?;
        let mut shared_data = Vec::new();
        let mut local_data = Vec::new();
        for i in buf.trim().lines() {
            if i.starts_with("Start") {
                continue;
            }
            if i.starts_with("No share") {
                shared = false;
                continue;
            }
            if i.starts_with("Share") {
                shared = true;
                continue;
            }
            let mut split = i.split(',');
            let x = split
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing first field"))
                .and_then(|x| x.trim().parse().map_err(Into::into))?;
            let y = split
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing first field"))
                .and_then(|x| x.trim().parse().map_err(Into::into))?;
            let z = split
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing first field"))
                .and_then(|x| x.trim().parse().map_err(Into::into))?;
            if shared {
                shared_data.push((x, y, z));
            } else {
                local_data.push((x, y, z));
            }
        }
        shared_data.sort_by_key(|x| x.0);
        local_data.sort_by_key(|x| x.0);
        Ok(Self {
            shared: shared_data,
            local: local_data,
        })
    }
}

#[cfg(test)]
pub(in crate::shared) mod test {
    pub fn shared_data() -> super::DumppedData {
        let mut result = super::DumppedData {
            local: vec![
                (512, 1.83501e+06, 0.225013),
                (256, 260096.0, 0.0318935),
                (4, 1.83501e+06, 0.225013),
                (2, 2.09715e+06, 0.257157),
                (1, 2.12787e+06, 0.260924),
            ],
            shared: vec![(32768, 253952.0, 1.0)],
        };
        result.local.sort_by_key(|x| x.0);
        result.shared.sort_by_key(|x| x.0);
        result
    }

    #[test]
    fn test() {
        let input = r#"
No share reuses
Start to dump reuse time
512,1.83501e+06,0.225013
256,260096,0.0318935
4,1.83501e+06,0.225013
2,2.09715e+06,0.257157
1,2.12787e+06,0.260924
Share reuses
Start to dump reuse time
32768,253952,1
        "#;
        let dumpped_data = super::DumppedData::new(input.as_bytes()).unwrap();
        println!("{:?}", dumpped_data);
    }
}
