use log::{info, debug};
use serde::Deserialize;
use serde_json::Value;
use std::{fs::read, io::{self, Stderr, Read}, ops::Not};
use std::collections::HashMap;
use regex::Regex;

#[derive(Deserialize, Debug)]
struct Notification {
    event: String,

    #[serde(default)]
    uri: String,

    #[serde(rename = "oldMailBoxID", default)]
    old_mailbox_id: String,
    
    #[serde(rename = "vnd_cmu_oldMailboxID", default)]
    vnd_cmu_old_mailbox_id: String,

    #[serde(default)]
    uidset: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let notif = read_json(io::stdin())?;

    match notif.event.as_str() {
        ev if ev.ends_with("MessageMove") || ev.ends_with("MessageCopy") => {
            eprintln!("json: {:#?}", notif);
        },
        _ => return Ok(()),
    }

    Ok(())
}

// Read JSON directly from stdin
fn read_json<R: Read>(stream: R) -> Result<Notification, Box<dyn std::error::Error>> {
    let notif: Notification = serde_json::from_reader(stream)?;
    
    debug!("Parsed Notification: {:?}", notif);

    Ok(notif)
}

fn mbox_from_uri(uri: &str) -> Result <String, &str> {
    let re = Regex::new(r"^imap://[^/]+/(.+?);").unwrap();
    
    if let Some(captures) = re.captures(uri) {
        return Ok(captures[1].to_string());
    };

    Err("No Match")
}

#[cfg(test)]
mod tests {
    use super::*;   // bring parse_uidset into scope
    use std::io::Cursor;

    #[test]
    fn correct_input() {
        let test_json = Cursor::new(b"{\"event\": \"vnd\",
                                                                \"uri\": \"imap\",
                                                                \"old_mailbox_id\": \"\",
                                                                \"vnd_cmu_old_mailbox_id\": \"\",
                                                                \"uidset\": \"1\" }");

        let result = read_json(test_json).unwrap();

        assert_eq!(result.event, "vnd");
        assert_eq!(result.uri, "imap");
        assert_eq!(result.old_mailbox_id, "");
        assert_eq!(result.vnd_cmu_old_mailbox_id, "");
        assert_eq!(result.uidset, "1");
    }

    #[test]
    fn missing_field() {
        let test_json = Cursor::new(b"{\"event\": \"vnd\",
                                                                \"old_mailbox_id\": \"\",
                                                                \"vnd_cmu_old_mailbox_id\": \"\",
                                                                \"uidset\": \"1\" }");

        let result = read_json(test_json).unwrap();

        assert_eq!(result.uri, "");
    }

    #[test]
    fn unaccounted_field() {
        let test_json = Cursor::new(b"{\"even\": \"vnd\"}");

        let err = read_json(test_json).unwrap_err();

        assert!(err.to_string().contains("missing field `event`"));
    }

    #[test]
     fn ignore_undefined_fields() {
        let test_json = Cursor::new(b"{\"event\": \"vnd\",
                                                                \"random\": \"random\"}");

        let result = read_json(test_json).unwrap();

        assert_eq!(result.event, "vnd");
    }

        #[test]
     fn _field() {
        let test_json = Cursor::new(b"{\"event\": \"vnd\",
                                                                \"random\": \"random\"}");

        let result = read_json(test_json).unwrap();
        
        assert_eq!(result.extra.len(), 1);
    }   
}