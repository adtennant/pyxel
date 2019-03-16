use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData, time::Duration};

pub fn deserialize_as_degrees<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct DegreesVisitor;

    impl<'de> Visitor<'de> for DegreesVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number to be converted to degrees")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok((v as f64) * 90.)
        }
    }

    let visitor = DegreesVisitor;
    deserializer.deserialize_u64(visitor)
}

pub fn deserialize_as_milliseconds<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    struct MillisecondsVisitor;

    impl<'de> Visitor<'de> for MillisecondsVisitor {
        type Value = Duration;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number to be converted to milliseconds")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Duration::from_millis(v))
        }
    }

    let visitor = MillisecondsVisitor;
    deserializer.deserialize_u64(visitor)
}

pub fn deserialize_map_as_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    struct ArrayMapVisitor<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for ArrayMapVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with numbers as keys")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut vec = Vec::with_capacity(access.size_hint().unwrap_or(0));

            while let Some((key, value)) = access.next_entry::<usize, T>()? {
                vec.insert(key, value);
            }

            Ok(vec)
        }
    }

    let visitor = ArrayMapVisitor(PhantomData);
    deserializer.deserialize_map(visitor)
}

pub fn deserialize_multipliers<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct MultipliersVisitor;

    impl<'de> Visitor<'de> for MultipliersVisitor {
        type Value = Vec<f64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array of numbers to convert to multipliers")
        }

        fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut vec = Vec::with_capacity(access.size_hint().unwrap_or(0));

            while let Some(value) = access.next_element::<u64>()? {
                vec.push((value as f64) / 100.);
            }

            Ok(vec)
        }
    }

    let visitor = MultipliersVisitor;
    deserializer.deserialize_seq(visitor)
}
