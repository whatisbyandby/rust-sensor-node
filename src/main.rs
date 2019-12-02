extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

use std::fmt::Write;
use kafka::producer::{Producer, Record, RequiredAcks};

fn main() {
    println!("Starting");
    main_loop()
}

fn main_loop() {
    loop {
        sleep(Duration::from_secs(1));
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        println!("{}", datetime.format("%d/%m/%Y %T"));
        test_producer()
    }
}

fn test_producer(){
    let mut producer =
        Producer::from_hosts(vec!("192.168.0.28:9092".to_owned()))
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

    let mut buf = String::with_capacity(2);

    let _ = write!(&mut buf, "{}", 1); // some computation of the message data to be sent
    producer.send(&Record::from_value("simple_test", buf.as_bytes())).unwrap();
//    buf.clear();
}
