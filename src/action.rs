use std::{fmt, string::ToString};

use nostr_sdk::prelude::*;
use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize, Serialize,
};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Refresh,
    Error(String),
    Help,
    ReceiveEvent(nostr_sdk::Event),
    ScrollUp,
    ScrollDown,
    ScrollToTop,
    ScrollToBottom,
    React,
    SendReaction(EventId),
    Repost,
    SendRepost(EventId),
    Unselect,
}
