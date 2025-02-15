// c:delete
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::super::super::BooleanValue;
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Delete {
    val: BooleanValue,
}
impl Delete {
    #[must_use]
    pub fn get_val(&self) -> bool {
        self.val.value()
    }

    pub fn set_val(&mut self, value: bool) -> &mut Delete {
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
        // c:delete
        write_start_tag(
            writer,
            "c:delete",
            vec![("val", self.val.value_string()).into()],
            true,
        );
    }
}
