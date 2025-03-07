// cacheField
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    structs::{
        SharedItems,
        StringValue,
        UInt32Value,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct CacheField {
    name:             StringValue,
    number_format_id: UInt32Value,
    shared_items:     SharedItems,
}
impl CacheField {
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use name()")]
    pub fn get_name(&self) -> &str {
        self.name()
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn number_format_id(&self) -> u32 {
        self.number_format_id.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use number_format_id()")]
    pub fn get_number_format_id(&self) -> u32 {
        self.number_format_id()
    }

    #[inline]
    pub fn set_number_format_id(&mut self, value: u32) -> &mut Self {
        self.number_format_id.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn shared_items(&self) -> &SharedItems {
        &self.shared_items
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use shared_items()")]
    pub fn get_shared_items(&self) -> &SharedItems {
        self.shared_items()
    }

    #[inline]
    pub fn shared_items_mut(&mut self) -> &mut SharedItems {
        &mut self.shared_items
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use shared_items_mut()")]
    pub fn get_shared_items_mut(&mut self) -> &mut SharedItems {
        self.shared_items_mut()
    }

    #[inline]
    pub fn set_shared_items(&mut self, value: SharedItems) -> &mut Self {
        self.shared_items = value;
        self
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, name, "name");
        set_string_from_xml!(self, e, number_format_id, "numFmtId");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"sharedItems" {
                    let mut obj = SharedItems::default();
                    obj.set_attributes(reader, e);
                    self.set_shared_items(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"cacheField" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "cacheField")
        );
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pivotField
        write_start_tag(
            writer,
            "pivotField",
            vec![
                ("name", self.name.value_str()).into(),
                (
                    "numFmtId",
                    &self.number_format_id.value_string(),
                )
                    .into(),
            ],
            false,
        );

        // sharedItems
        self.shared_items.write_to(writer);

        write_end_tag(writer, "pivotField");
    }
}
