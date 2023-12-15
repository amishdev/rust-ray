use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
struct RayPayload {
    r#type: &'static str,
    origin: RayOrigin,
    content: Payload,
}

type RayMeta = HashMap<String, String>;

#[derive(Serialize, Debug, Clone)]
struct RayRequest {
    uuid: String,
    payloads: Vec<RayPayload>,
    meta: RayMeta,
}

#[derive(Debug, Clone)]
enum Payload {
    Log(LogPayload),
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
impl From<LogPayload> for Payload {
    fn from(payload: LogPayload) -> Self {
        Self::Log(payload)
    }
}

impl serde::Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Payload::Log(LogPayload { values, label }) => {
                let mut payload = serializer.serialize_struct("Payload", 2)?;
                payload.serialize_field("values", values)?;
                payload.serialize_field("label", label)?;
                payload.end()
            }
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

#[derive(Serialize, Debug, Clone)]
struct LogPayload {
    values: Vec<String>,
    label: String,
}

#[derive(Serialize, Debug, Clone)]
struct TextPayload {
    content: String,
    label: String,
}

#[derive(Serialize, Debug, Clone)]
struct ColorPayload {
    color: String,
}

#[derive(Serialize, Debug, Clone)]
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

    pub fn log(&mut self, values: Vec<String>) -> &mut Self {
        let content = LogPayload {
            values,
            label: "Log".to_string(),
        };

        let payload = RayPayload {
            r#type: "log",
            content: Payload::from(content),
            origin: RayOrigin::new(),
        };

        self.request.payloads.push(payload);
        self.send();

        self
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

        let _ = task::spawn_blocking(move || {
            let client = reqwest::blocking::Client::new();
            let _ = client.post("http://localhost:23517/").json(&request).send();
        });
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
    ($arg:expr) => {{
        let mut ray = Ray::new();
        ray.log(vec![format!("{:?}", $arg)]);
        ray
    }};
    ($($arg:expr),*) => {{
        let mut ray = Ray::new();
        let mut vec = Vec::new();

       $(vec.push(format!("{:?}", $arg));)*

        ray.log(vec);

        ray
    }};
}

#[cfg(test)]
#[cfg(not(feature = "with_tokio"))]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestStruct(&'static str);

    #[test]
    fn macro_no_args() {
        let _ray = ray!();
    }

    #[test]
    fn macro_one_arg() {
        let _ray = ray!(TestStruct("One arg")).color("green");
    }

    #[test]
    fn macro_multiple_args() {
        let _ray = ray!("multiple", TestStruct("args"), vec![1, 2, 3]).color("green");
    }

    #[test]
    fn log() {
        ray!().log(vec!["log".to_string()]);
    }

    #[test]
    fn text() {
        ray!().text("text".to_string());
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

#[cfg(feature = "with_tokio")]
mod tests {
    use super::*;

    #[tokio::test]
    async fn text() {
        ray!("foobar").color("red").confetti();
    }
}
