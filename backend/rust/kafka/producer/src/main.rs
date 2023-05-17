use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use rdkafka::{
    config::ClientConfig,
    message::{Header, OwnedHeaders},
    producer::{FutureProducer, FutureRecord},
};
use std::time::Duration;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("{\"msg\":\"Hello world!\"}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/send")]
async fn send(req_body: String, producer: web::Data<FutureProducer>) -> impl Responder {
    let delivery_status = producer
        .send(
            FutureRecord::to("test-topic")
                .payload(&format!("Message: {}", req_body))
                .key("test")
                .headers(OwnedHeaders::new().insert(Header {
                    key: "header_key",
                    value: Some("header_value"),
                })),
            Duration::from_secs(0),
        )
        .await
        .expect("Error on delivery_status");

    HttpResponse::Ok().body(format!("Delivery Status: {:?}", delivery_status))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9094")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .service(send)
            .app_data(web::Data::new(producer.clone()))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
