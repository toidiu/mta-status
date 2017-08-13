//#![deny(warnings)]
//#![allow(unused)]
extern crate xml;
extern crate serde;
extern crate html5ever;

use self::html5ever::rcdom::{Handle, NodeData};
use parse_xml::StatusDetail;


fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

pub fn parse_html(indent: usize, handle: Handle, status_detail: &mut Vec<StatusDetail>) {
    if let NodeData::Text { ref contents } = handle.data {
        let text: String = escape_default(&contents.borrow());

        if !text.trim().starts_with("\\n") {
            match text.trim().as_ref() {
                "Planned Work" | "Service Change" | "Delays" => status_detail.push(StatusDetail {
                    text: String::new(),
                }),
                string if !status_detail.is_empty() => {
                    let len = status_detail.len();
                    status_detail[len - 1].text.push_str(string)
                }
                _ => (),
            }
        }
    }

    for child in handle.children.borrow().iter() {
        parse_html(indent + 4, child.clone(), status_detail);
    }
}
