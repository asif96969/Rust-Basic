fn main() {
    // Exercise: Project Euler, problem 1.
    /*{
    }

    // Exercise: counting the vowels of a lowercase text.
    {
        //let text = "random apple tree text";
    }

    // Exercise: word count and the longest word.
    {
        //let text = "the quick brown fox jumps over the laziest dog";
    }*/

    // Exercise: number guessing game.
    {
        
        use rand::RngExt;
        let secret = rand::rng().random_range(1..=100u32);
        println!("{secret}");
    
        //use rand::RngExt;
        //let secret = rand::rng().random_range(1..=100u32);
    }
}
