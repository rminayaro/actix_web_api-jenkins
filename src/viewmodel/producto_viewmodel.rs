use actix_web::{HttpResponse, web, Responder};
use reqwest::Client;
use serde::{Serialize}; // Elimina Deserialize que no se usa
use log::{debug, error}; // Importa log
use crate::producto_count::notification::count_and_notify_products; // Importa la nueva función de notificación

#[derive(Serialize)]
struct NotificationPayload {
    app_id: String,
    contents: Contents,
    included_segments: Vec<String>,
}

#[derive(Serialize)]
struct Contents {
    en: String,
}

pub async fn send_notification(client: web::Data<Client>, message: &str) -> impl Responder { // Hacer pública esta función
    let payload = NotificationPayload {
        app_id: "9177a4cd-6fd2-4d07-aeef-eae53006ed30".to_string(),
        contents: Contents {
            en: message.to_string(),
        },
        included_segments: vec!["All".to_string()],
    };

    let res = client.post("https://onesignal.com/api/v1/notifications")
        .header("Authorization", "Basic os_v2_app_sf32jtlp2jgqplxp5lstabxngbwfkaa4p75esanzpnor72zj35unuerlgob6nabt5cjen4h4dvxgxy2jn5xz37wfnh6gwdjgdnxq4oi")
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            debug!("Notification Response: Status: {:?}, Body: {:?}", status, body);
            format!("Notification sent. Status: {:?}, Body: {:?}", status, body)
        }
        Err(e) => {
            error!("Error sending notification: {:?}", e);
            format!("Error sending notification: {:?}", e)
        }
    }
}

pub async fn get_productos(client: web::Data<Client>) -> HttpResponse {
    count_and_notify_products(client.clone()).await
}
