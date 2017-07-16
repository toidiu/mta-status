#![deny(warnings)]
extern crate xml;
extern crate serde;
extern crate html5ever;

use self::xml::reader::{EventReader, XmlEvent};
use self::html5ever::parse_document;
use self::html5ever::tendril::TendrilSink;
use self::html5ever::rcdom::{NodeData, RcDom, Handle};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug)]
pub struct Query {
    timestamp: String,
    lines: Vec<Line>,
}

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug)]
struct Line {
    name: String,
    status: String,
    date: String,
    time: String,
    status_detail: Vec<StatusDeatil>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct StatusDeatil {
    text: String,
}

#[derive(PartialEq)]
enum XmlTag {
    TimeStamp,
    LineName,
    LineStatus,
    LineDate,
    LineTime,
    LineText,
    Ignore
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}


fn parse_html(indent: usize, handle: Handle, status_detail: &mut Vec<StatusDeatil>) {

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

pub fn parse_xml(xml: &mut str) -> Query {
    let reader = EventReader::new(xml.as_bytes());

    let mut query = Query { ..Default::default() };
    let mut lines: Vec<Line> = Vec::new();
    let mut xml_tag: XmlTag = XmlTag::Ignore;

    let mut temp_line = Line { ..Default::default() };

    for e in reader {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                match name.local_name.as_ref() {
                    "timestamp" => {
                        xml_tag = XmlTag::TimeStamp;
                    }
                    "text" => {
                        xml_tag = XmlTag::LineText;
                    }
                    "name" => {
                        xml_tag = XmlTag::LineName;
                    }
                    "status" => {
                        xml_tag = XmlTag::LineStatus;
                    }
                    "Date" => {
                        xml_tag = XmlTag::LineDate;
                    }
                    "Time" => {
                        xml_tag = XmlTag::LineTime;
                    }
                    _ => xml_tag = XmlTag::Ignore,
                }
            }
            Ok(XmlEvent::Characters(e)) => {
                let txt: String = e;
                match xml_tag {
                    XmlTag::TimeStamp => query.timestamp = txt,
                    XmlTag::LineName => temp_line.name = txt,
                    XmlTag::LineStatus => temp_line.status = txt,
                    XmlTag::LineDate => temp_line.date = txt,
                    XmlTag::LineTime => temp_line.time = txt,
                    XmlTag::LineText => {
                        let dom = parse_document(RcDom::default(), Default::default())
                            .from_utf8()
                            .read_from(&mut txt.as_bytes())
                            .unwrap();

                        parse_html(0, dom.document, &mut temp_line.status_detail);
                    }
                    XmlTag::Ignore => (),
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_ref() {
                    "line" => {
                        lines.push(temp_line);
                        temp_line = Line { ..Default::default() };
                    }
                    "subway" => break,
                    _ => (),
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => (),
        }
    }

    query.lines = lines;
    query
}