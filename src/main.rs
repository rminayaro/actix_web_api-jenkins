use actix_web::{web, App, HttpServer, HttpResponse};
use viewmodel::producto_viewmodel::{get_productos};
use aumento_precios::aplicar_aumento_precios;
use descuento_precios::aplicar_descuento_precios;
use env_logger;
use dotenvy::dotenv;

mod model;
mod view;
mod viewmodel;
mod producto_count;
mod aumento_precios;
mod descuento_precios;


async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Cargar variables de entorno
    env_logger::init(); // Inicializa el logger

    
    let client = reqwest::Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/health", web::get().to(health_check))
            .route("/productos", web::get().to(get_productos))
            .route("/aumentarprecio", web::post().to(aplicar_aumento_precios)) // Nueva ruta para aumentar precio
            .route("/reducirprecio", web::post().to(aplicar_descuento_precios)) // Nueva ruta para reducir precio
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
                                        