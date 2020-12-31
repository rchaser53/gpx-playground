extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

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
              Err(e) => {
                  println!("Error: {}", e);
                  break;
              }
              _ => {}
            }
            treak.push(e.clone());
            continue
        }

        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);                
                depth += 1;
                if &name.local_name == "trk" {
                    is_treak = true;
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    dbg!(treak);
}