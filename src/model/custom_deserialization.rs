// The following code is adapted from https://serde.rs/string-or-struct.html
// as it appears on 2021-05-29

// the `string_or_struct` function uses these impl to instantiate a Type
// if the input file contains a string and not a struct.

use serde::{Deserialize, Deserializer};
use serde::de::{self, MapAccess, Visitor};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use void::Void;

pub fn option_string_or_struct<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct OptionStringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for OptionStringOrStruct<Option<T>>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("null, string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Option<T>, E>
        where
            E: de::Error,
        {
            Ok(Some(FromStr::from_str(value).unwrap()))
        }

        fn visit_map<M>(self, map: M) -> Result<Option<T>, M::Error>
        where
            M: MapAccess<'de>
        {
            match Deserialize::deserialize(de::value::MapAccessDeserializer::new(map)) {
                Ok(res) => Ok(Some(res)),
                Err(err) => Err(err),
            }
        }
    }

    deserializer.deserialize_any(OptionStringOrStruct(PhantomData))
}

pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
