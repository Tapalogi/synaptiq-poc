use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tapa_trait_serde::{IJsonSerializable, IRonSerializable};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SynaptiqChannelKind {
    Broadcast,
    Group,
    PrivateMessage,
}

#[derive(Clone, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SynaptiqChannelInfo {
    pub id: Uuid,
    pub alias: String,
    pub members: Vec<u64>,
    pub kind: SynaptiqChannelKind,
}

#[derive(Clone, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SynaptiqCommandMessage {
    Purge,
    CreateChannel(SynaptiqChannelInfo),
    RemoveChannel(Uuid),
}

#[derive(Clone, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SynaptiqMessage {
    pub channel_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SynaptiqCallInfo {
    pub channel_id: Uuid,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, IJsonSerializable, IRonSerializable, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SynaptiqPayload {
    Command(SynaptiqCommandMessage),
    LiveStream(SynaptiqCallInfo),
    Message(SynaptiqMessage),
    VideoCall(SynaptiqCallInfo),
    VoiceCall(SynaptiqCallInfo),
}

impl SynaptiqPayload {
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
pub struct SynaptiqProtocol {
    pub timestamp: DateTime<Utc>,
    pub from_id: u64,
    pub to_id: u64,
    pub payload_kind: String,
    pub payload: SynaptiqPayload,
}
