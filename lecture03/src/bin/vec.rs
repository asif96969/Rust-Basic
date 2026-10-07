fn main() {
    // Creating a Vec.
    {
        // vec!, Vec::new, Vec::with_capacity, Vec::from, len, capacity
    }

    // What does vec! do behind the scenes? It is a macro with three forms:
    // - vec![]        is Vec::new(): nothing is allocated until the first push.
    // - vec![1, 2, 3] builds the array [1, 2, 3], moves it to the heap with a
    //                 single allocation, and turns it into a Vec without copying.
    //                 That is why len and capacity are both 3 above.
    // - vec![x; n]    reserves room for n elements, then fills it with n - 1
    //                 clones of x and with x itself, so x must be Clone. For a
    //                 zero it asks for zeroed memory instead, which is much
    //                 faster for a big vector.

    // Adding and removing elements.
    /*{
        // push, pop, insert, remove, clear
        let v = vec![];
        v.push(10);
        v.push(20);
        println!("{v:?}");

        let mut e = v.pop();
        println!("e:?");
        v.insert(0, 30);
        v.remove(0);
        v.clear();

        println!("{v:?}");
        println!("{}", v.capacity());

    }*/

    // push and pop are fast. insert and remove have to move every element
    // after the index, so they are slow near the front of a long Vec.

    // A full Vec grows by moving to a bigger buffer.
    {
        // len, capacity, as_ptr
        let mut v = vec![42];
        for i in 0..20 {
            println!("{} {} {:?}", v.len(), v.capacity(), v.as_ptr());
            v.push(i);
        }
        println!("{v:?}");
    }

    // When the buffer is full, push allocates a new one, twice as big, copies
    // the elements over and frees the old one. (The new buffer may start at
    // the same address if the allocator could simply extend the old one.)
    // Doubling makes push fast on average, but it also means that the address
    // of the elements may change: that is why no reference into a Vec may be
    // alive during a push.

    // Reaching an element by its index: [i] or get(i).
    {
        // [i], get, first, last, match, usize
        let w = vec![10, 20, 30];
        println!("{}", w[0]);
        println!("{:?}", w.get(0));

        let i = 1_i32;
        println!("{}", w[i as usize]);

        let i = 1;
        println!("{}", w[i]);
    }

    // v[i] stops the program (panics) if i is out of range, get(i) returns
    // an Option instead: Some with a reference to the element, or None.
    // Use get when the index may be out of range, and [i] when it would be
    // a bug if it were. An index is always a usize.

    // A slice borrows a run of elements: &v[a..b].
    {
        // &v[a..b], &v[..b], &v[a..], &v[..], &mut v[a..b]
        let mut v = vec![1.5, 2.5, 3.5];
        let s = &v[0..2];
        println!("{s:?} {}", util::type_of(&s));
        let s = &v[..2];
        println!("{s:?}");
        let s = &v[1..];
        println!("{s:?}");

        let z = &mut v[0..2];
        z[0] = 42.0;
        println!("{z:?}");
        println!("{v:?}");

    }

    // A slice is a pointer and a length: it points into the buffer of v and
    // owns nothing. While a slice is alive, v cannot be modified, since a
    // push might move the buffer. A &Vec<T> or a reference to an array turns
    // into a &[T] automatically wherever a &[T] is expected.

    // Searching and sorting.
    {
        // contains, sort, dedup, reverse, is_empty
        let mut v = vec![2, 7, 3, 8, 2];
        println!("{}", v.contains(&2));
        println!("{}", v.contains(&10));
 
        v.sort(); //reverse elements in place
        println!("{v:?}");

        v.dedup(); //reverse elements in place
        println!("{v:?}");

        v.reverse(); //reverse elements in place
        println!("{v:?}");
    }

    // contains takes a reference, because it only has to look at the value
    // it compares with. sort, dedup and reverse change the Vec in place, so
    // they need a mut Vec, and return nothing.
    // Floats cannot be sorted with sort: NaN is neither smaller nor larger
    // than anything, so there is no total order among floats. Sorting them
    // needs a comparison function (sort_by), which comes later.
    // sort changes the Vec itself. For a sorted copy, clone first:
    // let mut s = v.clone(); s.sort();
    // The itertools crate (not part of the standard library) also offers a
    // one-liner like Python's sorted():
    // let s: Vec<_> = v.iter().sorted().collect();

    /*{
        let mut v = vec![1.5, 2.5, 1.0];
        v.sort(); // Floats cannot be sorted with sort
    }*/

    // Methods that keep the length (len, get, first, last, contains, sort,
    // reverse, ...) belong to slices, so they work on arrays and on parts of a
    // Vec too: v[1..3].sort(). Methods that change the length (push, pop,
    // insert, remove, clear, dedup) exist only on Vec.

    // Three ways to loop over a Vec: &v, &mut v and v.
    {
        // for x in &v, for x in &mut v, *x, for x in v

        //type 1
        let v = vec![10, 20, 30];
        for x in &v {
            println!("{x} {}", util::type_of(&x));
        }


        //type 2
        let mut w =  vec![10, 20, 30];
        for x in &mut w {
            *x +=1;
        }
        println!("{w:?}");

         //type 3
         for x in v {
            println!("{x}");
         }
         println!("{x}"); // help: a local variable with a similar name exists: `v`

    }


    // for x in &v borrows v for reading, for x in &mut v borrows it for
    // modifying, and for x in v moves v into the loop, which consumes it.
    // These are the three ways of passing any value: shared borrow, mutable
    // borrow and move. Ask for the least that is enough: &v by default,
    // &mut v to modify the elements, and v only when you are done with it.

    // A Vec cannot be changed while it is being looped over.
    {
        // extend, for i in 0..v.len()
    }

    // The loop borrows v for its whole length, and a push may move the buffer
    // (see the capacity cell above), which would leave x pointing at freed
    // memory. In many languages this is a bug found at runtime, if at all;
    // in Rust it does not compile. Fix 2 is fine for Copy elements like i32;
    // with String elements v[i] could only be borrowed or cloned.

    // A Vec owns its elements.
    {
        // push moves, vec! moves, &v[i], clone, remove
    }

    // When a Vec is dropped, all of its elements are dropped with it.
    // Moving an element out by indexing would leave a hole in the Vec, which
    // it would then try to free at its end, so it is not allowed: borrow the
    // element, clone it, or take it out with remove (or pop).
}
