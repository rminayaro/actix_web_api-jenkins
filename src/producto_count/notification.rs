use crate::viewmodel::producto_viewmodel::send_notification;
use reqwest::Client;
use actix_web::{web, HttpResponse};
use serde_json::json;

pub async fn count_and_notify_products(client: web::Data<Client>) -> HttpResponse {
    let firebase_url = "https://inventoryropa-default-rtdb.firebaseio.com/Productos.json";

    match client.get(firebase_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json_data) => {
                        let cantidad = match &json_data {
                            serde_json::Value::Object(productos) => {
                                println!("Datos son un objeto JSON");
                                productos.len()
                            },
                            serde_json::Value::Array(productos) => {
                                println!("Datos son una lista JSON");
                                productos.len()
                            },
                            _ => {
                                println!("Error: Datos no son un objeto o lista JSON");
                                return HttpResponse::InternalServerError().body("Error al procesar los datos: no son un objeto o lista JSON");
                            }
                        };

                        let cantidad_modificada = if cantidad > 0 { cantidad - 1 } else { 0 };

                        let notification_message = format!("Listado de productos\nTotal de productos registrado: {}", cantidad_modificada);
                        println!("Mensaje de notificación: {}", notification_message); // Mensaje de depuración
                        let _ = send_notification(client, &notification_message).await;
                        
                        // Devuelve el JSON de los productos junto con la cantidad modificada
                        let response_data = json!({
                            "cantidad_modificada": cantidad_modificada,
                            "productos": json_data
                        });
                        HttpResponse::Ok().json(response_data)
                    },
                    Err(e) => {
                        println!("Error al procesar los datos: {:?}", e); // Mensaje de depuración
                        HttpResponse::InternalServerError().body("Error al procesar los datos")
                    },
                }
            } else {
                println!("Error al obtener datos de Firebase: {:?}", response.status()); // Mensaje de depuración
                HttpResponse::InternalServerError().body("Error al obtener datos de Firebase")
            }
        },
        Err(e) => {
            println!("Error de conexión con Firebase: {:?}", e); // Mensaje de depuración
            HttpResponse::InternalServerError().body("Error de conexión con Firebase")
        },
    }
}
