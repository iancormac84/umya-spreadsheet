// brk
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    structs::{
        BooleanValue,
        UInt32Value,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Break {
    id:                UInt32Value,
    max:               UInt32Value,
    min:               UInt32Value,
    manual_page_break: BooleanValue,
}

impl Break {
    #[inline]
    #[must_use]
    pub fn id(&self) -> u32 {
        self.id.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use id()")]
    pub fn get_id(&self) -> u32 {
        self.id()
    }

    #[inline]
    pub fn set_id(&mut self, value: u32) -> &mut Self {
        self.id.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn max(&self) -> u32 {
        self.max.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use max()")]
    pub fn get_max(&self) -> u32 {
        self.max()
    }

    #[inline]
    pub fn set_max(&mut self, value: u32) -> &mut Self {
        self.max.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn manual_page_break(&self) -> bool {
        self.manual_page_break.get_value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use manual_page_break()")]
    pub fn get_manual_page_break(&self) -> bool {
        self.manual_page_break()
    }

    #[inline]
    pub fn set_manual_page_break(&mut self, value: bool) -> &mut Self {
        self.manual_page_break.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, id, "id");
        set_string_from_xml!(self, e, max, "max");
        set_string_from_xml!(self, e, min, "min");
        set_string_from_xml!(self, e, manual_page_break, "man");
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // brk
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let id = self.id.value_string();
        attributes.push(("id", &id).into());

        let max = self.max.value_string();
        if self.max.has_value() {
            attributes.push(("max", &max).into());
        }

        let min = self.min.value_string();
        if self.min.has_value() {
            attributes.push(("min", &min).into());
        }

        let manual_page_break = self.manual_page_break.get_value_string();
        if self.manual_page_break.has_value() {
            attributes.push(("man", manual_page_break).into());
        }
        write_start_tag(writer, "brk", attributes, true);
    }
}
