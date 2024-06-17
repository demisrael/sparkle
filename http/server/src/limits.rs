use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct RateLimit {
    pub requests: u64,
    pub period: u64,
}

impl FromStr for RateLimit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(':');
        let (requests, period) = match parts {
            None | Some(("", _)) | Some((_, "")) => {
                return Err("invalid rate limit, must be `<requests>:<period>`".to_string());
            }
            Some(x) => x,
        };
        let requests = requests.parse().map_err(|_| {
            format!(
                "Unable to parse number of requests, the value must be an integer, supplied: {:?}",
                requests
            )
        })?;
        let period = period.parse().map_err(|_| {
            format!("Unable to parse period, the value must be an integer specifying number of seconds, supplied: {:?}", period)
        })?;

        Ok(RateLimit { requests, period })
    }
}
