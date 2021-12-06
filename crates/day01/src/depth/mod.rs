use common::error::AocError;

pub trait Depth {
    fn depth(self) -> Result<u32, AocError>;
}

impl Depth for &[u32] {
    fn depth(self) -> Result<u32, AocError> {
        Ok(self.iter().sum())
    }
}

impl Depth for Result<String, std::io::Error> {
    fn depth(self) -> Result<u32, AocError> {
        Ok(self?.parse()?)
    }
}

pub fn rate_of_depth_increase<T>(iter: T) -> Result<u32, AocError>
where
    T: IntoIterator,
    T::Item: Depth,
{
    let mut last: u32 = 0;
    let mut count: u32 = 0;

    for e in iter {
        let current = e.depth()?;
        if current > last {
            count += 1;
        }
        last = current;
    }
    Ok(count - 1)
}
