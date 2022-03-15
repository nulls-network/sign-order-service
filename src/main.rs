mod lib;
use actix_web::{web, App, HttpServer, HttpResponse};

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Order {
    order_no: String,
    chain_id: String,
    pay_token: String,
    pay_amount: String,
    notify: String,
    private_key: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderSign {
    order: Order,
    sign: String
}

#[derive(Debug, Serialize, Deserialize)]
struct SignRecover {
    pubKey: Option<String>
}

/// This handler uses json extractor
async fn sign(item: web::Json<Order>) -> HttpResponse {
    println!("Model: {:?}", &item);

    let cloned_order = item.0.clone();
    let signature = lib::sign_order(item.0.order_no, item.0.chain_id, item.0.pay_token, item.0.pay_amount, item.0.notify, item.0.private_key.unwrap());

    HttpResponse::Ok().json(OrderSign{ order: cloned_order, sign: hex::encode(signature.0) }) // <- send response
}

async fn recover(item: web::Json<OrderSign>) -> HttpResponse {
    println!("Model: {:?}", &item);

    let option = lib::recover_order(item.0.order.order_no, item.0.order.chain_id, item.0.order.pay_token, item.0.order.pay_amount, item.0.order.notify, item.0.sign);

    HttpResponse::Ok().json(SignRecover{ pubKey: option}) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/sign").route(web::post().to(sign)))
            .service(web::resource("/recover").route(web::post().to(recover)))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
