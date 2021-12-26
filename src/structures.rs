use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tapa_trait_serde::{IJsonSerializable, IRonSerializable};
use uuid::Uuid;

#[derive(
    Clone, Copy, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize, PartialEq, Eq,
)]
#[serde(rename_all = "snake_case")]
pub enum SqChannelKind {
    Broadcast,
    Group,
    PrivateMessage,
}

#[derive(Clone, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SqNewChannelInfo {
    pub id: Uuid,
    pub alias: String,
    pub members: Vec<i32>,
    pub kind: SqChannelKind,
}

#[derive(Clone, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SqCommandMessage {
    Purge,
    NewChannel(SqNewChannelInfo),
    RemoveChannel(Uuid),
}

#[derive(Clone, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SqMessage {
    pub content: String,
    pub media_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SqCallInfo {
    pub call_token: String,
}

#[derive(Clone, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SqPayload {
    Command(SqCommandMessage),
    LiveStream(SqCallInfo),
    Message(SqMessage),
    VideoCall(SqCallInfo),
    VoiceCall(SqCallInfo),
}

impl SqPayload {
    pub fn get_payload_kind(&self) -> String {
        match self {
            Self::Command(_) => "command".into(),
            Self::LiveStream(_) => "live_stream".into(),
            Self::Message(_) => "message".into(),
            Self::VideoCall(_) => "video_call".into(),
            Self::VoiceCall(_) => "voice_call".into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SqProtocol {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub sender_id: i32,
    pub sent_timestamp: DateTime<Utc>,
    pub kind: String,
    pub payload: SqPayload,
}
