//! Hide credentials from debug output
//!
//! ## Example
//!
//! ```rust
//! use hide::Hide;
//!
//! #[derive(Debug)]
//! pub struct MyStruct {
//!     username: String,
//!     password: Hide<String>,
//! }
//!
//! fn example1() {
//!     let data = MyStruct {
//!         username: "user".to_string(),
//!         password: "password".to_string().into(),
//!     };
//!     println!("{data:#?}");
//! }
//! ```

use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

const SUBSTITUTE: &str = "***";

/// Wraps a type and hides it from debug output.
///
/// This also works for types which don't implement [`Debug`].
#[derive(Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Hide<T>(pub T);

impl<T> Hide<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn take(self) -> T {
        self.0
    }
}

impl<T> Deref for Hide<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Hide<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Display for Hide<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(SUBSTITUTE)
    }
}

impl<T> Debug for Hide<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(SUBSTITUTE)
    }
}

impl<T> From<T> for Hide<T> {
    fn from(value: T) -> Self {
        Hide(value)
    }
}

impl From<&str> for Hide<String> {
    fn from(value: &str) -> Self {
        Hide(value.to_string())
    }
}

impl<T> FromStr for Hide<T>
where
    T: FromStr,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hide(T::from_str(s)?))
    }
}

impl<T> AsRef<T> for Hide<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl AsRef<str> for Hide<&str> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl AsRef<str> for Hide<String> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Hide<String>> for String {
    fn from(value: Hide<String>) -> Self {
        value.0
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Hide<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Hide<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(T::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Example {
        username: String,
        password: Hide<String>,
    }

    #[test]
    fn test_simple() {
        let ex = Example {
            username: "foo".to_string(),
            password: Hide("bar".to_string()),
        };

        assert_eq!(
            format!("{ex:#?}"),
            r#"Example {
    username: "foo",
    password: ***,
}"#
        );
    }

    /// Data which misses a bunch of traits
    struct NoTraitData;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct ExampleNoTraitData {
        ntd: Hide<NoTraitData>,
    }

    #[test]
    fn test_ntr() {
        let ex = ExampleNoTraitData {
            ntd: NoTraitData.into(),
        };
        assert_eq!(
            format!("{ex:#?}"),
            r#"ExampleNoTraitData {
    ntd: ***,
}"#
        );
    }

    #[test]
    fn test_serde() {
        #[allow(dead_code)]
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        struct ExampleSerde {
            username: String,
            password: Hide<String>,
        }
        assert_eq!(
            serde_json::to_value(ExampleSerde {
                username: "foo".into(),
                password: "bar".into(),
            })
            .unwrap(),
            json!({
                "username": "foo",
                "password": "bar",
            })
        );
    }
}
