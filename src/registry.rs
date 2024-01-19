use crate::Atom;

use std::{
    collections::HashMap,
    sync::{
        Arc, Weak,
        Mutex, OnceLock
    },
};

type WeakAtom = Weak<str>;

#[derive(Clone, Debug, Default)]
pub struct AtomRegistry(Arc<Mutex<HashMap<String, WeakAtom>>>);

static ATOMS: OnceLock<AtomRegistry> = OnceLock::new();
static EMPTY: OnceLock<Atom> = OnceLock::new();

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
