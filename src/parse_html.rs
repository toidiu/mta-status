//#![deny(warnings)]
//#![allow(unused)]
extern crate xml;
extern crate serde;
extern crate html5ever;

use self::html5ever::rcdom::NodeData;
use self::html5ever::rcdom;
use models::StatusDetail;


fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

pub fn parse_html(status_detail: &mut Vec<StatusDetail>, dom_handle: rcdom::Handle) {
    if let NodeData::Text { ref contents } = dom_handle.data {
        let text: String = escape_default(&contents.borrow());

        if !text.trim().starts_with("\\n") {
            match text.trim().as_ref() {
                "Planned Work" | "Service Change" | "Delays" => status_detail.push(
                    StatusDetail { text: String::new() }
                ),
                status_txt if !status_detail.is_empty() => {
                    let len = status_detail.len();
                    status_detail[len - 1].text.push_str(status_txt)
                }
                _ => (),
            }
        }
    }

    for child in dom_handle.children.borrow().iter() {
        parse_html(status_detail, child.clone());
    }
}
