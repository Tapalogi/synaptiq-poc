# SynaptiQ - Proof of Concept

## Data Structures Examples

```json
[S1: Purge Command]
{
  "id": "6df61ac8-ae51-4636-ae4b-f4a84a6128ea",
  "channel_id": "98bf485f-75d8-47b5-aae0-a9604abe3d95",
  "sender_id": 0,
  "sent_timestamp": "2021-12-26T13:46:03.010542951Z",
  "kind": "command",
  "payload": {
    "command": "purge"
  }
}
---

[S2: Remove Any Channel]
{
  "id": "4cd0c5d8-69c9-4e51-be05-3c760472fdad",
  "channel_id": "b26c37db-c65e-404a-bf9d-3a2bd670debb",
  "sender_id": 0,
  "sent_timestamp": "2021-12-26T13:46:03.010735783Z",
  "kind": "command",
  "payload": {
    "command": {
      "remove_channel": "905ba986-3860-46ed-95a1-0988eb1ae5f8"
    }
  }
}
---

[S2: Create Private Channel]
{
  "id": "d39a92bf-9c6c-4ac6-ab27-2ab28cf9aeb9",
  "channel_id": "ba3637b6-f61c-42c1-b653-e34455fcb935",
  "sender_id": 0,
  "sent_timestamp": "2021-12-26T13:46:03.010893760Z",
  "kind": "command",
  "payload": {
    "command": {
      "new_channel": {
        "id": "b4dbfe37-ce1d-415c-b039-1230327239e5",
        "alias": "Jambang Pisang",
        "members": [
          0,
          1,
          3
        ],
        "kind": "private_message"
      }
    }
  }
}
---

[S2: Create Group Channel]
{
  "id": "5120ce01-1917-492e-ba70-b45e89c7c146",
  "channel_id": "75cf235f-1696-4902-9aea-2a6ff4a2a7ba",
  "sender_id": 0,
  "sent_timestamp": "2021-12-26T13:46:03.011139590Z",
  "kind": "command",
  "payload": {
    "command": {
      "new_channel": {
        "id": "ebb609fb-0b87-49dc-bafd-a541477f931d",
        "alias": "Kita Bersama",
        "members": [
          0,
          3,
          4,
          5
        ],
        "kind": "group"
      }
    }
  }
}
---

[S2: Create Broadcast Channel]
{
  "id": "8f9b33e1-0a92-4aaf-827c-ec582d743307",
  "channel_id": "5f95b538-a78b-459e-b975-6e125cff24c0",
  "sender_id": 0,
  "sent_timestamp": "2021-12-26T13:46:03.011381066Z",
  "kind": "command",
  "payload": {
    "command": {
      "new_channel": {
        "id": "2160df07-1302-4a78-b9c3-c587229da934",
        "alias": "Sekilas Info!",
        "members": [
          0,
          3,
          4,
          5
        ],
        "kind": "broadcast"
      }
    }
  }
}
---

[S3: Chat Message]
{
  "id": "3da2c0e9-ca4a-48d0-ba4e-9803a69d99f1",
  "channel_id": "f32a8666-155e-49a5-9fd3-5326537d390b",
  "sender_id": 3,
  "sent_timestamp": "2021-12-26T13:46:03.011618567Z",
  "kind": "message",
  "payload": {
    "message": {
      "content": "This a chat message with a media file embedded...",
      "media_url": "https://synaptiq.tapalogi.com/5b4e666a-d4fb-4a6d-8d89-44351f21f9c3"
    }
  }
}
---

[S4: Voice Call]
{
  "id": "2866cf36-5fbb-4245-bb52-639a84acbb28",
  "channel_id": "89f0fc0d-6791-457d-9450-615da568a23e",
  "sender_id": 3,
  "sent_timestamp": "2021-12-26T13:46:03.011786273Z",
  "kind": "voice_call",
  "payload": {
    "voice_call": {
      "call_token": "some_funky_token"
    }
  }
}
---

[S5: Video Call]
{
  "id": "da003fbd-7e1d-40e0-ae80-90f195007126",
  "channel_id": "f910b3af-ee6d-4a6f-aaa1-4684e023d1ca",
  "sender_id": 3,
  "sent_timestamp": "2021-12-26T13:46:03.011936385Z",
  "kind": "video_call",
  "payload": {
    "video_call": {
      "call_token": "some_funky_token"
    }
  }
}
---

[S6: Live Stream]
{
  "id": "346d6f01-1b64-4a26-9a63-0d260722c343",
  "channel_id": "67f3bc79-7989-44f5-a1f7-f86a6bd586c9",
  "sender_id": 3,
  "sent_timestamp": "2021-12-26T13:46:03.012077851Z",
  "kind": "live_stream",
  "payload": {
    "live_stream": {
      "call_token": "some_funky_token"
    }
  }
}
---gi
```
