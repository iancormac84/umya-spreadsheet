// c:scatterStyle
use super::ScatterStyleValues;
use super::super::super::EnumValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct ScatterStyle {
    val: EnumValue<ScatterStyleValues>,
}
impl ScatterStyle {
    pub fn get_val(&self)-> &ScatterStyleValues {
        &self.val.get_value()
    }

    pub fn set_val(&mut self, value:ScatterStyleValues)-> &mut ScatterStyle {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader:&mut Reader<R>,
        e:&BytesStart
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:scatterStyle
        write_start_tag(writer, "c:scatterStyle", vec![
            ("val", &self.val.get_value_string()),
        ], true);
    }
}