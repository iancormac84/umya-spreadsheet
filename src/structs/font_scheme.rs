// scheme
use super::EnumValue;
use super::FontSchemeValues;
use crate::reader::driver::get_attribute;
use crate::writer::driver::write_start_tag;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FontScheme {
    pub(crate) val: EnumValue<FontSchemeValues>,
}

impl FontScheme {
    #[inline]
    #[must_use]
    pub fn get_val(&self) -> &FontSchemeValues {
        self.val.get_value()
    }

    #[inline]
    pub fn set_val(&mut self, value: FontSchemeValues) -> &mut Self {
        self.val.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // scheme
        if self.val.has_value() {
            let attributes = vec![("val", self.val.get_value_string())];
            write_start_tag(writer, "scheme", attributes, true);
        }
    }
}
