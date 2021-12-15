# SynaptiQ - Proof of Concept

## Data Structures Examples

```json
[S1: Purge Command]
{
  "timestamp": "2021-12-15T20:49:51.455478306Z",
  "from_id": 0,
  "to_id": 3,
  "payload_kind": "command",
  "payload": {
    "command": "purge"
  }
}
---

[S2: Remove Any Channel]
{
  "timestamp": "2021-12-15T20:49:51.455545061Z",
  "from_id": 0,
  "to_id": 1,
  "payload_kind": "command",
  "payload": {
    "command": {
      "remove_channel": "0c9db79e-8000-4c1f-83e8-4fbbd748941e"
    }
  }
}
---

[S2: Create Private Channel]
{
  "timestamp": "2021-12-15T20:49:51.455611865Z",
  "from_id": 0,
  "to_id": 3,
  "payload_kind": "command",
  "payload": {
    "command": {
      "create_channel": {
        "id": "4d92d57f-99d7-4763-8f39-0100717c1136",
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
  "timestamp": "2021-12-15T20:49:51.455681176Z",
  "from_id": 0,
  "to_id": 3,
  "payload_kind": "command",
  "payload": {
    "command": {
      "create_channel": {
        "id": "94d2e302-a3c4-40c2-8fea-64e24530076f",
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
  "timestamp": "2021-12-15T20:49:51.455758511Z",
  "from_id": 0,
  "to_id": 3,
  "payload_kind": "command",
  "payload": {
    "command": {
      "create_channel": {
        "id": "159b35e9-b57a-4727-a0f4-497ae26e87ac",
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
  "timestamp": "2021-12-15T20:49:51.455807926Z",
  "from_id": 3,
  "to_id": 0,
  "payload_kind": "message",
  "payload": {
    "message": {
      "channel_id": "6692c9da-e7da-4883-95e9-62bfcf6241a2",
      "timestamp": "2021-12-15T20:49:51.455807101Z",
      "content": "Hello Jambang Pisang!"
    }
  }
}
---

[S4: Voice Call]
{
  "timestamp": "2021-12-15T20:49:51.455851570Z",
  "from_id": 3,
  "to_id": 0,
  "payload_kind": "voice_call",
  "payload": {
    "voice_call": {
      "channel_id": "31f5e389-8f43-426d-8224-60f66f65a4d3",
      "token": "some_funky_token"
    }
  }
}
---

[S5: Video Call]
{
  "timestamp": "2021-12-15T20:49:51.455895035Z",
  "from_id": 3,
  "to_id": 0,
  "payload_kind": "video_call",
  "payload": {
    "video_call": {
      "channel_id": "2a4bc03d-2cb2-4f3e-b6a9-dee1bc4e8fd8",
      "token": "some_funky_token"
    }
  }
}
---

[S6: Live Stream]
{
  "timestamp": "2021-12-15T20:49:51.455936155Z",
  "from_id": 3,
  "to_id": 0,
  "payload_kind": "live_stream",
  "payload": {
    "live_stream": {
      "channel_id": "7cbdd2c0-4ad4-4fdf-9056-1220a4c9d519",
      "token": "some_funky_token"
    }
  }
}
---
```
