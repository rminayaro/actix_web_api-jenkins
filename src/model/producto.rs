use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)] // Añadir #[derive(Debug)]
pub struct Producto {
    pub id: String,
    pub name: String,
    pub quantity: i32,
}
