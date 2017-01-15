use super::cmos;
use super::pit;
use time::DateTime;

pub fn init() -> DateTime {
    pit::init();

    cmos::current_datetime()
}
