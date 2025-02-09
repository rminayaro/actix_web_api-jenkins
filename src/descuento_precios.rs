use actix_web::{web, HttpResponse, Responder};
use crate::viewmodel::ajuste_precio::AjustePrecio;
use crate::viewmodel::producto_viewmodel::send_notification;
use reqwest::Client;
use serde_json::Value;
use tokio::task;
use std::sync::{Arc, Mutex};

pub async fn aplicar_descuento_precios(ajuste: web::Json<AjustePrecio>, client: web::Data<Client>) -> impl Responder {
    let firebase_url = "https://inventoryropa-default-rtdb.firebaseio.com/Productos.json";
    let client_ref = client.get_ref().clone();

    match client_ref.get(firebase_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Value>().await {
                    Ok(json_data) => {
                        let ajuste_valor = ajuste.porcentaje / 100.0;
                        let json_data = Arc::new(Mutex::new(json_data));
                        let mut handles = vec![];

                        {
                            let json_data = Arc::clone(&json_data);
                            let handle = task::spawn(async move {
                                let mut data = json_data.lock().unwrap();
                                match &mut *data {
                                    Value::Array(productos) => {
                                        let chunk_size = productos.len() / 4 + 1;
                                        productos.chunks_mut(chunk_size).for_each(|chunk| {
                                            for producto in chunk.iter_mut() {
                                                if let Some(precio) = producto.get_mut("Precio") {
                                                    if let Value::Number(precio_num) = precio {
                                                        if let Some(precio_val) = precio_num.as_f64() {
                                                            let nuevo_precio = precio_val * (1.0 - ajuste_valor);
                                                            *precio = Value::from(nuevo_precio);
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                    }
                                    Value::Object(productos) => {
                                        for (_, producto) in productos.iter_mut() {
                                            if let Some(precio) = producto.get_mut("Precio") {
                                                if let Value::Number(precio_num) = precio {
                                                    if let Some(precio_val) = precio_num.as_f64() {
                                                        let nuevo_precio = precio_val * (1.0 - ajuste_valor);
                                                        *precio = Value::from(nuevo_precio);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            });
                            handles.push(handle);
                        }

                        for handle in handles {
                            let _ = handle.await;
                        }

                        let json_data = json_data.lock().unwrap().clone();
                        match client_ref.put(firebase_url).json(&json_data).send().await {
                            Ok(put_response) => {
                                if put_response.status().is_success() {
                                    let notification_message = format!("Todos los precios tuvieron un descuento de un {:.2} %", ajuste.porcentaje);
                                    let _ = send_notification(client.clone(), &notification_message).await;
                                    HttpResponse::Ok().json(&json_data)
                                } else {
                                    HttpResponse::InternalServerError().body("Error al actualizar datos en Firebase")
                                }
                            }
                            Err(_) => HttpResponse::InternalServerError().body("Error al enviar datos a Firebase"),
                        }
                    }
                    Err(e) => HttpResponse::InternalServerError().body(format!("Error al procesar los datos: {:?}", e)),
                }
            } else {
                HttpResponse::InternalServerError().body(format!("Error al obtener datos de Firebase: {:?}", response.status()))
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error de conexión con Firebase: {:?}", e)),
    }
}