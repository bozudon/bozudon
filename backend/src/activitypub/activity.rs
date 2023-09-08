use serde::{Deserialize, Serialize};

const CONTEXT: &str = "https://www.w3.org/ns/activitystreams";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonPublicKey {
    pub id: String,
    pub owner: String,
    #[serde(rename = "publicKeyPem")]
    pub public_key_pem: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonEndpoints {
    #[serde(rename = "sharedInbox")]
    pub shared_inbox: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Activity {
    Update {
        id: String,
        actor: String,
        to: Vec<String>,
        object: Box<Activity>,
    },
    Document {
        #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
        media_type: Option<String>,
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<i64>,
    },
    Like {
        id: String,
        actor: String,
        object: String,
    },
    Announce {
        id: String,
        actor: String,
        published: String,
        to: Vec<String>,
        cc: Vec<String>,
        object: String,
    },
    Create {
        id: String,
        actor: String,
        published: String,
        to: Vec<String>,
        cc: Vec<String>,
        object: Box<Activity>,
    },
    Note {
        id: String,
        summary: Option<String>,
        published: String,
        url: String,
        #[serde(rename = "attributedTo")]
        attributed_to: String,
        to: Vec<String>,
        cc: Vec<String>,
        content: String,
        #[serde(rename = "inReplyTo")]
        in_reply_to: serde_json::Value,
        attachment: Vec<Activity>,
    },
    Image {
        url: String,
    },
    Person {
        id: String,
        following: String,
        followers: String,
        inbox: String,
        outbox: String,
        endpoints: PersonEndpoints,
        #[serde(rename = "preferredUsername")]
        preferred_username: String,
        name: String,
        summary: String,
        url: String,
        tag: Vec<String>,
        #[serde(rename = "publicKey")]
        public_key: PersonPublicKey,
        icon: Option<Box<Activity>>,
        image: Option<Box<Activity>>,
    },
    Follow {
        id: String,
        actor: String,
        object: String,
    },
    Accept {
        id: String,
        actor: String,
        object: Box<Activity>,
    },
    Undo {
        id: String,
        actor: String,
        object: Box<Activity>,
    },
    #[serde(other)]
    Unknown,
}

impl From<crate::model::account::Model> for Activity {
    fn from(m: crate::model::account::Model) -> Self {
        Activity::Person {
            id: m.uri.clone(),
            following: format!("{}/following", m.uri),
            followers: m.followers_url,
            inbox: m.inbox_url,
            endpoints: PersonEndpoints {
                shared_inbox: m.shared_inbox_url,
            },
            outbox: m.outbox_url,
            preferred_username: m.username,
            name: m.display_name,
            summary: m.note,
            url: m.uri.clone(),
            tag: vec![],
            public_key: PersonPublicKey {
                id: format!("{}/#main-key", m.uri),
                owner: m.uri.clone(),
                public_key_pem: m.public_key,
            },
            icon: None,
            image: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithContext<T> {
    #[serde(rename = "@context")]
    pub context: serde_json::Value,
    #[serde(flatten)]
    pub data: T,
}

impl<T> WithContext<T> {
    pub fn new(data: T) -> Self {
        WithContext {
            context: serde_json::Value::String(CONTEXT.to_string()),
            data,
        }
    }
}
