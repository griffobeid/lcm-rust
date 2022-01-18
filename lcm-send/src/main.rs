use std::{io::Error, thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}};

use lcm::{Lcm, Message};


#[derive(Clone, Debug, Message)]
pub struct Pose {
    pub utime: i64,
    pub latitude: f64,
    pub longitude: f64,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let mut lcm = Lcm::new().expect("Failed to init LCM");
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let pose = Pose {
        latitude: 1.0,
        longitude: 1.2,
        utime: since_the_epoch.as_micros() as i64
    };
    let mut counter: usize = 0;
    loop {
        lcm.publish::<Pose>("POSE", &pose).expect("Failed to publish LCM message");
        counter = counter + 1;
        println!("{:?} total messages sent", counter);
        sleep(Duration::from_micros(10000));
    }
}