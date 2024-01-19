use std::{
    fmt::Display,
    ops::Deref,
    sync::Arc,
};

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq)]
pub struct Atom(Arc<str>);

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de>
for Atom{
    fn deserialize<D: serde::Deserializer<'de>> (
        deserializer: D
    ) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize
for Atom
{
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S
    ) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

impl Atom
{
    #[inline]
    pub fn as_str(
        &self
    ) -> &str {
        self.as_ref()
    }

    #[inline]
    pub fn to_string(
        &self
    ) -> String {
        self.as_str().to_string()
    }

    #[inline]
    pub fn new<S: AsRef<str>>(
        text: S
    ) -> Self {
        Self::from(text.as_ref())
    }
}

impl Deref
for Atom
{
    type Target = Arc<str>;

    fn deref(
        &self
    ) -> &Self::Target {
        &self.0
    }
}

impl Display
for Atom
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str>
for Atom
{
    fn as_ref(
        &self
    ) -> &str {
        self.0.as_ref()
    }
}

pub trait IntoAtom
{
    fn into_atom(
        self
    ) -> Atom;
}

impl<A: Into<Atom>> IntoAtom
for A {
    fn into_atom(
        self
    ) -> Atom {
        self.into()
    }
}

impl From<&str>
for Atom
{
    fn from(
        value: &str
    ) -> Self {
        Self::from(Arc::from(value))
    }
}

impl From<String>
for Atom
{
    fn from(
        value: String
    ) -> Self {
        Self::from(Arc::from(value.as_ref()))
    }
}

impl From<Arc<str>>
for Atom
{
    fn from(
        value: Arc<str>
    ) -> Self {
        Self(value)
    }
}
