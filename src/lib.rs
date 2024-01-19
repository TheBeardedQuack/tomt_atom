use std::{
    collections::HashMap,
    ops::Deref,
    sync::{
        Arc, Weak,
        Mutex, OnceLock
    },
};

static EMPTY: OnceLock<Atom> = OnceLock::new();
pub static ATOMS: OnceLock<AtomRegistry> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct Atom(Arc<str>);

type WeakAtom = Weak<str>;

#[derive(Clone, Debug, Default)]
pub struct AtomRegistry(Arc<Mutex<HashMap<String, WeakAtom>>>);

impl AtomRegistry
{
    pub fn global(
        // no args
    ) -> Self {
        ATOMS.get_or_init(Self::new).clone()
    }

    pub fn new(
        // no args
    ) -> Self {
        let empty = EMPTY.get_or_init(|| Atom::from(""));
        let mut map = HashMap::new();
        map.insert(empty.to_string(), Arc::downgrade(empty));

        Self(Arc::new(Mutex::new(map)))
    }

    pub fn register<S: AsRef<str>>(
        &mut self,
        value: S
    ) -> Atom {
        let mut lock = self.0.lock()
            .expect("Mutex poisoned");

        let arc = lock.get(value.as_ref())
            .and_then(Weak::upgrade)
            .unwrap_or_else(|| {
                let s = value.as_ref().to_string();
                let arc = Arc::from(s.as_ref());
                _ = lock.insert(s, Arc::downgrade(&arc));

                arc
            });

        Atom::new(arc)
    }

    pub fn unregister<S: AsRef<str>>(
        &mut self,
        value: S
    ) {
        let mut lock = self.0.lock()
            .expect("Mutex poisoned");

        _ = lock.remove_entry(value.as_ref());
    }
}

impl Atom
{
    #[inline]
    pub fn as_str(
        &self
    ) -> &str {
        &self.0
    }

    pub(crate) fn new(
        inner: Arc<str>
    ) -> Self {
        Self(inner)
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
        Self::new(Arc::from(value))
    }
}

impl From<String>
for Atom
{
    fn from(
        value: String
    ) -> Self {
        Self::new(Arc::from(value.as_ref()))
    }
}
