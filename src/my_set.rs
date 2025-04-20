use std::collections::HashSet;
use std::fmt;

/// `MySet<T, N>` contains at most `N` unique elements of type `T`. Types `T` are expected to
/// satisfy `MySetItem`
pub trait MySet<T, const N: usize>: fmt::Debug + Clone {
    fn new() -> Self;
    fn insert(&mut self, item: T) -> bool;
    fn len(&self) -> usize;
    // fn contains(&self, item: T) -> bool;

    /// Replace `find` with `repl` if `repl` is _not_ in the set
    fn replace(&mut self, find: T, repl: T);

    fn iter(&self) -> impl Iterator<Item = T>;
}

/// Common traits assumed for elements `T` of `MySet<T,N>`
pub trait MySetItem: Clone + Copy + PartialEq + fmt::Debug {}

#[derive(Clone, Copy)]
pub struct ArraySet<T: MySetItem, const N: usize> {
    /// Invariant: arr does not contain duplicates
    arr: [Option<T>; N],
    /// Invariant: first_none is always the index of the first None entry in the array, or N if there
    /// are no None entries
    first_none: usize,
}

impl<T: MySetItem, const N: usize> MySet<T, N> for ArraySet<T, N> {
    fn new() -> ArraySet<T, N> {
        ArraySet {
            arr: [None; N],
            first_none: 0,
        }
    }

    fn insert(&mut self, item: T) -> bool {
        for x in self.arr.iter_mut() {
            if *x == Some(item) {
                return true;
            } else if *x == None {
                *x = Some(item);
                self.first_none += 1;
                return true;
            }
        }
        return false;
    }

    fn len(&self) -> usize {
        self.first_none
    }

    // fn contains(&self, item: T) -> bool {
    //     self.arr.contains(&Some(item))
    // }

    fn replace(&mut self, find: T, repl: T) {
        for i in 0..N {
            if self.arr[i] == Some(repl) {
                // repl already in the set
                return;
            } else if self.arr[i] == Some(find) {
                // Found the item to replace
                // Check if the rest contains repl
                let mut cont = true;
                for j in i+1..N {
                    // repl found, set replacement flag to false
                    if self.arr[j] == Some(repl) { cont = false }
                }
                if cont { self.arr[i] = Some(repl) }
                return;
            } else if self.arr[i] == None {
                return;
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = T> {
        self.arr.iter().flatten().map(|item| { *item })
    }
}

impl<T: MySetItem, const N: usize> fmt::Debug for ArraySet<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.arr.fmt(f)
    }
}

// HashSet is significantly slower (about + 125% runtime) despite the shrinking. It probably just
// allocates way too much, and despite the potential lookup speed gain it's just not comparable to
// an 8 element array. Leaving this here in case I'm wrong about something
impl<T: Eq + std::hash::Hash + MySetItem, const N: usize> MySet<T, N> for HashSet<T> {
    fn new() -> Self {
        let mut ret = HashSet::new();
        ret.shrink_to(N);
        ret
    }

    fn insert(&mut self, item: T) -> bool {
        self.insert(item)
    }

    fn len(&self) -> usize {
        self.len()
    }

    // fn contains(&self, item: T) -> bool {
    //     self.contains(&item)
    // }

    fn replace(&mut self, find: T, repl: T) {
        if self.contains(&repl) { return } else {
            let removed = self.remove(&find);
            if removed {
                self.insert(repl);
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = T> {
        self.iter().map(|item| { *item })
    }
}
