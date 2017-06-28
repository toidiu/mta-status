//#![deny(warnings)]
extern crate xml;
extern crate serde;
extern crate html5ever;

use self::xml::reader::{EventReader, XmlEvent};
use self::html5ever::{parse_document, serialize};
use self::html5ever::driver::ParseOpts;
use self::html5ever::tendril::TendrilSink;
use self::html5ever::tree_builder::TreeBuilderOpts;
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


fn walk(indent: usize, handle: Handle) {
    use std::ascii::AsciiExt;
    use std::iter::repeat;

    let node = handle;
    // FIXME: don't allocate
//    print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.data {
        NodeData::Document
        => ()//println!("#Document")
        ,

        NodeData::Doctype { ref name, ref public_id, ref system_id }
        => ()//println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id)
        ,

        NodeData::Text { ref contents }
//        => println!("#text: {}", escape_default(&contents.borrow())),
        => {

            let th: String = format!("{}", escape_default(&contents.borrow()) );
            println!("{:?}", th.to_ascii_lowercase());

                if th != "\\".to_string() {
//                    println!("{}", th)
                }
            }
            ,

        NodeData::Comment { ref contents }
        => ()//println!("<!-- {} -->", escape_default(contents))
        ,

        NodeData::Element { ref name, ref attrs, .. } => {
            assert!(name.ns == ns!(html));

//            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
//                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
//            println!(">");
        }

        NodeData::ProcessingInstruction { .. } => unreachable!()
    }

    for child in node.children.borrow().iter() {
        walk(indent+4, child.clone());
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
            Ok(XmlEvent::StartElement {name, .. }) => {
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
                    XmlTag::LineName =>  if (txt == "BDFM".to_string()) {break}, //temp_line.name = txt,
                    XmlTag::LineStatus => temp_line.status = txt,
                    XmlTag::LineDate => temp_line.date = txt,
                    XmlTag::LineTime => temp_line.time = txt,
                    XmlTag::LineText => {

                        let dom = parse_document(RcDom::default(), Default::default())
                            .from_utf8()
                            .read_from(&mut txt.as_bytes())
                            .unwrap();

                      walk(0, dom.document);

                        temp_line.text = txt
                    },
                    XmlTag::Ignore => (),
                }
            }
            Ok(XmlEvent::EndElement{name}) => {
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