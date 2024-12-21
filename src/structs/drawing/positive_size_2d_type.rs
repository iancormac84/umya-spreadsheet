// a:ext
// a:chExt
use crate::reader::driver::{get_attribute, set_string_from_xml};
use crate::structs::Int64Value;
use crate::writer::driver::write_start_tag;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct PositiveSize2DType {
    cx: Int64Value,
    cy: Int64Value,
}

impl PositiveSize2DType {
    #[inline]
    #[must_use]
    pub fn get_cx(&self) -> i64 {
        self.cx.get_value()
    }

    #[inline]
    pub fn set_cx(&mut self, value: i64) {
        self.cx.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn get_cy(&self) -> i64 {
        self.cy.get_value()
    }

    #[inline]
    pub fn set_cy(&mut self, value: i64) {
        self.cy.set_value(value);
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, cx, "cx");
        set_string_from_xml!(self, e, cy, "cy");
    }

    #[inline]
    pub(crate) fn write_to_ext(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:ext");
    }

    #[inline]
    pub(crate) fn write_to_ch_ext(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:chExt");
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let cx_str = self.cx.get_value_string();
        attributes.push(("cx", &cx_str));
        let cy_str = self.cy.get_value_string();
        attributes.push(("cy", &cy_str));
        write_start_tag(writer, tag_name, attributes, true);
    }
}
