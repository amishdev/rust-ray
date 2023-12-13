use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
struct RayPayload<'a> {
    r#type: &'a str,
    origin: RayOrigin,
    content: Payload,
}

type RayMeta = HashMap<String, String>;

#[derive(Serialize, Debug)]
struct RayRequest<'a> {
    uuid: String,
    payloads: Vec<RayPayload<'a>>,
    meta: RayMeta,
}

#[derive(Debug)]
enum Payload {
    Text(TextPayload),
    Confetti,
    Color(ColorPayload),
}

impl From<TextPayload> for Payload {
    fn from(payload: TextPayload) -> Self {
        Self::Text(payload)
    }
}
impl From<ColorPayload> for Payload {
    fn from(payload: ColorPayload) -> Self {
        Self::Color(payload)
    }
}

impl serde::Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Payload::Text(TextPayload { content, label }) => {
                let mut payload = serializer.serialize_struct("Payload", 2)?;
                payload.serialize_field("content", content)?;
                payload.serialize_field("label", label)?;
                payload.end()
            }
            Payload::Color(ColorPayload { color }) => {
                let mut payload = serializer.serialize_struct("Payload", 1)?;
                payload.serialize_field("color", color)?;
                payload.end()
            }
            Payload::Confetti => {
                let payload = serializer.serialize_struct("Payload", 1)?;
                payload.end()
            }
        }
    }
}

#[derive(Serialize, Debug)]
struct TextPayload {
    content: String,
    label: String,
}

#[derive(Serialize, Debug)]
struct ColorPayload {
    color: String,
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

pub struct Ray<'a> {
    request: RayRequest<'a>,
}

impl<'a> Ray<'a> {
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
        let content = TextPayload {
            content,
            label: "Text".to_string(),
        };

        let payload = RayPayload {
            r#type: "custom",
            content: Payload::from(content),
            origin: RayOrigin::new(),
        };

        self.request.payloads.push(payload);
        self.send();

        self
    }

    pub fn color(&mut self, color: &str) -> &mut Self {
        let payload = RayPayload {
            r#type: "color",
            origin: RayOrigin::new(),
            content: Payload::Color(ColorPayload {
                color: color.to_string(),
            }),
        };

        self.request.payloads.push(payload);
        self.send();

        self
    }

    pub fn confetti(&mut self) -> &mut Self {
        let payload = RayPayload {
            r#type: "confetti",
            content: Payload::Confetti,
            origin: RayOrigin::new(),
        };

        self.request.payloads.push(payload);
        self.send();

        self
    }

    // Async version using tokio
    #[cfg(feature = "with_tokio")]
    fn send(&self) {
        use tokio::task;

        let request = self.request.clone();

        task::spawn(async move {
            let client = reqwest::Client::new();
            let _ = client.post("http://localhost:23517/").json(&request).await;
        })?;
    }

    // Blocking version without tokio
    #[cfg(not(feature = "with_tokio"))]
    fn send(&self) {
        let client = reqwest::blocking::Client::new();
        let _ = client
            .post("http://localhost:23517/")
            .json(&self.request)
            .send();
    }
}

#[macro_export]
macro_rules! ray {
    () => {{
        Ray::new()
    }};
    ($($arg:tt)*) => {{
        let mut ray = Ray::new();
        ray.text(format!("{}", format_args!($($arg)*)));
        ray
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_no_args() {
        let _ray = ray!();
    }

    #[test]
    fn macro_with_args() {
        let some_var = "test string";
        let _ray = ray!("{:?}", some_var).color("green");
    }

    #[test]
    fn text() {
        ray!("foobar");
    }

    #[test]
    fn color() {
        ray!("red").color("red");

        ray!("green").color("green");
    }

    #[test]
    fn confetti() {
        ray!().confetti();
    }
}
