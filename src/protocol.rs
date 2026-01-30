use serde::{Deserialize, Serialize};

// --- ESTRUCTURA DEL MENSAJE ---
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub username: String,
    pub content: String,
}

impl Message {
    pub fn new(username: &str, content: &str) -> Self {
        Self {
            username: username.to_string(),
            content: content.to_string(),
        }
    }

    // Serializar: De Struct a Texto JSON (String)
    pub fn to_json(&self) -> String {
        // Añadimos un salto de línea al final para saber cuándo acaba el mensaje
        serde_json::to_string(self).unwrap() + "\n"
    }

    // Deserializar: De Texto JSON a Struct
    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}
