use lcm::{Lcm, Message};

#[derive(Clone, Debug, Message)]
pub struct Pose {
    pub utime: i64,
    pub latitude: f64,
    pub longitude: f64,
}

fn main() {
    env_logger::init();
    let mut lcm = Lcm::new().expect("Failed to init LCM");
    let buffer_size = 4096;
    // lcm.subscribe_raw(".*", buffer_size, move |chan: &str, b: &[u8]| {
    //     println!("Receieved length {} bytes from channel {}", b.len(), chan);
    // }).expect("Failed to sub");
    let callback_fn = |_chan: &str, pose: Pose| {
        println!("{:?} total messages received", pose.utime);  
    };
    lcm.subscribe::<Pose, Box<dyn Fn(&str, Pose)>>(
        "POSE",
        buffer_size,
        Box::new(callback_fn)
    )
    .expect("Failed to subscribe to POSE");
    loop {
        lcm.handle().unwrap();
    }
}
