use std::env;
use std::process::exit;

fn completely_safe_storage(value: String) {
    // <- value immediately freed.
}
//creating text manipulator.
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     // for a in args {  //Note that this causes the location of args to move,
//         //so we cannot call it afterwards this scope
//     //     println!("{}", a);
//     // }
//     if args.len() != 3 {
//         eprintln!("usage: {} <op> <text>", args[0]);
//         exit(1);
//     }
//     let op = &args[1];
//     let text = &args[2];
//     let res = match op.as_str() {
//         "reverse"=>text.chars().rev().collect::<String>(), //turbofish syntax helps compiler understand
//         //which dtype you are "collecting" to
//         "invert"=>text.chars().map(|c|{
//             if c.is_uppercase() {
//                 c.to_lowercase().to_string()
//             }else {
//                 c.to_uppercase().to_string()
//             }
//         }).collect::<String>(),
//         "uppercase"=> text.to_uppercase(),
//         "acronym" => text.split_whitespace().map(|word| word.chars().next().unwrap()).collect::<String>().to_uppercase(),
//         _ => {
//             eprintln!("somethings wrong i can feel it");
//             exit(1)
//         }

//     };
//     println!("{}", res);

// }

//having both a mutable and immutable reference
// fn main(){
//     let mut bitcoin  = String::from("bitcoin");
//     let mut_ref = &mut bitcoin;

//     let ro_ref = &bitcoin;

//     println!("{}", ro_ref);
//     mut_ref.push_str(", the cryptocurrency");

// }

//returned as a reference, but since the variable as defined inside the scope
//it cannot outlive it and hence the compiler gives an error.
// fn give_me_a_ref<'a>() -> &'a String {
//     let temp = String::from("Something");
//     &temp
// }

//'a and 'b are two lifetime variables where 'a lives as long or longer than 'b.
// fn foobar <'a, 'b>(_x: &'a i32, _y: &'b i32) where 'a: 'b {
//     //code
// }

fn main() {
    let outer;
    {
        let owned = String::from("hello");
        let reference = &owned;
        outer = reference; // Error: `owned` does not live long
        //enough
        //ie we cannot "leak" the variable defined inside the scope to outside using references

        println!("{}", reference);
    } // `owned` is dropped here
    // println!("{}", outer); // Would not compile
    let mut ref1 = &5;
    {
        let x = 10;
        ref1 = &x; // ref1 now points to x
        println!("{}", ref1);
    } // x is dropped, ref1 now dangling
    // println!("{}", ref1); // Would not compile
}
fn print_ref<T: std::fmt::Display>(r: &T) {
    println!("Reference value: {}", r);
}

//using ref mut we can extract the inner "hey" and now it becomes a
//mutable reference to "hey" (in immutable reference str)

// if let Some(ref mut contents) = Some("hey") {
//     //
// }

//here Some("hey") returns a copy of the Option<&str>, so it just becomes a mutable
//reference to a copy.
// if let Some(&mut contents) = Some("hey")

// if let Some(val) = some_option {
//     // Do something with val
// }
//will only execute when some_option equals Some(val)
fn something(excuse: &mut somethingvar) {
    println!("ecxuse come");

    let y = vec![1, 2,3];
}
