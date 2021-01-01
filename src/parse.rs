use std::io::{Read, Write};

use xml::reader::{EventReader, Result, XmlEvent};
use xml::writer::EventWriter;

use super::trkseg::reverse_trkseg;

pub fn parse<R: Read, W: Write>(parser: EventReader<R>, mut writer: EventWriter<W>) -> Result<()> {
    let mut trksegs: Vec<XmlEvent> = vec![];
    let mut is_treak = false;
    for e in parser {
        if is_treak {
            match e {
                Ok(XmlEvent::EndElement { name }) => {
                    if &name.local_name == "trkseg" {
                        is_treak = false;
                        for ie in reverse_trkseg(trksegs) {
                            if let Some(v) = ie.as_writer_event() {
                                writer.write(v).expect("parse failed");
                            }
                        }
                        trksegs = vec![];
                        let end_event = XmlEvent::EndElement { name };
                        writer
                            .write(end_event.as_writer_event().unwrap())
                            .expect("parse failed");
                    } else {
                        trksegs.push(XmlEvent::EndElement { name });
                    }
                }
                Ok(e) => {
                    trksegs.push(e);
                }
                _ => {}
            }
        } else {
            let e = match e {
                Ok(XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => {
                    if &name.local_name == "trkseg" {
                        is_treak = true;
                    }
                    XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    }
                }
                Ok(XmlEvent::EndElement { name }) => XmlEvent::EndElement { name },
                Ok(a) => a,
                Err(e) => panic!(e),
            };
            if let Some(v) = e.as_writer_event() {
                writer.write(v).expect("parse failed");
            }
        }
    }
    Ok(())
}
