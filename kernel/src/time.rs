use core::cmp::min;
use core::fmt;

pub struct DateTime {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8
}

impl DateTime {
    // Based on http://stackoverflow.com/a/8020212/1917313
    pub fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }

    pub fn day_of_year(&self) -> u16 {
        let days = [[0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
                    [0, 0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335]];
        let leap = if self.is_leap_year() { 1 } else { 0 };

        days[leap][self.month as usize] + self.day as u16
    }

    // Based on http://stackoverflow.com/a/8020212/1917313
    pub fn seconds_since_epoch(&self) -> u64 {
        let recent_year: u32 = self.year - 1900;

        self.seconds as u64 + (self.minutes as u32 * 60) as u64 +
                (self.hours as u32 * 3600) as u64 +
                ((self.day_of_year() as u32 - 1) * 86400) as u64 +
                ((recent_year - 70) * 31536000) as u64 +
                (((recent_year - 69) / 4) * 86400) as u64 -
                (((recent_year - 1) / 100) * 86400)  as u64 +
                (((recent_year + 299) / 400) * 86400) as u64
    }

    // Based on http://stackoverflow.com/a/11197532/1917313
    pub fn from_seconds_since_epoc(seconds_since_epoch: u64) -> DateTime {
        let days_since_start = [
            [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365], // 365 days, non-leap
            [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366]  // 366 days, leap
        ];

        /*
          400 years:

          1st hundred, starting immediately after a leap year that's a multiple of 400:
          n n n l  \
          n n n l   } 24 times
          ...      /
          n n n l /
          n n n n

          2nd hundred:
          n n n l  \
          n n n l   } 24 times
          ...      /
          n n n l /
          n n n n

          3rd hundred:
          n n n l  \
          n n n l   } 24 times
          ...      /
          n n n l /
          n n n n

          4th hundred:
          n n n l  \
          n n n l   } 24 times
          ...      /
          n n n l /
          n n n L <- 97'th leap year every 400 years
        */

        // Re-bias from 1970 to 1601:
        // 1970 - 1601 = 369 = 3*100 + 17*4 + 1 years (incl. 89 leap days) =
        // (3*100*(365+24/100) + 17*4*(365+1/4) + 1*365)*24*3600 seconds
        let mut seconds = seconds_since_epoch + 11644473600;

        // dayOfWeek = (uint16_t)((seconds / 86400 + 1) % 7); // day of week

        // Remove multiples of 400 years (incl. 97 leap days)
        let quadricentennials = seconds / 12622780800; // 400*365.2425*24*3600
        seconds %= 12622780800;

        // Remove multiples of 100 years (incl. 24 leap days), can't be more than 3
        // (because multiples of 4*100=400 years (incl. leap days) have been removed)
        let centennials = min(seconds / 3155673600, 3); // 100*(365+24/100)*24*3600
        seconds -= centennials * 3155673600;

        // Remove multiples of 4 years (incl. 1 leap day), can't be more than 24
        // (because multiples of 25*4=100 years (incl. leap days) have been removed)
        let quadrennials = min(seconds / 126230400, 24); // 4*(365+1/4)*24*3600
        seconds -= quadrennials * 126230400;

        // Remove multiples of years (incl. 0 leap days), can't be more than 3
        // (because multiples of 4 years (incl. leap days) have been removed)
        let annuals = min(seconds / 31536000, 3); // 365*24*3600
        seconds -= annuals * 31536000;

        // Calculate the year and find out if it's leap
        let year = (1601 + quadricentennials * 400 + centennials * 100 + quadrennials * 4 +
                         annuals) as u32;
        let leap = if year % 4 == 0 && (year % 100 > 0 || year % 400 == 0) { 1 } else { 0 };

        // Calculate the day of the year and the time
        let day_of_year = seconds / 86400;
        seconds %= 86400;
        let hours = (seconds / 3600) as u8;
        seconds %= 3600;
        let minutes = (seconds / 60) as u8;
        seconds %= 60;

        let mut day_of_month: u8 = 0;
        let mut month: u8 = 0;

        // Calculate the month
        for index in 1..12 {
            if day_of_year < days_since_start[leap][index] {
                day_of_month = (day_of_year - days_since_start[leap][index - 1]) as u8;
                month = index as u8;
                break;
            }
        }

        DateTime { year: year, month: month, day: day_of_month, hours: hours,
                   minutes: minutes, seconds: seconds as u8 }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let is_pm = self.hours >= 12;
        let postfix = if is_pm { "PM" } else { "AM" };
        let hours = if is_pm { self.hours - 12 } else { self.hours };
        let nice_hours = if hours == 0 { 12 } else { hours };

        write!(f, "{}/{}/{} {}:{}:{} {}", self.month, self.day, self.year, nice_hours,
                self.minutes, self.seconds, postfix)
    }
}
