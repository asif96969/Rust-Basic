fn main() {
    let x: i32 = -50; // i32 range = 2p31 (1 bit allocated fo sign + or -) = -2p31 to 2p31 - 1
    let y: u64 = 100; // u32 range = 2p32 = 4294967296 = 0 to 4294967295
                      // so u32 have larger maximum since it doesn't need of sign bit
                      // i16 to i128 possible and u16 to u128 possible
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);



    let pi: f64 = 3.14; // f32 and f64 possible
    println!("The value of Pi: {}", pi);

    let raining: bool = true; 
    println!("Is it raining outside? {}", raining);

    let letter: char = 'a';
    println!("first letter of alphabet: {}", letter);

    // Compound Data type: 
    // Array, tuples, Slice and String ( Slice String )

    // Array: 
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Roll number: {:?}", numbers);

    let students: [&str ; 5] = ["Asif", "Arif", "Akib", "Azad", "Ahad"];
    
    println!("Name fo Students: {:?}", students);
    println!("Name of 1st Stu: {}", students[0]);
    println!("Name of 2nd stu: {}", students[1]);

    // Tupels
    let human: (String, i32, bool)= ("Asif".to_string(), 29, true);
    println!("Profile to the Man: {:?}", human);

    let my_mix_tuples = ("Asif", 29, true, [1,2,3,4]);
    println!("My mix touple is: {:?}", my_mix_tuples);

    // String and String Slice
    let mut asif: String =String::from("Abdur ");
    println!("Asifs first name: {}", asif);
    
    asif.push_str("Rahman");
    println!("Asifs Full name is: {}", asif);

    let asif_lasname = &asif[6..12];
    println!("Asif's last name is: {}", asif_lasname);

    // Function
    hello_rust();
    mail("abdurrahman@gamil.com");
    candidate("Asif", 29, 76.5);

   

}


// No perameter
fn hello_rust(){     
    println!("Hello Rust");
}

//Single peramete
fn mail(gamail: &str) {
    println!("Asif's Mail is: {}", gamail);
}

//Multi-perameter

fn candidate(name: &str, age: u32, weight: f64){
    println!("My name is {}, I'm {} years old and My wight is {} kg", name, age, weight);
    
}

// connection with +

//{
    //let s = format!("hello {}", 42);
    //println!("{s} {}", until::type_of(&s));
//}


