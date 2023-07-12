pub fn nth(mut n: i32) -> i32 {
    let mut i = 2;
    loop {
        if isPrime(i){
            n-=1;
         }
        if  n <0{
            break;
        }
        i+=1;
    }
    i
}

fn isPrime(k: i32) -> bool {
    if k <2 {
        return  false;
    }
   for i in 2..=(k/2) {
       if k%i==0{
        return false;
       }
   }
   return  true;
}
