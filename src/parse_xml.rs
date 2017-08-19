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

/// We use this to keep track of which part of the XML we are parsing
/// and thus generate the correct json.
#[derive(PartialEq)]
enum XmlState {
    TimeStamp,
    LineName,
    LineStatus,
    LineDate,
    LineTime,
    LineText,
    Ignore,
}

/// We are borrowing the original client response here as `&` because we only want to read
/// it without taking ownership of it. If we were to take ownership then the response could
/// not be used again (ie. logging, additional parsing).
///
/// As a general rule, attempt to take a reference if you are only processing the data
/// and don't wish to hold on to it.
pub fn parse(xml: &str) -> Query {
    let reader = EventReader::new(xml.as_bytes());

    let mut time = String::new();
    let mut lines: Vec<Line> = Vec::new();
    let mut xml_state: XmlState = XmlState::Ignore;
    let mut temp_line = Line::default();

    for e in reader {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => match name.local_name.as_ref() {
                "timestamp" => xml_state = XmlState::TimeStamp,
                "text" => xml_state = XmlState::LineText,
                "name" => xml_state = XmlState::LineName,
                "status" => xml_state = XmlState::LineStatus,
                "Date" => xml_state = XmlState::LineDate,
                "Time" => xml_state = XmlState::LineTime,
                _ => xml_state = XmlState::Ignore,
            },
            Ok(XmlEvent::Characters(e)) => {
                let txt: String = e;
                match xml_state {
                    XmlState::TimeStamp => time = txt,
                    XmlState::LineName => temp_line.name = txt,
                    XmlState::LineStatus => temp_line.status = txt,
                    XmlState::LineDate => temp_line.date = txt,
                    XmlState::LineTime => temp_line.time = txt,
                    XmlState::LineText => {
                        // This is hillariously a html text. Parse it..
                        let dom = parse_document(RcDom::default(), Default::default())
                            .from_utf8().read_from(&mut txt.as_bytes()).unwrap();
                        parse_html::parse_html(&mut temp_line.status_details, dom.document);
                    }
                    XmlState::Ignore => (),
                }
            },
            Ok(XmlEvent::EndElement { name }) => match name.local_name.as_ref() {
                "line" => {
                    // push the current temp_line onto the vector and create a new one
                    lines.push(temp_line);
                    temp_line = Line::default();
                },
                // only care about subway and not busses...
                "subway" => break,
                _ => (),
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            },
            _ => () //ignore all other events,
        }
    }

    Query {
        timestamp: time,
        lines: lines,
    }
}
