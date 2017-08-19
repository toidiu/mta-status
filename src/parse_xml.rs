//#![deny(warnings)]
//#![allow(unused)]
extern crate xml;
extern crate serde;
extern crate html5ever;

use self::xml::reader::{EventReader, XmlEvent};
use self::html5ever::parse_document;
use self::html5ever::tendril::TendrilSink;
use self::html5ever::rcdom::RcDom;
use parse_html;
use models::*;

#[derive(PartialEq)]
enum XmlTag {
    TimeStamp,
    LineName,
    LineStatus,
    LineDate,
    LineTime,
    LineText,
    Ignore,
}

pub fn parse(xml: &str) -> Query {
    let reader = EventReader::new(xml.as_bytes());

    let mut time = String::new();
    let mut lines: Vec<Line> = Vec::new();
    let mut xml_tag: XmlTag = XmlTag::Ignore;
    let mut temp_line = Line {
        ..Default::default()
    };

    for e in reader {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => match name.local_name.as_ref() {
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
            },
            Ok(XmlEvent::Characters(e)) => {
                let txt: String = e;
                match xml_tag {
                    XmlTag::TimeStamp => time = txt,
                    XmlTag::LineName => temp_line.name = txt,
                    XmlTag::LineStatus => temp_line.status = txt,
                    XmlTag::LineDate => temp_line.date = txt,
                    XmlTag::LineTime => temp_line.time = txt,
                    XmlTag::LineText => {
                        let dom = parse_document(RcDom::default(), Default::default())
                            .from_utf8()
                            .read_from(&mut txt.as_bytes())
                            .unwrap();

                        parse_html::parse_html(&mut temp_line.status_detail, dom.document);
                    }
                    XmlTag::Ignore => (),
                }
            }
            Ok(XmlEvent::EndElement { name }) => match name.local_name.as_ref() {
                "line" => {
                    lines.push(temp_line);
                    temp_line = Line {
                        ..Default::default()
                    };
                }
                "subway" => break,
                _ => (),
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => (),
        }
    }

    Query {
        timestamp: time,
        lines: lines,
    }
}
