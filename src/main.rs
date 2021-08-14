mod event_hub_trigger;
use actix_web::{
    middleware::Compress,
    post,
    web::{self, Bytes, Json},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;
use serde::Deserialize;
use std::env;
use std::net::Ipv4Addr;

async fn default_service(req: HttpRequest, bytes: Bytes) -> impl Responder {
    println!(
        "Received in default service {} {} with body {:?}",
        req.method(),
        req.path(),
        String::from_utf8(bytes.to_vec()).unwrap()
    );
    HttpResponse::Ok()
}

#[derive(Deserialize, Debug)]
pub(crate) struct Payload {
    pub(crate) key: String,
}

#[post("/EventHubTrigger")]
async fn event_hub_trigger_handler(
    body: Json<event_hub_trigger::EventHubTrigger<Payload>>,
) -> impl Responder {
    for value in &body.data.event_hub_messages {
        println!("Got event hub triggered {:?}", value);
    }
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = env::var(port_key)
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).build();

    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .service(event_hub_trigger_handler)
            .default_service(web::route().to(default_service))
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .run()
    .await
}
