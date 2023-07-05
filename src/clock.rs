use std::fmt::Display;

///
/// Clock
///
#[derive(Debug,PartialEq)]
pub struct Clock {
    hours: i128,
    minutes: i128,
}
impl Clock {
    pub fn new(hours: i128, minutes: i128) -> Clock {
        //unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");
        //change hours,minutes -> minutes in 1 day (60*24=1440)
        let mut tem: i128=hours * 60 + minutes; 
        let mut minutes_one_day=tem.rem_euclid(1440);
        // if tem>=0 && tem<=1440{
        //     minutes_one_day=1440 -tem;
        // }else if tem>1440{
        //     minutes_one_day=tem-1440;
        // }
        // else{
        //     minutes_one_day=1440 +(tem% 1440);
        // }
       
        Clock {
            hours: minutes_one_day / 60,
            minutes: minutes_one_day % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i128) -> Clock {
        //unimplemented!("Add {minutes} minutes to existing Clock time");
        Clock::new(self.hours,self.minutes+minutes)
    } 
}
impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
