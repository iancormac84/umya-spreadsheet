// c:lang
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::super::super::StringValue;
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct EditingLanguage {
    val: StringValue,
}
impl EditingLanguage {
    #[must_use]
    pub fn get_val(&self) -> &str {
        self.val.get_value_str()
    }

    pub fn set_val<S: Into<String>>(&mut self, value: S) -> &mut EditingLanguage {
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
        // c:lang
        write_start_tag(writer, "c:lang", vec![("val", self.val.get_value_str())], true);
    }
}
