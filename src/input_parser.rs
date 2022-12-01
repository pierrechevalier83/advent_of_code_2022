use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_line_vec<T>(s: &str) -> Result<Vec<T>, <T as FromStr>::Err>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.lines().map(|line| line.parse()).collect()
}

pub fn parse_delimited_vec<T>(s: &str, sep: &str) -> Result<Vec<T>, <T as FromStr>::Err>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.split(sep).map(|line| line.parse()).collect()
}
