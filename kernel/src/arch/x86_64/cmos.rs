use spin::Mutex;
use time::DateTime;
use arch::io::PortPair;
use super::nmi;

const CURRENT_YEAR: u32 = 2017; // Change this each year!

static CMOS: Mutex<PortPair<u8>> = Mutex::new(unsafe { PortPair::new(0x70, 0x71) });

// TODO: Implement this and make it mut
static REGISTER_CENTURY: u8 = 0x00; // Set by ACPI table parsing code if possible

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum CMOSRegister {
    Seconds = 0x00,
    Minutes = 0x02,
    Hours = 0x04,
    Day = 0x07,
    Month = 0x08,
    Year = 0x09,
    StatusA = 0x0A,
    StatusB = 0x0B
}

#[derive(PartialEq)]
struct RTC {
    seconds: u8,
    minutes: u8,
    hours: u8,
    day: u8,
    month: u8,
    year: u8,
    century: u8
}

impl RTC {
    fn read() -> RTC {
        // Make sure an update isn't in progress
        while is_updating() {}

        RTC { seconds: read_register(CMOSRegister::Seconds),
              minutes: read_register(CMOSRegister::Minutes),
              hours: read_register(CMOSRegister::Hours),
              day: read_register(CMOSRegister::Day),
              month: read_register(CMOSRegister::Month),
              year: read_register(CMOSRegister::Year),
              century: read_century() }
    }

    fn to_datetime(&self) -> DateTime {
        let (mut seconds, mut minutes, mut hours) = (self.seconds, self.minutes, self.hours);
        let (mut day, mut month, mut year) = (self.day, self.month, self.year as u32);
        let mut century = self.century;

        let reg_b = read_register(CMOSRegister::StatusB);

        // Convert BCD to binary values if necessary
        if (reg_b & 0x04) == 0 {
            seconds = (seconds & 0x0F) + ((seconds / 16) * 10);
            minutes = (minutes & 0x0F) + ((minutes / 16) * 10);
            hours = ((hours & 0x0F) + (((hours & 0x70) / 16) * 10)) | (hours & 0x80);
            day = (day & 0x0F) + ((day / 16) * 10);
            month = (month & 0x0F) + ((month / 16) * 10);
            year = (year & 0x0F) + ((year / 16) * 10);

            if has_century() {
                century = (century & 0x0F) + ((century / 16) * 10);
            }
        }

        // Convert 12 hour clock to 24 hour clock if necessary
        if (reg_b & 0x02) == 0 && (hours & 0x80) > 0 {
            hours = ((hours & 0x7F) + 12) % 24;
        }

        // Calculate the full (4-digit) year
        if has_century() {
            year += century as u32 * 100;
        } else {
            year += CURRENT_YEAR / 100 * 100;
            if year < CURRENT_YEAR {
                year += 100;
            }
        }

        DateTime { year: year, month: month, day: day, hours: hours, minutes: minutes,
                   seconds: seconds }
    }
}

fn read_register(register: CMOSRegister) -> u8 {
    CMOS.lock().read((nmi::ENABLED << 7) | register as u8)
}

fn has_century() -> bool {
    REGISTER_CENTURY != 0
}

fn read_century() -> u8 {
    if has_century() {
        CMOS.lock().read((nmi::ENABLED << 7) | REGISTER_CENTURY)
    } else {
        0
    }
}

fn is_updating() -> bool {
    let value = read_register(CMOSRegister::StatusA);

    (value & 0x80) > 0
}

pub fn current_datetime() -> DateTime {
    let mut rtc: RTC = RTC::read();

    // Read the RTC until we get matching values (to avoid getting values during an update)
    loop {
        let new_rtc = RTC::read();

        if new_rtc == rtc {
            break;
        } else {
            rtc = new_rtc;
        }
    }

    rtc.to_datetime()
}
