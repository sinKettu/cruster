use super::request_response;
use hudsucker::{
    tokio_tungstenite::tungstenite::Message,
    WebSocketContext
};
use crate::utils::CrusterError;

pub(crate) enum ProxyEvents {
    RequestSent((request_response::HyperRequestWrapper, usize)),
    ResponseSent((request_response::HyperResponseWrapper, usize)),
    WebSocketMessageSent((WebSocketContext, Message)),
    Error((CrusterError, Option<usize>))
}
