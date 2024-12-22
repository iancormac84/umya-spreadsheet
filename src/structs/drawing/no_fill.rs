// a:noFill
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::BytesStart;

use crate::writer::driver::write_start_tag;

#[derive(Clone, Default, Debug)]
pub struct NoFill {}
impl NoFill {
    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        _: &BytesStart,
    ) {
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:noFill
        write_start_tag(writer, "a:noFill", vec![], true);
    }
}
