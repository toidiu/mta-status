//#![deny(warnings)]
//#![allow(unused)]
extern crate xml;
extern crate serde;
extern crate html5ever;

use self::html5ever::rcdom::NodeData;
use self::html5ever::rcdom;
use models::StatusDetail;


/// This takes a `mut` parameter because we need to push `StatusDetail`s dynamically
/// and therefore modify the data. Additionally, rust will guarantee there is only
/// one mutable reference in scope and that nothing else will be modifying status_details
///
///
/// One alternative would be to `move` the entire `Line` struct here and then return it,
/// which would be clumsy since all this functions needs to do is modify the status_details
/// parameter.
pub fn parse_html(status_details: &mut Vec<StatusDetail>, dom_handle: rcdom::Handle) {
    if let NodeData::Text { ref contents } = dom_handle.data {
        let text: String = escape_default(&contents.borrow());

        if !text.trim().starts_with("\\n") {
            match text.trim().as_ref() {
                "Planned Work" | "Service Change" | "Delays" => status_details.push(
                    StatusDetail { text: String::new() }
                ),
                status_txt if !status_details.is_empty() => {
                    let len = status_details.len();
                    status_details[len - 1].text.push_str(status_txt)
                }
                _ => (),
            }
        }
    }

    for child in dom_handle.children.borrow().iter() {
        parse_html(status_details, child.clone());
    }
}

fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}
