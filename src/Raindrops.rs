pub fn raindrops(n: u32) -> String {
    //unimplemented!("what sound does Raindrop #{n} make?")
    if n%3==0 && n%5==0 && n%7==0{
       format!("PlingPlangPlong")
    }
    else if n%3==0 && n%5==0{
        format!("PlingPlang")
    }
    else if n%5==0 && n%7==0 {
        format!("PlangPlong")
    } else if n%3==0 && n%7==0{
        format!("PlingPlong")
    }else if n%3==0 {
        format!("Pling")
    } else if n%5==0 {
        format!("Plang")
    }
    else if  n%7==0 {
        format!("Plong")
    }
    else {
        format!("{}",n)
    }
}
