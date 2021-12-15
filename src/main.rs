pub mod structures;

use chrono::Utc;
use structures::{
    SynaptiqCallInfo, SynaptiqChannelInfo, SynaptiqChannelKind, SynaptiqCommandMessage, SynaptiqMessage, SynaptiqPayload,
    SynaptiqProtocol,
};
use tapa_trait_serde::IJsonSerializable;
use uuid::Uuid;

pub fn print_agora_wrapped_message(title: &str, wrapped: SynaptiqProtocol) {
    println!("[{}]\n{}\n---\n", title, wrapped.to_json_string_pretty());
}

pub fn s1_demo_purge() {
    let command = SynaptiqCommandMessage::Purge;
    let payload = SynaptiqPayload::Command(command);
    let wrapped = SynaptiqProtocol {
        timestamp: Utc::now(),
        from_id: 0,
        to_id: 3,
        payload_kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S1: Purge Command", wrapped);
}

pub fn s2_demo_remove_channel() {
    let remove_channel = SynaptiqCommandMessage::RemoveChannel(Uuid::new_v4());
    let payload = SynaptiqPayload::Command(remove_channel);
    let wrapped = SynaptiqProtocol {
        timestamp: Utc::now(),
        from_id: 0,
        to_id: 1,
        payload_kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S2: Remove Any Channel", wrapped);
}

pub fn s2_demo_create_channel_1on1() {
    let channel_kind = SynaptiqChannelInfo {
        id: Uuid::new_v4(),
        alias: "Jambang Pisang".into(),
        members: vec![0, 1, 3],
        kind: SynaptiqChannelKind::PrivateMessage,
    };

    let create_channel = SynaptiqCommandMessage::CreateChannel(channel_kind);
    let payload = SynaptiqPayload::Command(create_channel);
    let wrapped = SynaptiqProtocol {
        timestamp: Utc::now(),
        from_id: 0,
        to_id: 3,
        payload_kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S2: Create Private Channel", wrapped);
}

pub fn s2_demo_create_channel_many() {
    let channel_kind = SynaptiqChannelInfo {
        id: Uuid::new_v4(),
        alias: "Kita Bersama".into(),
        members: vec![0, 3, 4, 5],
        kind: SynaptiqChannelKind::Group,
    };

    let create_channel = SynaptiqCommandMessage::CreateChannel(channel_kind);
    let payload = SynaptiqPayload::Command(create_channel);
    let wrapped = SynaptiqProtocol {
        timestamp: Utc::now(),
        from_id: 0,
        to_id: 3,
        payload_kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S2: Create Group Channel", wrapped);
}

pub fn s2_demo_create_channel_broadcast() {
    let channel_kind = SynaptiqChannelInfo {
        id: Uuid::new_v4(),
        alias: "Sekilas Info!".into(),
        members: vec![0, 3, 4, 5],
        kind: SynaptiqChannelKind::Broadcast,
    };

    let create_channel = SynaptiqCommandMessage::CreateChannel(channel_kind);
    let payload = SynaptiqPayload::Command(create_channel);
    let wrapped = SynaptiqProtocol {
        timestamp: Utc::now(),
        from_id: 0,
        to_id: 3,
        payload_kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S2: Create Broadcast Channel", wrapped);
}

pub fn s3_demo_message() {
    let new_message = SynaptiqMessage {
        channel_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        content: "Hello Jambang Pisang!".into(),
    };
    let payload = SynaptiqPayload::Message(new_message);
    let wrapped = SynaptiqProtocol {
        timestamp: Utc::now(),
        from_id: 3,
        to_id: 1,
        payload_kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S3: Chat Message", wrapped);
}

pub fn s4_demo_voice_call() {
    let call_info = SynaptiqCallInfo {
        channel_id: Uuid::new_v4(),
        token: "some_funky_token".into(),
    };
    let payload = SynaptiqPayload::VoiceCall(call_info);
    let wrapped = SynaptiqProtocol {
        timestamp: Utc::now(),
        from_id: 3,
        to_id: 4,
        payload_kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S4: Voice Call", wrapped);
}

pub fn s5_demo_video_call() {
    let call_info = SynaptiqCallInfo {
        channel_id: Uuid::new_v4(),
        token: "some_funky_token".into(),
    };
    let payload = SynaptiqPayload::VideoCall(call_info);
    let wrapped = SynaptiqProtocol {
        timestamp: Utc::now(),
        from_id: 3,
        to_id: 4,
        payload_kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S5: Video Call", wrapped);
}

pub fn s6_demo_live_streaming() {
    let call_info = SynaptiqCallInfo {
        channel_id: Uuid::new_v4(),
        token: "some_funky_token".into(),
    };
    let payload = SynaptiqPayload::LiveStream(call_info);
    let wrapped = SynaptiqProtocol {
        timestamp: Utc::now(),
        from_id: 3,
        to_id: 4,
        payload_kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S6: Live Stream", wrapped);
}

fn main() {
    s1_demo_purge();
    s2_demo_remove_channel();
    s2_demo_create_channel_1on1();
    s2_demo_create_channel_many();
    s2_demo_create_channel_broadcast();
    s3_demo_message();
    s4_demo_voice_call();
    s5_demo_video_call();
    s6_demo_live_streaming();
}
