use std::time::{self, Duration};

use std::io::Write;
use std::thread;

use chrono::prelude::*;
use env_logger::fmt::Formatter;
use env_logger::Builder;
use log::{error, info, warn, LevelFilter, Record};

use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{CommitMode, Consumer, StreamConsumer},
    message::{Header, Headers, OwnedHeaders},
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};

#[tokio::main]
async fn main() {
    setup_logger(true, Option::None);
    println!("Hello, world!");
    // for i in 0..30000{
    //     produce().await;
    // }
    
    consume(
        "10.176.120.136:9092",
        "kafka-learn-consumer-group",
        "test_topic_name",
    )
    .await;
}

async fn produce() {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "10.176.120.136:9092")
        .set("message.timeout.ms", "5000")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Producer creation error");

    let futuressync = producer
        .send(
            FutureRecord::to("test_topic_name")
                .payload(&format!(
                    "Producer Send Message {:?}",
                    time::SystemTime::now()
                ))
                .key(&format!("Key {}", "test"))
                .headers(OwnedHeaders::new().insert(Header {
                    key: "header_key",
                    value: Some("header_value"),
                })),
            Duration::from_secs(0),
        )
        .await;

    // This will be executed when the result is received.
    //println!("Delivery status for message {} received", i);

    // This loop will wait until all delivery statuses have been received.
    println!("Future completed. Result: {:?}", futuressync);
}

async fn consume(broker_server: &str, group_id: &str, topic: &str) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("client.id","test072")
        .set("bootstrap.servers", broker_server)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        .set("auto.offset.reset", "earliest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&vec![topic])
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Err(e) => error!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                // if let Some(headers) = m.headers() {
                //     for header in headers.iter() {
                //         info!("  Header {:#?}: {:?}", header.key, header.value);
                //     }
                // }
                //consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}

pub fn setup_logger(log_thread: bool, rust_log: Option<&str>) {
    let output_format = move |formatter: &mut Formatter, record: &Record| {
        let thread_name = if log_thread {
            format!("(t: {}) ", thread::current().name().unwrap_or("unknown"))
        } else {
            "".to_string()
        };

        let local_time: DateTime<Local> = Local::now();
        let time_str = local_time.format("%H:%M:%S%.3f").to_string();
        write!(
            formatter,
            "{} {}{} - {} - {}\n",
            time_str,
            thread_name,
            record.level(),
            record.target(),
            record.args()
        )
    };

    let mut builder = Builder::new();
    builder
        .format(output_format)
        .filter(None, LevelFilter::Info);

    rust_log.map(|conf| builder.parse_filters(conf));

    builder.init();
}
