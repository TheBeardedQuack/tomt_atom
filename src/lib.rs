use std::{
    collections::{
        hash_map::DefaultHasher,
        HashMap,
    },
    hash::{Hash, Hasher},
    ops::Deref,
    sync::{
        Arc, Weak,
        Mutex, OnceLock
    },
};

#[derive(Clone, Debug)]
pub struct Atom(Arc<str>);

type WeakAtom = Weak<str>;
type AtomRegistry = HashMap<u64, WeakAtom>;

static EMPTY: OnceLock<Atom> = OnceLock::new();
static ATOMS: OnceLock<Mutex<AtomRegistry>> = OnceLock::new();

impl Atom
{
    #[inline]
    pub fn as_str(
        &self
    ) -> &str {
        &self.0
    }

    pub fn new(
        text: &str
    ) -> Self {
        let reg = ATOMS.get_or_init(|| {
            let empty = EMPTY.get_or_init(|| "".into());
            let mut map = HashMap::new();
            map.insert(
                hash_str(&*empty),
                Arc::downgrade(&empty.0)
            );
            
            Mutex::new(map)
        });
        let mut reg_lock = reg.lock().expect("Mutex poisoned");

        let hash = hash_str(text);
        let arc = reg_lock.get(&hash)
            .and_then(Weak::upgrade)
            .unwrap_or_else(|| Arc::from(text));

        _ = reg_lock.insert(hash, Arc::downgrade(&arc));
        Self(arc)
    }
}

impl Deref
for Atom
{
    type Target = str;
    fn deref(
        &self
    ) -> &Self::Target {
        self.as_str()
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

impl<S: AsRef<str>> From<S>
for Atom
{
    fn from(
        value: S
    ) -> Self {
        Self::new(value.as_ref())
    }
}

fn hash_str(
    text: &str
) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);

    hasher.finish()
}
