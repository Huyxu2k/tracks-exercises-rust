pub fn verse(n: u32) -> String {
    //unimplemented!("emit verse {n}")
    match n {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        n => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",n,n,n-1)
    }
    
}

pub fn sing(mut start: u32, end: u32) -> String {
    //unimplemented!("sing verses {start} to {end}, inclusive")
    // let mut result:String=String::default();
    // loop {
    //     result.push_str(verse(start).as_str());
    //     start=start-1;
    //     if start<end{
    //         break;
    //     }
    // }
    // result
    format!(
        "{}",
        (end..=start)
            .into_iter()
            .rev()
            .map(verse)
            .collect::<Vec<String>>()
            .join("\n")
    )
}