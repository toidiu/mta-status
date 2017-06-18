//#![deny(warnings)]
//extern crate xml;
//extern crate serde;
//
//use self::xml::reader::{EventReader, XmlEvent};
//
//#[derive(Serialize, Deserialize)]
//#[derive(Default, Debug)]
//pub struct Query {
//    timestamp: String,
//    lines: Vec<Line>,
//}
//
//#[derive(Serialize, Deserialize)]
//#[derive(Default, Debug)]
//struct Line {
//    name: String,
//    status: String,
//    date: String,
//    time: String,
//}
//
//#[derive(PartialEq)]
//enum XmlTag {
//    TimeStamp,
//    LineName,
//    LineStatus,
//    LineDate,
//    LineTime,
//    Ignore
//}
//
//pub fn parse_xml(xml: &mut str) -> Query {
//    let mut reader = EventReader::from_str(&xml);
//    reader.trim_text(true);
//
//    let mut query = Query { ..Default::default() };
//
//    let mut lines: Vec<Line> = Vec::new();
//    let mut xml_tag: XmlTag = XmlTag::Ignore;
//    let mut buf = Vec::new();
//
//    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`xml_resp)
//    let mut temp_line = Line { ..Default::default() };
//
//    loop {
//        match reader.read_event(&mut buf) {
//            // for triggering namespaced events, use this instead:
//            // match reader.read_namespaced_event(&mut buf) {
//            Ok(XmlEvent::StartElement(ref e)) => {
//                // for namespaced:
//                // Ok((ref namespace_value, Event::Start(ref e)))
//                match e.name() {
//                    b"timestamp" => {
//                        xml_tag = XmlTag::TimeStamp;
//                    }
//                    b"text" => {
//                        xml_tag = XmlTag::Ignore;
//                    }
//                    b"name" => {
//                        xml_tag = XmlTag::LineName;
//                    }
//                    b"status" => {
//                        xml_tag = XmlTag::LineStatus;
//                    }
//                    b"Date" => {
//                        xml_tag = XmlTag::LineDate;
//                    }
//                    b"Time" => {
//                        xml_tag = XmlTag::LineTime;
//                    }
//                    _ => xml_tag = XmlTag::Ignore,
//                }
//            }
//            Ok(XmlEvent::Text(e)) => {
//                let txt: String = e.unescape_and_decode(&reader).unwrap();
//                match xml_tag {
//                    XmlTag::TimeStamp => query.timestamp = txt,
//                    XmlTag::LineName => temp_line.name = txt,
//                    XmlTag::LineStatus => temp_line.status = txt,
//                    XmlTag::LineDate => temp_line.date = txt,
//                    XmlTag::LineTime => temp_line.time = txt,
//                    XmlTag::Ignore => (),
//                }
//            }
//            Ok(XmlEvent::EndElement(ref e)) => {
//                match e.name() {
//                    b"line" => {
//                        lines.push(temp_line);
//                        temp_line = Line { ..Default::default() };
//                    }
//                    b"subway" => {
//                        break
//                    }
//                    _ => (),
//                }
//            }
////            Ok(XmlEvent::Eof) => break, // exits the loop when reaching end of file
//            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
//            _ => (), // There are several other `Event`xml_resp we do not consider here
//        }
//
//        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
//        buf.clear();
//    }
//
//    query.lines = lines;
//    query
//}