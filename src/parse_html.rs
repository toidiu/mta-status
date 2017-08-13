//#![deny(warnings)]
//#![allow(unused)]
extern crate xml;
extern crate serde;
extern crate html5ever;

use self::html5ever::rcdom::{NodeData, Handle};
use parse_xml::StatusDeatil;


fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

pub fn parse_html(indent: usize, handle: Handle, status_detail: &mut Vec<StatusDeatil>) {

    match handle.data {
        NodeData::Text { ref contents }
        => {
            let text: String = format!("{}", escape_default(&contents.borrow()));

            if !text.trim().starts_with("\\n") {

                match text.trim().as_ref() {
                    "Planned Work" => status_detail.push(StatusDeatil { text: String::new() }),
                    "Service Change" => status_detail.push(StatusDeatil { text: String::new() }),
                    "Delays" => status_detail.push(StatusDeatil { text: String::new() }),
                    string   if status_detail.len() > 0 => {
                        let len = status_detail.len();
                        status_detail[len - 1].text.push_str(string)
                    }
                    _ => ()
                }
            }
        }

        _ => ()
    }

    for child in handle.children.borrow().iter() {
        parse_html(indent + 4, child.clone(), status_detail);
    }
}

