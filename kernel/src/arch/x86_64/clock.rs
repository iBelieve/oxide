use arch::{cmos, pit};
use spin::Mutex;
use time::DateTime;

static CURRENT_SECONDS: Mutex<u64> = Mutex::new(0);

pub fn init() {
    pit::init();

    let now = cmos::current_datetime();

    {
        let mut seconds = CURRENT_SECONDS.lock();
        *seconds = now.seconds_since_epoch();
    }

    if current_seconds() > 0 {
        println!("Clock initialized. Current time is: {}", now);
    } else {
        panic!("Clock failed to initialize");
    }
}

pub fn tick() {
    let mut seconds = CURRENT_SECONDS.lock();
    *seconds += 1;
}

pub fn current_seconds() -> u64 {
    let seconds = CURRENT_SECONDS.lock();
    *seconds
}

pub fn current_datetime() -> DateTime {
    DateTime::from_seconds_since_epoc(current_seconds())
}
