mod lib;
use actix_web::{web, App, HttpServer, HttpResponse};

use serde::{Deserialize, Serialize};
use std::env;


#[derive(Debug, Serialize, Deserialize, Clone)]
struct SignBody {
    data: Vec<String>,
    private_key: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct SignedBody {
    data: Vec<String>,
    sign: String
}

#[derive(Debug, Serialize, Deserialize)]
struct SignRecover {
    pub_key: Option<String>
}

/// This handler uses json extractor
async fn sign(item: web::Json<SignBody>) -> HttpResponse {
    println!("Model: {:?}", &item);

    let cloned_data = item.0.clone();
    let signature = lib::sign_data(item.0.data, item.0.private_key.unwrap());

    HttpResponse::Ok().json(SignedBody{ data: cloned_data.data, sign: String::from("0x") + &hex::encode(signature.0) }) // <- send response
}

async fn recover(mut item: web::Json<SignedBody>) -> HttpResponse {
    println!("Model: {:?}", &item);

    let option = lib::recover_data( item.0.data, item.0.sign.split_off(2));

    HttpResponse::Ok().json(SignRecover{ pub_key: option}) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut  port: u16 = 8080;
    if args.len() == 1 {
        println!("No port number is specified. 8080 is used by default");
    } else if args.len() == 2 {
        match args[1].parse::<u16>() {
            Ok(n) => {
                port = n;
            },
            Err(e) => {
                panic!("{}", e);
            },
        }
    } else {
        panic!("Input parameter error");
    }

    println!("starting HTTP server at http://localhost:{}", port);

    HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/sign").route(web::post().to(sign)))
            .service(web::resource("/recover").route(web::post().to(recover)))
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
