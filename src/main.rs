pub mod macro_init;
pub mod test_clock;
pub mod clock;
pub mod test_Anagram;
pub mod Anagram;


use time::PrimitiveDateTime as DateTime;

use crate::clock::Clock;

fn main() {

    //println!("{}", add_1!(1,2));
    //println!("{}", add_1!(2));

    //println!("{}", add_as!(1, 2, u8));

    //println!("{}", add_multi!(1, 2, 3, 4, 5, 6, 7, 8, 9));

    //println!("{}", add!(1, 2, 3,4,5));

    //reverse("test");

    //let clock= Clock::new(8, 0);
    //println!("{}",clock);
    let word = "listen";
    let inputs = ["enlists", "google", "inlets", "banana"];
    Anagram::anagrams_for(word, &inputs);
}

///
/// reverse string
///
pub fn reverse(input: &str) {
    //unimplemented!("Write a function to reverse {input}");
    let val: String = input.chars().rev().collect();

    println!("{}", val);
}

///
// Returns a DateTime one billion seconds after start.
///
pub fn after(start: DateTime) {
    //unimplemented!("What time is a gigasecond later than {start}");

    let DateOut: DateTime = start + time::Duration::seconds(1000000000);
    println!("{:?}", DateOut)
}

// macro_rules! ok_or_return{
//     // internal rule.
//        (@error $a:ident,$($b:tt)* )=>{
//            {
//            match $a($($b)*) {
//                Ok(value)=>value,
//                Err(err)=>{
//                    return Err(err);
//                }
//            }
//            }
//        };

//    // public rule can be called by the user.
//        ($a:ident($($b:tt)*))=>{
//            ok_or_return!(@error $a,$($b)*)
//        };
//    }

//    fn some_work(i:i64,j:i64)->Result<(i64,i64),String>{
//        if i+j>2 {
//            Ok((i,j))
//        } else {
//            Err("error".to_owned())
//        }
//    }

//    fn main()->Result<(),String>{
//       // instead of round bracket curly brackets can also be used
//        ok_or_return!{some_work(1,4)};
//        ok_or_return!(some_work(1,0));
//        Ok(())
//    }
