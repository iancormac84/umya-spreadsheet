// pivotFields
use crate::reader::driver::xml_read_loop;
use crate::structs::PivotField;
use crate::writer::driver::{write_end_tag, write_start_tag};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct PivotFields {
    list: ThinVec<PivotField>,
}
impl PivotFields {
    #[inline]
    #[must_use]
    pub fn get_list(&self) -> &[PivotField] {
        &self.list
    }

    #[inline]
    pub fn get_list_mut(&mut self) -> &mut ThinVec<PivotField> {
        &mut self.list
    }

    #[inline]
    pub fn add_list_mut(&mut self, value: PivotField) -> &mut Self {
        self.list.push(value);
        self
    }

    #[inline]
    #[allow(unused_variables)]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"pivotField" {
                    let mut obj = PivotField::default();
                    obj.set_attributes(reader, e);
                    self.add_list_mut(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"pivotFields" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "pivotFields")
        );
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pivotFields
        write_start_tag(
            writer,
            "pivotFields",
            vec![("count", self.list.len().to_string().as_str())],
            false,
        );

        // pivotField
        for sheet_view in &self.list {
            sheet_view.write_to(writer);
        }

        write_end_tag(writer, "pivotFields");
    }
}
