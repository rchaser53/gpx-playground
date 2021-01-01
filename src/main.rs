extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EmitterConfig, EventWriter, Result, XmlEvent as WXMLEvent};

fn main() {
    let file = File::open("./test/test.gpx").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut treak: Vec<XmlEvent> = vec![];
    let mut is_treak = false;
    let mut file = File::create("output.xml").unwrap();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut file);
    for e in parser {
        if is_treak {
            match e {
                Ok(XmlEvent::EndElement { ref name, .. }) => {
                    if &name.local_name == "trk" {
                        is_treak = false;
                        treak.reverse();
                        for ie in treak {
                            if let Some(v) = ie.as_writer_event() {
                                writer.write(v).expect("parse failed");
                            }
                        }
                        treak = vec![];
                    }
                }
                Ok(e) => {
                    treak.push(e);
                }
                _ => {}
            }
        } else {
            let e = match e {
                Ok(XmlEvent::StartElement {
                    name,
                    mut attributes,
                    namespace,
                }) => {
                    if &name.local_name == "trk" {
                        is_treak = true;
                    }
                    if &name.local_name == "author" {
                        attributes.push(OwnedAttribute::new(OwnedName::local("attr"), "bar"));
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
}
