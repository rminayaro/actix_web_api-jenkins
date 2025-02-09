use serde::Deserialize;

#[derive(Deserialize)]
pub struct AjustePrecio {
    pub porcentaje: f64,
}
