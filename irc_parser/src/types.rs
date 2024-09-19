use std::str::FromStr;

#[derive(Default)]
pub struct SpaceSeparatedList<T: FromStr> {
    pub values: Vec<T>,
}

impl<T: FromStr> FromStr for SpaceSeparatedList<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(' ')
            .map(|val| T::from_str(val))
            .collect::<Result<_, _>>()
            .map_err(|_| anyhow::anyhow!("failed to parse space separated array value"))?;

        Ok(Self { values })
    }
}
