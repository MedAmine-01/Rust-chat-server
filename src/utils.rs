use async_std::prelude::*;
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;
use std::net::Incoming;
use serde::de::DeserializeOwned;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_jsaon<O, P>(leaving: &mut O, packet: &P) -> ChatResult<()>
where
    O: async_std::io::Write + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');
    leaving.write_all(json.as_bytes()).await?;
    Ok(())
}
pub fn receive<I, T>(incoming: I) -> impl Stream<Item= ChatResult<T>>
where
    I: async_std::io::BufRead + Unpin,
    T: DeserializeOwned,
{
    incoming.lines().map(|line| -> chatResult<T>{
        let li = line?;
        let message = serde_json::from_str::<T>(&li)?;
        Ok(message)
    })
}
