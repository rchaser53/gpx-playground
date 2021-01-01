use xml::reader::XmlEvent;

pub fn reverse_trkseg(trksegs: Vec<XmlEvent>) -> Vec<XmlEvent> {
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
