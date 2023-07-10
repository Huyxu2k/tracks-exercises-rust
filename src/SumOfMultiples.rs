pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    //unimplemented!("Sum the multiples of all of {factors:?} which are less than {limit}")
    let mut vec_all:Vec<u32>=Vec::new();
    let mut sum=0;
    for i in factors {
        vec_all.append(&mut common_multiples_limit(limit,i));
    }
    vec_all.sort();
    vec_all.dedup();
    vec_all.iter().sum()
}
fn common_multiples_limit(limit: u32,number:&u32)->Vec<u32>{
   let mut su:Vec<u32>=Vec::new();
   for i in 1..limit {
       if i*number<limit{
        su.push(i*number);
       }
   }
   su
}