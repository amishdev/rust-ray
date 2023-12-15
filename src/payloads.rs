use serde::{ser::SerializeStruct, Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum Payload {
    Log(LogPayload),
    Text(TextPayload),
    Confetti,
    ClearAll,
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
            Payload::ClearAll => {
                let payload = serializer.serialize_struct("Payload", 1)?;
                payload.end()
            }
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct LogPayload {
    pub values: Vec<String>,
    pub label: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct TextPayload {
    pub content: String,
    pub label: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ColorPayload {
    pub color: String,
}
