use std::collections::{HashMap, HashSet};

fn main() {
    // A HashMap stores values under unique keys.
    {
        // HashMap::new, insert, get, len, HashMap::from
        let mut m = HashMap::new();
        m.insert("Alice", 10);
        m.insert("Bob", 20);
        println!("{m:?} {}", util::type_of(&m));
        println!("{}", m.len());
        println!("{}", HashMap::len(&m));

        let x = HashMap::from([("a", 1), ("b", 2)]);
        println!("{x:?}");

        println!("{:?}", x.get("a"));

    }

    // get returns an Option: Some with a reference to the value, or None.
    // The keys here are Strings, but get("Alice") takes a &str: to look a key
    // up, a borrowed form of it is enough, no String has to be built for it.

    // Looping over a HashMap: the order is unspecified.
    {
        // for (k, v) in &m, for (k, v) in &mut m, keys, values

        let m = HashMap::from([("a", 1), ("b", 2)]);
        for (k, v) in &m {
            println!("{k} {v} {} {}", util::type_of(&k), util::type_of(&v));
        }
        println!("{m:?}");
    }

    // A HashMap puts each key where its hash says, so the order of the pairs
    // has nothing to do with the order of insertion, and it may even differ
    // from run to run: the hash function is seeded randomly at startup.
    // The keys cannot be modified in place, because a changed key would
    // belong to a different place in the map.
    // If the pairs are needed in key order all the time, a BTreeMap
    // (std::collections::BTreeMap) is the better choice: it has the same
    // methods (insert, get, entry, ...), but it keeps its keys sorted.

    // Checking, changing and removing a key.
    {
        // contains_key, m[k], get_mut, remove
        let mut m = HashMap::from([("a", 1), ("b", 2)]);
        println!("{}", m.contains_key("a"));
        println!("{}", m.contains_key("c"));

        println!("{}", m["a"]);

        let v = m.get_mut("b").unwrap();
        println!("{}", util::type_of(&v));
        *v = 20;
        println!("{m:?}");

        m.remove("a");
        println!("{m:?}");

        

    }

    // m[k] panics for a missing key, just like v[i] for a missing index, and
    // it can only read: a HashMap cannot be written by indexing, since m[k] = v
    // would be unclear for a missing key. To change a value, use get_mut (or
    // insert, or entry, below); to take a pair out, use remove.

    // Counting and grouping with the entry API.
    {
        // entry, or_insert
        let mut m = HashMap::from([("a", 1), ("b", 2)]);
        let e = m.entry("c");
        println!("{}", util::type_of(&e));

        let v = m.entry("c").or_insert(10);
        println!("{}", util::type_of(&v));
        println!("{m:?}");

        *m.entry("c").or_insert(10) += 1;
        println!("{m:?}");
    }

    // entry(k) finds the place of the key k, and or_insert(v) puts v there if
    // the key is missing. Either way, it returns a mutable reference to the
    // value, so *... += 1 changes the value inside the map, and .push(w)
    // adds to the Vec inside the map.

    // A HashSet stores each element at most once.
    {
        // HashSet::new, insert, contains, remove, len, for x in &s

    }

    // A HashSet is like a HashMap with keys only: insert, contains and remove
    // work the same way, and the order is unspecified here too. insert tells
    // whether the element was new. There is no indexing, since the elements
    // have no positions.

    // Set operations: &, |, - and ^.
    {
        // &a & &b, &a | &b, &a - &b, &a ^ &b, is_subset, is_disjoint
        let s1 = HashSet::from([1,2,3]);
        let s1 = HashSet::from([4, 5, 6]);
        println!("{:?}", &s1 & &s2);
        
    }

    // The operators work on references: they only read the two sets, and
    // build a new set from clones of the elements. Without the & signs, it
    // does not compile. (Each has a method form too, like a.intersection(&b),
    // but that gives an iterator, not a set.)

    // Converting between collections: collect and from_iter.
    {
        // into_iter, collect, from_iter
        let a = [2, 3, 4];
        let s:HashSet<_> = a.iter().collect();
        println!("{s:?}");
        println!("{a:?}");



    }

    // into_iter hands over the elements one by one (the source is consumed),
    // and collect builds whatever collection the type asks for, so the type
    // has to be written out. T::from_iter(x) is the same as
    // x.into_iter().collect::<T>(). For a HashSet or a HashMap, from_iter
    // still needs the type written out: these types have an extra, hidden
    // type parameter (the hash function), which from_iter leaves open.
    // A string has no into_iter: it could mean its chars or its bytes, so
    // chars() or bytes() has to be chosen.
}
