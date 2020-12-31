extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EventWriter, XmlEvent as WXMLEvent, EmitterConfig, Result};
use xml::name::OwnedName;
use xml::attribute::OwnedAttribute;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let file = File::open("./test/test.gpx").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    let mut treak = vec![];
    let mut is_treak = false;
    let mut file = File::create("output.xml").unwrap();
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(&mut file);
    for e in parser {
        if is_treak {
            match e {
              Ok(XmlEvent::StartElement { .. }) => {
                  depth += 1;
              }
              Ok(XmlEvent::EndElement { ref name, .. }) => {
                  depth -= 1;
                  if &name.local_name == "trk" {
                      is_treak = false;
                  }
              }
              _ => {}
            }            
            treak.push(e.clone());
            if let Ok(e) = e {
                if let Some(v) = e.as_writer_event() {
                    writer.write(v);
                }
            }
        } else {
            let e = match e {
                Ok(XmlEvent::StartElement { name, mut attributes, namespace }) => {
                    println!("{}+{}", indent(depth), name);                
                    depth += 1;
                    if &name.local_name == "trk" {
                        is_treak = true;
                    }
                    if &name.local_name == "author" {
                        attributes.push(OwnedAttribute::new(OwnedName::local("attr"), "bar"));
                    }
                    XmlEvent::StartElement { name, attributes, namespace }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    println!("{}-{}", indent(depth), name);
                    XmlEvent::EndElement { name }
                }
                Ok(a) => a,
                Err(e) => panic!(e)
            };
            if let Some(v) = e.as_writer_event() {
                writer.write(v);
            }
        }
    }
}