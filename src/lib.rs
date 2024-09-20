use core::hash::*;
use std::collections::{
    hash_map::*,
    HashMap as _HashMap,
    HashSet as _HashSet,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct NonRandomState;

impl BuildHasher for NonRandomState {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher { Self::Hasher::default() }
}

#[derive(Debug, Clone)]
pub struct HashMap<K, V> (pub _HashMap<K, V, NonRandomState>);

#[derive(Debug, Clone)]
pub struct HashSet<T> (pub _HashSet<T, NonRandomState>);

macro_rules! implement {
    ($base: ident $id: ident, $($t:tt)*) => {
        impl<$($t)*> Default for $id<$($t)*> {
            fn default() -> Self { Self($base::default()) }
        }

        impl<$($t)*> core::ops::Deref for $id<$($t)*> {
            type Target = $base<$($t)*, NonRandomState>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$($t)*> core::ops::DerefMut for $id<$($t)*> {
            fn deref_mut(&mut self) -> &mut $base<$($t)*, NonRandomState> {
                &mut self.0
            }
        }

        impl<'a, $($t)*> core::iter::IntoIterator for &'a $id<$($t)*> {
            type Item = <&'a $base<$($t)*, NonRandomState> as core::iter::IntoIterator>::Item;
            type IntoIter = <&'a $base<$($t)*, NonRandomState> as core::iter::IntoIterator>::IntoIter;

            fn into_iter(self) -> Self::IntoIter { (&self.0).into_iter() }
        }

        impl<$($t)*> core::iter::IntoIterator for $id<$($t)*> {
            type Item = <$base<$($t)*, NonRandomState> as core::iter::IntoIterator>::Item;
            type IntoIter = <$base<$($t)*, NonRandomState> as core::iter::IntoIterator>::IntoIter;

            fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
        }

        impl<$($t)*> $id<$($t)*> {
            pub fn new() -> Self {
                Self(<$base<$($t)*, NonRandomState> as Default>::default())
            }

            pub fn with_capacity(capacity: usize) -> Self {
                Self(<$base<$($t)*, NonRandomState>>::with_capacity_and_hasher(capacity, NonRandomState::default()))
            }
        }
    };
}

// --- HashSet ---
implement!(_HashSet HashSet, T);

impl<T: Eq + Hash> core::cmp::Eq for HashSet<T> {}

impl<T: Eq + Hash> core::cmp::PartialEq<_HashSet<T, NonRandomState>> for HashSet<T> {
    fn eq(&self, other: &_HashSet<T, NonRandomState>) -> bool { self.0 == *other }
    fn ne(&self, other: &_HashSet<T, NonRandomState>) -> bool { self.0 != *other }
}

impl<T: Eq + Hash> core::cmp::PartialEq<HashSet<T>> for HashSet<T> {
    fn eq(&self, other: &HashSet<T>) -> bool { self.0 == other.0 }
    fn ne(&self, other: &HashSet<T>) -> bool { self.0 != other.0 }
}

impl<'a, T: 'a + Eq + Hash + Copy> Extend<&'a T> for HashSet<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter().cloned());
    }
}

impl<T: Eq + Hash> Extend<T> for HashSet<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<T: Eq + Hash, const N: usize> From<[T; N]> for HashSet<T> {
    fn from(value: [T; N]) -> Self {
        Self::from_iter(value)
    }
}

impl<T: Eq + Hash> FromIterator<T> for HashSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(_HashSet::from_iter(iter))
    }
}

// --- HashMap ---
implement!(_HashMap HashMap, K, V);

impl<K: Eq + Hash, V: PartialEq> core::cmp::Eq for HashMap<K, V> {}

impl<K: Eq + Hash, V: PartialEq> core::cmp::PartialEq<_HashMap<K, V, NonRandomState>> for HashMap<K, V> {
    fn eq(&self, other: &_HashMap<K, V, NonRandomState>) -> bool { self.0 == *other }
    fn ne(&self, other: &_HashMap<K, V, NonRandomState>) -> bool { self.0 != *other }
}

impl<K: Eq + Hash, V: PartialEq> core::cmp::PartialEq<HashMap<K, V>> for HashMap<K, V> {
    fn eq(&self, other: &HashMap<K, V>) -> bool { self.0 == other.0 }
    fn ne(&self, other: &HashMap<K, V>) -> bool { self.0 != other.0 }
}

impl<K, V> HashMap<K, V> {
    pub fn into_keys(self) -> IntoKeys<K, V> { self.0.into_keys() }
    pub fn into_values(self) -> IntoValues<K, V> { self.0.into_values() }
}

impl<'a, K: Eq + Hash + Copy, V: Copy> Extend<(&'a K, &'a V)> for HashMap<K, V> {
    #[inline]
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter().map(|(k, v)| (*k, *v)));
    }
}

impl<K: Eq + Hash, V> Extend<(K, V)> for HashMap<K, V> {
    #[inline]
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<K: Eq + Hash, V, const N: usize> From<[(K, V); N]> for HashMap<K, V> {
    fn from(value: [(K, V); N]) -> Self {
        Self::from_iter(value)
    }
}

impl<K: Eq + Hash, V> FromIterator<(K, V)> for HashMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self(_HashMap::from_iter(iter))
    }
}

#[cfg(test)]
#[doc(hidden)]
mod tests {
    #[test]
    fn deterministic() {
        let mut a: super::HashSet<usize> = super::HashSet::new();
        a.insert(1);
        a.insert(2);
        a.insert(3);

        let mut b: super::HashSet<usize> = super::HashSet::new();
        b.insert(1);
        b.insert(2);
        b.insert(3);

        assert!(a.into_iter().eq(b.into_iter()));
    }

    #[test]
    fn std() {
        use std::collections::HashSet;

        for _ in (0..1000).chain(core::iter::once_with(|| panic!())).take_while(|_| {
            let mut a: HashSet<usize> = HashSet::new();
            a.insert(1);
            a.insert(2);
            a.insert(3);

            let mut b: HashSet<usize> = HashSet::new();
            b.insert(1);
            b.insert(2);
            b.insert(3);

            a.into_iter().eq(b.into_iter())
        }) {}
    }
}
