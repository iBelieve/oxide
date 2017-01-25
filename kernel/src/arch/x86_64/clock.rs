use arch::{cmos, pit};
use spin::Mutex;

static CURRENT_SECONDS: Mutex<u64> = Mutex::new(0);

pub fn init() {
    assert_has_not_been_called!("clock::init must be called only once");

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

// TODO: Hook up to our timer
// pub fn tick() {
//     let mut seconds = CURRENT_SECONDS.lock();
//     *seconds += 1;
// }

pub fn current_seconds() -> u64 {
    let seconds = CURRENT_SECONDS.lock();
    *seconds
}
