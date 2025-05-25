// zadanie 1, 3, 4

use std::cmp::Ordering;

enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}

struct Date {
    day: u8,
    month: u8,
    year: u16,
    time: Option<Time>
}

impl PartialEq for Date {
    fn eq(&self, other: &Date) -> bool {
        self.day == other.day &&
        self.month == other.month &&
        self.year == other.year &&
        self.time == other.time
    }
}

impl Eq for Date {}

impl PartialOrd for Date {
    fn partial_cmp(&self, date: &Date) -> Option<Ordering> {
        Some(self.cmp(date))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Date) -> Ordering {
        match self.year.cmp(&other.year) {
            Ordering::Equal => match self.month.cmp(&other.month) {
                Ordering::Equal => match self.day.cmp(&other.day) {
                    Ordering::Equal => self.time.cmp(&other.time),
                    ord => ord,
                },
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl Date {
    fn to_string(&self) -> String{
        if let Some(time) = &self.time {
            return format!("{}.{}.{}  {}", self.day, self.month, self.year, time.to_string());
        } else {
            return format!("{}.{}.{}", self.day, self.month, self.year);
        }        
    }

    fn from_3(day: u8, month: Month, year: u16) -> Date{
        Date {
            day: day,
            month: (month as u8) + 1,
            year: year,
            time: None
        }
    }

    fn from_string(string: &str, delim: char) -> Date{
        let string = String::from(string);
        let mut s = string.split(delim);
        let day = s.next().unwrap().parse::<u8>().unwrap();
        let month = s.next().unwrap().parse::<u8>().unwrap();
        let year = s.next().unwrap().parse::<u16>().unwrap();
        Date {day, month, year, time: None}
    }

    fn has_time(&self) -> bool {
        if let Some(time) = &self.time {
            return true;
        } else{
            return false;
        }
    }

    fn set_time(&mut self, time: Time) {
        self.time = Some(time);
    }

    fn clear_time(&mut self) {
        self.time = None;
    }
}

fn zad1(){
    let d = Date::from_3(12, Month::May, 2015);
    println!("{}", d.to_string());
    let d2 = Date::from_string("13.06.2025", '.');
    println!("{}", d2.to_string());
}

fn zad3(){
    zad1();
    println!("");
    let mut d = Date::from_3(12, Month::May, 2015);
    println!("{}", d.has_time());
    let t = Time::from_3(16, 54, 23).unwrap();
    d.set_time(t);
    println!("{:?}", d.has_time());
    println!("{}", d.to_string());
    d.clear_time();
    println!("{}", d.has_time());
}

fn zad4(){
    let mut d1 = Date { day: 15, month: 5, year: 2024, time: None };
    let mut d2 = Date { day: 15, month: 5, year: 2024, time: None };
    let mut d3 = Date { day: 30, month: 11, year: 2005, time: None };

    println!("{}", d1 == d2);
    println!("{}", d2 == d1);
    println!("{}", d1 != d3);
    println!("{}", d1 >= d2);
    println!("{}", d1 > d3);
    println!("{}", d3 < d2);

    println!("");

    d1 = Date {
        day: 15,
        month: 5,
        year: 2024,
        time: Some(Time { hour: 14, min: 30, sec: 0 }),
    };
    d2 = Date {
        day: 15,
        month: 5,
        year: 2024,
        time: Some(Time { hour: 14, min: 30, sec: 0 }),
    };
    d3 = Date {
        day: 15,
        month: 5,
        year: 2024,
        time: Some(Time { hour: 15, min: 0, sec: 0 }),
    };

    println!("{}", d1 == d2);
    println!("{}", d2 == d1);
    println!("{}", d1 != d3);
    println!("{}", d1 >= d2);
    println!("{}", d3 > d2);
    println!("{}", d1 < d3);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Time {
    hour: u8,
    min: u8,
    sec: u8
}

impl Time {
    fn to_string(&self) -> String {
        return format!("{}:{}:{}", self.hour, self.min, self.sec);
    }

    fn from_3(hour: u8, min: u8, sec: u8) -> Result<Time, String> {
        if (hour < 24) && (min < 60) && (sec < 60) {
            Ok(Time {hour: hour, min: min, sec: sec})
        } else {
            Err("bledne dane".to_string())
        }
    }

    fn from_string(string: &str, delim: char) -> Result<Time, String> {
        let string = String::from(string);
        let mut s = string.split(delim);
        let hour = s.next().unwrap().parse::<u8>().unwrap();
        let min = s.next().unwrap().parse::<u8>().unwrap();
        let sec = s.next().unwrap().parse::<u8>().unwrap();

        if (hour < 24) && (min < 60) && (sec < 60) {
            Ok(Time {hour: hour, min: min, sec: sec})
        } else {
            Err("bledne dane".to_string())
        }
    }
}

fn zad2(){
    let t = Time::from_3(16, 54, 23).unwrap();
    println!("{:?}", t.to_string());
    let t2 = Time::from_string("08:13:36", ':').unwrap();
    println!("{:?}", t2.to_string());
    let t3 = Time::from_string("25:13:36", ':').unwrap();
    println!("{:?}", t3.to_string());
}

// zadanie 5, 6

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    Low,
    Medium,
    High,
}

struct Task {
    name: String,
    descryption: String,
    priority: Priority,
    due: Date
}

impl PartialEq for Task {
    fn eq(&self, other: &Task) -> bool {
        self.name == other.name &&
        self.descryption == other.descryption &&
        self.priority == other.priority &&
        self.due == other.due 
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Task) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.due.cmp(&other.due).reverse() {
            Ordering::Equal => self.priority.cmp(&other.priority).reverse(),
            ord => ord,
        }
    }
}

fn zad5(){
    let task = Task {
        name: "nazwa".to_string(),
        descryption: "opis".to_string(),
        priority: Priority::Low,
        due: Date {day: 15, month: 5, year: 2025, time: None}
    };

    println!("{}", task.name);
    println!("{}", task.descryption);
    println!("{:?}", task.priority);
    println!("{}", task.due.to_string());
}

fn zad6(){
    let t1 = Task {
        name: "nazwa".to_string(),
        descryption: "opis".to_string(),
        priority: Priority::Low,
        due: Date {day: 15, month: 5, year: 2025, time: None}
    };
    let t2 = Task {
        name: "nazwa".to_string(),
        descryption: "opis".to_string(),
        priority: Priority::Low,
        due: Date {day: 15, month: 5, year: 2025, time: None}
    };
    let t3 = Task {
        name: "inna_nazwa".to_string(),
        descryption: "opis".to_string(),
        priority: Priority::High,
        due: Date {day: 6, month: 5, year: 2025, time: None}
    };

    println!("{}", t1 == t2);
    println!("{}", t2 == t1);
    println!("{}", t1 != t3);

    println!("");

    // zalozylam ze jezeli cos ma wyzszy priorytet albo blizsza date ukonczenia to jest wieksze
    println!("{}", t1 >= t2);
    println!("{}", t1 < t3);
    println!("{}", t3 > t2);
}

fn main() {
    // zad1();
    // zad2();
    // zad3();
    // zad4();
    // zad5();
    // zad6();
}
