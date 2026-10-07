fn main() {
    // Where does the data live?
    {
        // A scalar has a fixed size, so it is stored on the stack.
        // A String can grow, so its bytes are on the heap. The variable
        // itself holds only three words on the stack:
        //
        //   stack                     heap
        //   s: ptr  --------------->  h e l l o
        //      len = 5
        //      cap = 5
    }

    // The three rules of ownership.
    {
        // 1. Every value has an owner (a variable).
        // 2. There can be only one owner at a time.
        // 3. When the owner goes out of scope, the value is dropped
        //    and its heap memory is freed.
        //
        // The compiler checks all of this. There is no garbage collector,
        // and there is no manual free either.
    }

    // Scope and drop.
    {
        // println!("{s}");
        // error[E0425]: cannot find value `s` in this scope
    }

    // Assignment moves the ownership.
    {
        // println!("{s1}");
        // error[E0382]: borrow of moved value: `s1`
        //
        // Why not simply let both use it? Because then both would free the
        // same buffer at the end of the scope: that is a double free.
        // Rust invalidates the old name instead.
    }

    // Scalars are copied, not moved.
    {
        // Types that live entirely on the stack implement the Copy trait:
        // all the integers, the floats, bool and char from lecture01.
    }

    // Making a real copy: clone.
    {
    }

    // Passing a value to a function moves it.
    {
        // println!("{s}");
        // error[E0382]: borrow of moved value: `s`
    }

    // Giving the ownership back works, but it is clumsy.
    {
    }

    // Borrowing: & lends the value without giving up ownership.
    {
    }

    // A shared borrow is read only.
    {
        // r.push_str("!");
        // error[E0596]: cannot borrow `*r` as mutable, as it is behind
        //               a `&` reference
    }

    // Mutable borrow: &mut.
    {
    }

    // The borrowing rules.
    {
        // Any number of shared borrows may exist at the same time,
        // or exactly one mutable borrow, but never both kinds together.

        // let r3 = &s;
        // let m2 = &mut s;
        // println!("{r3} {m2}");
        // error[E0502]: cannot borrow `s` as mutable because it is also
        //               borrowed as immutable
        //
        // "One writer or many readers" is exactly what rules out a data race:
        // two accesses to the same data where at least one is a write.
        // Rust checks it at compile time, and this is what makes
        // concurrency in Rust "fearless".
    }

    // A borrow ends at its last use.
    {
    }

    // A reference must not outlive the value it points to.
    {
        // fn dangle() -> &String {
        //     let s = String::from("hello");
        //     &s
        // } // s is dropped here, so the returned reference would dangle
        // error[E0106]: missing lifetime specifier
        //
        // The same code in C compiles and gives a use-after-free bug.
        // In Rust the program simply does not build.
    }

    // &str is a borrowed view of a string.
    {
        // s.clear();
        // println!("{first}");
        // error[E0502]: cannot borrow `s` as mutable because it is also
        //               borrowed as immutable
    }

    // Therefore a function should take &str, not &String.
    {
    }

    // What all this buys us.
    {
        // - no dangling pointers and no use-after-free
        // - no double free
        // - no data races between threads
        // - no garbage collector, so no pauses and no runtime overhead
        //
        // The price is that the compiler says no more often. Fighting the
        // borrow checker is the normal beginner experience; it fades.
    }
}
