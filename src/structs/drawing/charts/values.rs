// c:val
use super::NumberReference;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Values {
    number_reference: NumberReference,
}
impl Values {
    pub fn get_number_reference(&self)-> &NumberReference {
        &self.number_reference
    }

    pub fn get_number_reference_mut(&mut self)-> &mut NumberReference {
        &mut self.number_reference
    }

    pub fn set_number_reference(&mut self, value:NumberReference)-> &mut Values {
        self.number_reference = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"c:numRef" => {
                            self.number_reference.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:val" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:val"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:val
        write_start_tag(writer, "c:val", vec![], false);

        // c:numRef
        &self.number_reference.write_to(writer);

        write_end_tag(writer, "c:val");
    }
}