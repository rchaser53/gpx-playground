extern crate xml;

use std::fs::File;
use std::io::BufReader;

use std::path::PathBuf;
use structopt::StructOpt;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::{EventReader, XmlEvent};
use xml::writer::EmitterConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    input: PathBuf,
    output: PathBuf,
}

fn reverse_trkseg(trksegs: Vec<XmlEvent>) -> Vec<XmlEvent> {
    let mut result = Vec::with_capacity(trksegs.len());
    let mut trks = vec![];
    let mut trksegs_iter = trksegs.into_iter();
    while let Some(e) = trksegs_iter.next() {
        let mut temp = vec![];
        temp.push(e);
        while let Some(e) = trksegs_iter.next() {
            temp.push(e.clone());
            if let XmlEvent::EndElement { ref name } = e {
                if name.local_name == "trkpt" {
                    break;
                }
            }
        }
        trks.push(temp);
    }
    trks.reverse();

    for trk in trks {
        for tag in trk {
            result.push(tag);
        }
    }
    result
}

fn main() {
    let opt = Opt::from_args();

    let file = File::open(opt.input).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut trksegs: Vec<XmlEvent> = vec![];
    let mut is_treak = false;
    let mut file = File::create(opt.output).unwrap();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut file);
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
}
