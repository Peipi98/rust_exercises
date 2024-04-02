use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes : i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mins_tmp = (minutes + hours * 60).rem_euclid(60*24);
        let (hours, mins) = check_and_new(mins_tmp);
        Clock{hours, minutes:mins}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mins_tmp = (minutes + self.minutes + self.hours * 60).rem_euclid(60*24);
        let (hours, mins) = check_and_new(mins_tmp);
        Clock{hours, minutes:mins}
    }
}

/**
Accepts H:M in minutes. Please convert it into minutes.

Example:
2:10 => 2 * 60 + 10 = 130 (i32)

Args:
* `minutes (i32)` - hours and minutes converted in minutes;

Return:
* `(hours, minutes) : (i32, i32)`
**/
fn check_and_new(minutes: i32) -> (i32, i32) {
    let mins = minutes % 60;
    let hours : i32 = (minutes/ 60) % 24;
    (hours,mins)
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        /*
        let hours = if self.hours >= 10 { self.hours.to_string() }
            else {
                let mut s = String::from("0");
                s.push_str(self.hours.to_string().as_str());
                s
            };
        let minutes = if self.minutes >= 10 { self.minutes.to_string() }
        else {
            let mut s = String::from("0");
            s.push_str(self.minutes.to_string().as_str());
            s
        };
        write!(f, "{}:{}", hours, minutes)
        */
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}