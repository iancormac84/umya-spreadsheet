// colItems
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
    reader::driver::xml_read_loop,
    structs::RowItem,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct ColumnItems {
    list: Vec<RowItem>,
}
impl ColumnItems {
    #[inline]
    #[must_use]
    pub fn get_list(&self) -> &[RowItem] {
        &self.list
    }

    #[inline]
    pub fn get_list_mut(&mut self) -> &mut Vec<RowItem> {
        &mut self.list
    }

    #[inline]
    pub fn add_list_mut(&mut self, value: RowItem) -> &mut Self {
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
                if e.name().into_inner() == b"i" {
                    let mut obj = RowItem::default();
                    obj.set_attributes(reader, e, true);
                    self.add_list_mut(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"i" {
                    let mut obj = RowItem::default();
                    obj.set_attributes(reader, e, false);
                    self.add_list_mut(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"colItems" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "colItems")
        );
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // colItems
        write_start_tag(
            writer,
            "colItems",
            vec![("count", self.list.len().to_string().as_str())],
            false,
        );

        // i
        for i in &self.list {
            i.write_to(writer);
        }

        write_end_tag(writer, "colItems");
    }
}
