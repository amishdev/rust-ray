use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, Debug)]
struct RayPayload {
    r#type: String,
    content: String,
    origin: RayOrigin,
}

type RayMeta = HashMap<String, String>;

#[derive(Serialize, Debug)]
struct RayRequest {
    uuid: String,
    payloads: Vec<RayPayload>,
    meta: RayMeta,
}

#[derive(Serialize, Debug)]
struct TextPayload {
    content: String,
    label: String,
}

#[derive(Serialize, Debug)]
struct RayOrigin {
    function_name: String,
    r#file: String,
    line_number: String,
    hostname: String,
}
impl RayOrigin {
    pub fn new() -> Self {
        Self {
            function_name: "".to_string(),
            file: "".to_string(),
            line_number: "".to_string(),
            hostname: "".to_string(),
        }
    }
}

pub struct Ray {
    request: RayRequest,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            request: RayRequest {
                uuid: uuid::Uuid::new_v4().to_string(),
                payloads: vec![],
                meta: HashMap::new(),
            },
        }
    }

    pub fn text(&mut self, content: String) -> &mut Self {
        let content = serde_json::to_string(&TextPayload {
            content,
            label: "Text".to_string(),
        })
        .unwrap();

        let payload = RayPayload {
            r#type: "custom".to_string(),
            content,
            origin: RayOrigin::new(),
        };

        self.request.payloads.push(payload);
        self.send();

        self
    }

    pub fn confetti(&mut self) -> &mut Self {
        let payload = RayPayload {
            r#type: "confetti".to_string(),
            content: "".to_string(),
            origin: RayOrigin::new(),
        };

        self.request.payloads.push(payload);
        self.send();

        self
    }

    fn send(&mut self) {
        let client = reqwest::blocking::Client::new();
        let _ = client
            .post("http://localhost:23517/")
            .json(&self.request)
            .send();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text() {
        let mut ray = Ray::new();

        ray.text("foobar".to_string());
    }

    #[test]
    fn confetti() {
        let mut ray = Ray::new();

        ray.confetti();
    }
}
