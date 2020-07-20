use std::fmt;
use std::hint::unreachable_unchecked;
use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Month {
    Jan,
    Feb,
    FebLeap,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Month {
    pub fn new(num: u8, leap_year: bool) -> Month {
        match num {
            1 => Month::Jan,
            2 => match leap_year {
                true => Month::FebLeap,
                false => Month::Feb,
            },
            3 => Month::Mar,
            4 => Month::Apr,
            5 => Month::May,
            6 => Month::Jun,
            7 => Month::Jul,
            8 => Month::Aug,
            9 => Month::Sep,
            10 => Month::Oct,
            11 => Month::Nov,
            12 => Month::Dec,
            _ => unsafe {
                unreachable_unchecked();
            },
        }
    }

    pub fn get_days(&self) -> u8 {
        match self {
            Month::Jan
            | Month::Mar
            | Month::May
            | Month::Jul
            | Month::Aug
            | Month::Oct
            | Month::Dec => 31,
            Month::Apr | Month::Jun | Month::Sep | Month::Nov => 30,
            Month::Feb => 28,
            Month::FebLeap => 29,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct DateInstant {
    days: u32, // Since 2000-01-01 (Y-M-D)
}

fn leap_year(year: u16) -> bool {
    match (year % 4 != 0, year % 100 != 0, year % 400 != 0) {
        (true, _, _) | (false, false, true) => false,
        _ => true,
    }
}

impl DateInstant {
    pub fn new(days: u32) -> Self {
        DateInstant { days }
    }

    pub fn from_date(year: u16, month: u8, day: u8) -> Self {
        if year < 2000 || month == 0 || month > 12 || day == 0 || day > 31 {
            panic!("{} {} {}", year, month, day);
        }
        let mut days: u32 = day as u32;
        days = (2000..year)
            .into_iter()
            .fold(days, |d, y| match leap_year(y) {
                true => d + 366,
                false => d + 365,
            });
        days = (1..month).into_iter().fold(days, |d, m| {
            d + Month::new(m, leap_year(year)).get_days() as u32
        });
        DateInstant::new(days)
    }

    pub fn to_date(mut self) -> (u16, u8, u8) {
        // If leap year and rem_days >= 366, add one year
        // If normal year and rem_days >= 365, add one year
        // Repeat
        // Check days in month, if rem_days >= days, add month
        // Repeat
        // Add remaining to days
        let mut year: u16 = 2000;
        let mut month: u8 = 1;
        let mut day: u8 = 1;

        loop {
            match (leap_year(year), self.days >= 366, self.days >= 365) {
                (true, true, _) => {
                    year += 1;
                    self.days -= 366;
                }
                (false, _, true) => {
                    year += 1;
                    self.days -= 365;
                }
                _ => break,
            }
        }

        {
            let leap = leap_year(year);
            loop {
                let days_in_month = Month::new(month, leap).get_days() as u32;
                if self.days < days_in_month {
                    break;
                }
                month += 1;
                self.days -= days_in_month;
            }
        }

        day += self.days as u8;

        (year, month, day)
    }
}

impl fmt::Display for DateInstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date = self.to_date();
        write!(f, "{}-{}-{}", date.0, date.1, date.2)
    }
}

impl Sub<DateInstant> for DateInstant {
    type Output = DateDuration;

    fn sub(self, rhs: DateInstant) -> Self::Output {
        if rhs.days < self.days {
            panic!("Date subtraction was negative: {:?} - {:?}", self, rhs);
        }
        Self::Output::new(rhs.days - self.days)
    }
}

impl Sub<DateDuration> for DateInstant {
    type Output = DateInstant;

    fn sub(self, rhs: DateDuration) -> Self::Output {
        if let Some(s) = self.days.checked_sub(rhs.days) {
            return Self::Output::new(s);
        }
        panic!("Could not subtract {:?} from {:?}", rhs, self);
    }
}

impl SubAssign<DateDuration> for DateInstant {
    fn sub_assign(&mut self, rhs: DateDuration) {
        *self = Self::sub(*self, rhs);
    }
}

impl Add<DateDuration> for DateInstant {
    type Output = Self;

    fn add(self, rhs: DateDuration) -> Self::Output {
        Self::new(self.days + rhs.days)
    }
}

impl AddAssign<DateDuration> for DateInstant {
    fn add_assign(&mut self, rhs: DateDuration) {
        *self = Self::add(*self, rhs);
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct DateDuration {
    days: u32,
}

impl DateDuration {
    pub fn new(days: u32) -> Self {
        DateDuration { days }
    }
}

impl Add for DateDuration {
    type Output = Self;

    fn add(self, rhs: DateDuration) -> Self::Output {
        Self::new(self.days + rhs.days)
    }
}

impl AddAssign for DateDuration {
    fn add_assign(&mut self, rhs: DateDuration) {
        *self = Self::add(*self, rhs);
    }
}
