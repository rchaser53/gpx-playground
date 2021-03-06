use std::io::{Read, Write};

use super::trkseg::reverse_trkseg;
use xml::reader::{EventReader, Result, XmlEvent};
use xml::writer::EventWriter;

pub struct ParsedXML {
    pre: Vec<XmlEvent>,
    trksegs: Vec<XmlEvent>,
    post: Vec<XmlEvent>,
}

fn write_xml<W: Write>(writer: &mut EventWriter<W>, input: Vec<XmlEvent>) -> Result<()> {
    for e in input {
        if let Some(e) = e.as_writer_event() {
            writer.write(e).expect("write xml failed")
        }
    }
    Ok(())
}

pub fn reverse<R: Read, W: Write>(
    parser: EventReader<R>,
    mut writer: EventWriter<W>,
) -> Result<()> {
    let mut persed_xml = parse_xml(parser)?;
    persed_xml.trksegs = reverse_trkseg(persed_xml.trksegs);
    write_xml(&mut writer, persed_xml.pre)?;
    write_xml(&mut writer, persed_xml.trksegs)?;
    write_xml(&mut writer, persed_xml.post)?;
    Ok(())
}

pub fn concat<R: Read, W: Write>(
    parser_a: EventReader<R>,
    parser_b: EventReader<R>,
    mut writer: EventWriter<W>,
) -> Result<()> {
    let persed_xml_a = parse_xml(parser_a)?;
    let persed_xml_b = parse_xml(parser_b)?;
    write_xml(&mut writer, persed_xml_a.pre)?;
    write_xml(&mut writer, persed_xml_a.trksegs)?;
    write_xml(&mut writer, persed_xml_b.trksegs)?;
    write_xml(&mut writer, persed_xml_b.post)?;
    Ok(())
}

pub fn parse_xml<R: Read>(mut parser: EventReader<R>) -> Result<ParsedXML> {
    let mut pre = vec![];
    let mut trksegs: Vec<XmlEvent> = vec![];
    let mut post = vec![];
    let mut is_treak = false;
    while let Ok(e) = parser.next() {
        if XmlEvent::EndDocument == e {
            break;
        }

        if is_treak {
            match e {
                XmlEvent::EndElement { name } => {
                    if &name.local_name == "trkseg" {
                        is_treak = false;
                        post.push(XmlEvent::EndElement { name });
                    } else {
                        trksegs.push(XmlEvent::EndElement { name });
                    }
                }
                e => {
                    trksegs.push(e);
                }
            }
        } else {
            let e = match e {
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    if &name.local_name == "trkseg" {
                        is_treak = true;
                    }
                    XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    }
                }
                XmlEvent::EndElement { name } => XmlEvent::EndElement { name },
                a => a,
            };

            if trksegs.is_empty() {
                pre.push(e);
            } else {
                post.push(e);
            }
        }
    }

    if let Err(e) = parser.next() {
        Err(e)
    } else {
        Ok(ParsedXML { pre, trksegs, post })
    }
}
