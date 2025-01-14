// c:symbol
use std::io::Cursor;

use quick_xml::{events::BytesStart, Reader, Writer};

use super::{super::super::EnumValue, MarkerStyleValues};
use crate::{reader::driver::get_attribute, writer::driver::write_start_tag};

#[derive(Clone, Default, Debug)]
pub struct Symbol {
    val: EnumValue<MarkerStyleValues>,
}
impl Symbol {
    #[must_use]
    pub fn get_val(&self) -> &MarkerStyleValues {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: MarkerStyleValues) -> &mut Symbol {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:symbol
        write_start_tag(
            writer,
            "c:symbol",
            vec![("val", self.val.get_value_string()).into()],
            true,
        );
    }
}
