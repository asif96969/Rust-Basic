//use std::collections::{HashMap}
use std::collections::BTreeMap;
fn main() {
    // Exercise: word frequencies. Count how many times each word occurs in
    // the text, then print the words in alphabetical order with their counts.

    {
        let text = "the cat and the dog and the bird";
        //text.split_whitespace();
        //for
        //HashMap
        //vec
        //sort

        let mut freq = BTreeMap::new();

        for w in text.split_whitespace(){
            *freq.entry(w).or_insert(0) +=1;

        }
        println!("{freq:?}")
        
    }

    /*{
        let text = "the cat and the dog and the bird";
        let x = text.split_whitespace();
        println!("{x:?}");
        for i in x {
            println!("{}", i); 
            let mut m = HashMap::new();
            m.insert(&i);
            println!("{m:?}")
            

        }
        
        
        
    }*/

    // Exercise: the distinct elements. Print the different numbers of the
    // list in increasing order, each of them once.
    {
        //let v = vec![5, 3, 8, 3, 1, 5, 8, 8];
    }

    // Exercise: common letters. Print the letters that occur in both words.
    {
        //let a = "strawberry";
        //let b = "raspberry";
    }

    // Exercise: transposing a matrix. Build the transpose of m (its rows
    // become the columns), and print both matrices row by row.
    {
        //let m = [[1, 2, 3], [4, 5, 6]];
    }

    // Exercise: Project Euler, problem 2. Collect the Fibonacci numbers below
    // four million in a Vec, then sum the even ones.
    {
    }
}
