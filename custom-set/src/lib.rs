use std::cmp::Eq;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::iter;

/*
 * We keep a fixed set of buckets, and for each object, calculate
 * its hash code to find which bucket to put it into (or find it from).
 * We use chaining to resolve collisions, i.e. for storing objects
 * that end up in the same bucket.
 * An object is added to the set if it doesn't already exist within
 * a bucket, which is determined by its equality.
 *
 * http://robertovormittag.net/how-to-implement-a-simple-hashset-in-java/
 */
#[derive(Debug, PartialEq)]
pub struct CustomSet<T: Hash + Eq + Clone> {
    buckets: Vec<Vec<T>>,
    len: u32,
}

impl<T: Hash + Eq + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        // Can't use an array of vectors because vector doesn't implement
        // Copy, which is required because the repeated element will be copied
        let mut set = CustomSet {
            buckets: iter::repeat(vec![]).take(100).collect::<Vec<_>>(),
            len: 0,
        };
        for item in input {
            set.add(item.clone());
        }
        set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.buckets[self.hash(element)].contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            let h = self.hash(&element);
            self.buckets[h].push(element);
            self.len += 1;
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.buckets
            .iter()
            .flat_map(|b| b.iter())
            .all(|e| other.contains(e))
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        self.collect_if(|e| other.contains(e))
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        self.collect_if(|e| !other.contains(e))
    }

    fn collect_if<F>(&self, f: F) -> Self
    where
        F: Fn(&T) -> bool,
    {
        let mut elements = Vec::new();
        self.buckets
            .iter()
            .flat_map(|b| b.iter())
            .filter(|e| f(e))
            .cloned()
            .for_each(|e| elements.push(e));
        CustomSet::new(&elements)
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut elements = Vec::new();
        self.buckets
            .iter()
            .flat_map(|b| b.iter())
            .chain(other.buckets.iter().flat_map(|b| b.iter()))
            .cloned()
            .for_each(|e| elements.push(e));
        CustomSet::new(&elements)
    }

    fn hash(&self, element: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }
}
