use crate::chats::Chats;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ChatTracker(Mutex<HashMap<Arc<String>, Arc<Chats>>>);

