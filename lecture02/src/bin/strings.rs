fn main() {
    // Rust has two string types.
    // - String: an owned, growable buffer on the heap
    // - &str  : a borrowed, fixed-size view of some text (aka string slice)
    //
    // Both string types hold a sequence of Unicode characters,
    // always stored in UTF-8 encoding, and in no other.
    {
    }

    // Creating a String.
    {
        // String::new, String::from, to_string, to_owned
    }

    // Differences between to_owned, String::from and to_string:
    // - x.to_owned()    : "give me my own copy of this borrowed value"
    // - String::from(x) : "turn this string-like value into a String"
    // - x.to_string()   : "render this value as text"


    // Growing a String.
    {
        // push, push_str, len, capacity, clear

        let mut s = String::from("Hello");
        s.push('!');
        s.push_str(" Rust");
        println!("{s}");
        println!("Lenght of s is: {}", s.len());
        println!("Capacity of s is: {}", s.capacity());
        s.clear();
        println!("{}", s.capacity());
        println!("value of S after : {s}");
        println!("Lenght of s is after clear: {}", s.len());
        println!("Capacity of s is after clear: {}", s.capacity());
    }

    // Concatenation with +.
    {
        // + clone
        let t = "foo".to_string();
        let u = "bar".to_string();
        println!("{}", t + &u);
    }

    // Building a String without printing it: format!.
    {
        let x = format!("apple {}", 42);
        println!("{x} {}", util::type_of(&x));
    }

    // The length of a string is measured in bytes.
    {
        // len, chars().count()
        let x = "Hello";
        println!("{}", x.len());
        println!("{}", x.chars().count());
    }

    // A string cannot be indexed by a number!
    {
        //let a = "Hello";
        // println!("{}", a[0]);
    }

    // Going through a string.
    {
        // chars, bytes, char_indices
        let a = "Hello";
        println!("{:?}", a.chars().nth(4));

        for c in a.chars(){
            println!("{c}");

        }

        for (i, c) in a.char_indices(){
            println!("{i} {c}");
        }

    }

    // Slicing: a part of a string is a &str.
    // A slice must never cut a character in half!
    {
        let a = "Hello".to_owned();
        let b = &a[0..2];
        println!("{b} {}", util::type_of(&b)); // He &Str
        println!("{a} {}", util::type_of(&a));
    }

    // Searching in a string.
    {
        // contains, starts_with, ends_with, find, replace
        let s = "Mississippi";
        println!("{}", s.contains("si"));
        println!("{}", s.starts_with("Mi"));
        println!("{}", s.starts_with("mi"));
        let t = "pi".to_owned();
        println!("{} {}", s.ends_with(&t), util::type_of(&t));

    }

    // Splitting a string.
    {
        // split, split_whitespace, lines
        for x in "aa,bb".split(',') {
            println!("{x}");
        }
        for x in "aa  bb\tcc".split_whitespace() {
            print!(" {x}");
        }
    }

    // Trimming, case, repetition.
    {
        // trim, trim_start, trim_end, to_uppercase, to_lowercase, repeat
    }

    // Converting text into a number: parse.
    {
        // parse, turbofish, unwrap
    }

    // Comparing strings.
    {
    }
}
