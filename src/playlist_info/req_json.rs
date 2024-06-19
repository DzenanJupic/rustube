use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContinuationReq {
    pub(crate) context: ContextSender,
    pub(crate) continuation: String,
}

impl ContinuationReq {
    pub(crate) fn new(continuation: &str) -> Self {
        Self {
            context: ContextSender {
                client: ClientSender {
                    client_name: "WEB".to_string(),
                    client_version: "2.20221124.00.00".to_string(),
                },
            },
            continuation: continuation.to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContextSender {
    pub(crate) client: ClientSender,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClientSender {
    pub(crate) client_name: String,
    pub(crate) client_version: String,
}