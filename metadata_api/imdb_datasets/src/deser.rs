use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

pub fn imdb_id<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    id_split(s, "tt")
        .map_err(de::Error::custom)
}

pub fn name_id<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    id_split(s, "nm")
        .map_err(de::Error::custom)
}

pub fn boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let i: u8 = Deserialize::deserialize(deserializer)?;
    Ok(num_to_bool(i))
}

pub fn option_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    match s {
        Some(st) if st == "\\N" => Ok(None),
        any => Ok(any),
    }
}

pub fn option_boolean<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match <u8 as Deserialize>::deserialize(deserializer) {
        Ok(i) => Ok(Some(num_to_bool(i))),
        Err(_) => Ok(None),
    }
}

pub fn option_string_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    match <Option<String> as Deserialize>::deserialize(deserializer)?
        .filter(|s| !is_null(s))
    {
        None => Ok(None),
        Some(s) => Ok(Some(s
            .split(",")
            .map(|s| s.to_string())
            .collect()
        )),
    }
}

pub fn option_imdb_id_vec<'de, D>(deserializer: D) -> Result<Option<Vec<u32>>, D::Error>
where
    D: Deserializer<'de>,
{
    match <Option<String> as Deserialize>::deserialize(deserializer)?
        .filter(|s| !is_null(s))
    {
        None => Ok(None),
        Some(s) => Ok(Some(s
            .split(",")
            .map(|s| id_split(s, "tt"))
            .collect::<Result<Vec<u32>, <u32 as FromStr>::Err>>()
            .map_err(de::Error::custom)?
        )),
    }
}

pub fn option_name_id_vec<'de, D>(deserializer: D) -> Result<Option<Vec<u32>>, D::Error>
where
    D: Deserializer<'de>,
{
    match <Option<String> as Deserialize>::deserialize(deserializer)?
        .filter(|s| !is_null(s))
    {
        None => Ok(None),
        Some(s) => Ok(Some(s
            .split(",")
            .map(|s| id_split(s, "nm"))
            .collect::<Result<Vec<u32>, <u32 as FromStr>::Err>>()
            .map_err(de::Error::custom)?
        )),
    }
}

fn id_split<T>(s: &str, pat: &str) -> Result<T, <T as FromStr>::Err>
where
    T: FromStr,
{
    match s.starts_with(pat) {
        false => s.parse(),
        true => s.split_at(pat.len())
            .1
            .parse(),
    }
}

fn is_null(s: &str) -> bool {
    match s {
        "\\N" | "" => true,
        _ => false,
    }
}

fn num_to_bool(i: u8) -> bool {
    match i {
        0 => false,
        _ => true,
    }
}