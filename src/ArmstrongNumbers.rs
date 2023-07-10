

pub fn is_armstrong_number(num: u64) -> bool {
    //unimplemented!("true if {num} is an armstrong number")
    let num_to_String= num.to_string();
    let leng=num_to_String.len() as u64;
    let mut result:u64=0;

    for i in num_to_String.chars() {
        result+=exp(i.to_digit(10).unwrap_or(0).into(), leng);
    }
    if num==result{
        true
    }
    else {
        false
    }
    
}
pub fn exp(x:u64,n:u64) -> u64 {
    if n==0 {
        1
    }
    else {
        x*exp(x, n-1)
    }
}
//#[cfg(not(test))]
#[test]
fn test_exp() {
    assert_eq!((9*9*9*9), exp(9,4))
}