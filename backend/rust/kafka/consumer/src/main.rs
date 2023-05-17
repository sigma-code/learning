use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::{info, warn};
use rdkafka::{
    config::ClientConfig,
    consumer::{stream_consumer::StreamConsumer, CommitMode, Consumer},
    message::{Headers, Message},
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("{\"msg\":\"Hello world!\"}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn init_consumer() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "demo")
        .set("bootstrap.servers", "localhost:9094")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation error");

    consumer
        .subscribe(vec!["test-topic"].as_slice())
        .expect("Error subscribing to 'test-topic'");

    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {:?}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(text)) => text,
                    Some(Err(e)) => {
                        warn!("Error deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!(
                    "Key: {:?}, payload: {}, topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                    m.key(),
                    payload,
                    m.topic(),
                    m.partition(),
                    m.offset(),
                    m.timestamp()
                );
                if let Some(headers) = m.headers() {
                    for header in headers.iter() {
                        info!("Header {:?}: {:?}", header.key, header.value)
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    actix_web::rt::spawn(async move { init_consumer().await });

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
