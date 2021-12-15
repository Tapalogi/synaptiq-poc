# SynaptiQ - Proof of Concept

## Data Structures Examples

```json
[S1: Purge Command]
{
  "timestamp": "2021-12-12T09:21:35.248304222Z",
  "from_id": 0,
  "to_id": 3,
  "payload": {
    "command": "purge"
  }
}
---

[S2: Remove Any Channel]
{
  "timestamp": "2021-12-12T09:21:35.248378923Z",
  "from_id": 0,
  "to_id": 1,
  "payload": {
    "command": {
      "remove_channel": "05fe8fec-8cf6-4eba-a094-a960827df698"
    }
  }
}
---

[S2: Create Private Channel]
{
  "timestamp": "2021-12-12T09:21:35.248440414Z",
  "from_id": 0,
  "to_id": 3,
  "payload": {
    "command": {
      "create_channel": {
        "id": "93538d65-2ab5-47d0-9774-a1a684ea5866",
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
  "timestamp": "2021-12-12T09:21:35.248531343Z",
  "from_id": 0,
  "to_id": 3,
  "payload": {
    "command": {
      "create_channel": {
        "id": "9e3b7e75-59a1-4b65-89c7-ab56981b1988",
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
  "timestamp": "2021-12-12T09:21:35.248623136Z",
  "from_id": 0,
  "to_id": 3,
  "payload": {
    "command": {
      "create_channel": {
        "id": "9c0e162a-8208-40d9-a80d-a8aaeacf5676",
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
  "timestamp": "2021-12-12T09:21:35.248715775Z",
  "from_id": 3,
  "to_id": 1,
  "payload": {
    "message": {
      "id": "0599750d-bad5-42ec-b668-12369014a36f",
      "timestamp": "2021-12-12T09:21:35.248713489Z",
      "content": "Hello Jambang Pisang!"
    }
  }
}
---

[S4: Voice Call]
{
  "timestamp": "2021-12-12T09:21:35.248796786Z",
  "from_id": 3,
  "to_id": 0,
  "payload": {
    "voice_call": "5b8b2f4d-466d-4e3f-807e-64810d7f2634"
  }
}
---

[S5: Video Call]
{
  "timestamp": "2021-12-12T09:21:35.248869313Z",
  "from_id": 3,
  "to_id": 0,
  "payload": {
    "video_call": "88560b6e-062d-4250-a4e3-ea73a8344883"
  }
}
---

[S6: Live Stream]
{
  "timestamp": "2021-12-12T09:21:35.248931667Z",
  "from_id": 3,
  "to_id": 0,
  "payload": {
    "live_stream": "a5755b6a-c9ee-4092-bf41-726e1954e3d6"
  }
}
---
```
