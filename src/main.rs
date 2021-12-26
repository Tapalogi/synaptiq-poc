pub mod structures;

use chrono::Utc;
use structures::{SqCallInfo, SqChannelKind, SqCommandMessage, SqMessage, SqNewChannelInfo, SqPayload, SqProtocol};
use tapa_trait_serde::IJsonSerializable;
use uuid::Uuid;

pub fn print_agora_wrapped_message(title: &str, wrapped: SqProtocol) {
    println!("[{}]\n{}\n---\n", title, wrapped.to_json_string_pretty());
}

pub fn s1_demo_purge() {
    let command = SqCommandMessage::Purge;
    let payload = SqPayload::Command(command);
    let wrapped = SqProtocol {
        id: Uuid::new_v4(),
        channel_id: Uuid::new_v4(),
        sent_timestamp: Utc::now(),
        sender_id: 0,
        kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S1: Purge Command", wrapped);
}

pub fn s2_demo_remove_channel() {
    let remove_channel = SqCommandMessage::RemoveChannel(Uuid::new_v4());
    let payload = SqPayload::Command(remove_channel);
    let wrapped = SqProtocol {
        id: Uuid::new_v4(),
        channel_id: Uuid::new_v4(),
        sent_timestamp: Utc::now(),
        sender_id: 0,
        kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S2: Remove Any Channel", wrapped);
}

pub fn s2_demo_create_channel_1on1() {
    let channel_kind = SqNewChannelInfo {
        id: Uuid::new_v4(),
        alias: "Jambang Pisang".into(),
        members: vec![0, 1, 3],
        kind: SqChannelKind::PrivateMessage,
    };

    let create_channel = SqCommandMessage::NewChannel(channel_kind);
    let payload = SqPayload::Command(create_channel);
    let wrapped = SqProtocol {
        id: Uuid::new_v4(),
        channel_id: Uuid::new_v4(),
        sent_timestamp: Utc::now(),
        sender_id: 0,
        kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S2: Create Private Channel", wrapped);
}

pub fn s2_demo_create_channel_many() {
    let channel_kind = SqNewChannelInfo {
        id: Uuid::new_v4(),
        alias: "Kita Bersama".into(),
        members: vec![0, 3, 4, 5],
        kind: SqChannelKind::Group,
    };

    let create_channel = SqCommandMessage::NewChannel(channel_kind);
    let payload = SqPayload::Command(create_channel);
    let wrapped = SqProtocol {
        id: Uuid::new_v4(),
        channel_id: Uuid::new_v4(),
        sent_timestamp: Utc::now(),
        sender_id: 0,
        kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S2: Create Group Channel", wrapped);
}

pub fn s2_demo_create_channel_broadcast() {
    let channel_kind = SqNewChannelInfo {
        id: Uuid::new_v4(),
        alias: "Sekilas Info!".into(),
        members: vec![0, 3, 4, 5],
        kind: SqChannelKind::Broadcast,
    };

    let create_channel = SqCommandMessage::NewChannel(channel_kind);
    let payload = SqPayload::Command(create_channel);
    let wrapped = SqProtocol {
        id: Uuid::new_v4(),
        channel_id: Uuid::new_v4(),
        sent_timestamp: Utc::now(),
        sender_id: 0,
        kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S2: Create Broadcast Channel", wrapped);
}

pub fn s3_demo_message() {
    let new_message = SqMessage {
        content: "This a chat message with a media file embedded...".into(),
        media_url: Some("https://synaptiq.tapalogi.com/5b4e666a-d4fb-4a6d-8d89-44351f21f9c3".into()),
    };
    let payload = SqPayload::Message(new_message);
    let wrapped = SqProtocol {
        id: Uuid::new_v4(),
        channel_id: Uuid::new_v4(),
        sent_timestamp: Utc::now(),
        sender_id: 3,
        kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S3: Chat Message", wrapped);
}

pub fn s4_demo_voice_call() {
    let call_info = SqCallInfo {
        call_token: "some_funky_token".into(),
    };
    let payload = SqPayload::VoiceCall(call_info);
    let wrapped = SqProtocol {
        id: Uuid::new_v4(),
        channel_id: Uuid::new_v4(),
        sent_timestamp: Utc::now(),
        sender_id: 3,
        kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S4: Voice Call", wrapped);
}

pub fn s5_demo_video_call() {
    let call_info = SqCallInfo {
        call_token: "some_funky_token".into(),
    };
    let payload = SqPayload::VideoCall(call_info);
    let wrapped = SqProtocol {
        id: Uuid::new_v4(),
        channel_id: Uuid::new_v4(),
        sent_timestamp: Utc::now(),
        sender_id: 3,
        kind: payload.get_payload_kind(),
        payload,
    };
    print_agora_wrapped_message("S5: Video Call", wrapped);
}

pub fn s6_demo_live_streaming() {
    let call_info = SqCallInfo {
        call_token: "some_funky_token".into(),
    };
    let payload = SqPayload::LiveStream(call_info);
    let wrapped = SqProtocol {
        id: Uuid::new_v4(),
        channel_id: Uuid::new_v4(),
        sent_timestamp: Utc::now(),
        sender_id: 3,
        kind: payload.get_payload_kind(),
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
