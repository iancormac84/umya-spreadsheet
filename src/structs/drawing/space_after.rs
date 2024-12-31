// a:spcAft
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::SpacingPercent;
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct SpaceAfter {
    spacing_percent: Option<SpacingPercent>,
}

impl SpaceAfter {
    #[inline]
    #[must_use]
    pub fn get_spacing_percent(&self) -> Option<&SpacingPercent> {
        self.spacing_percent.as_ref()
    }

    #[inline]
    pub fn get_spacing_percent_mut(&mut self) -> Option<&mut SpacingPercent> {
        self.spacing_percent.as_mut()
    }

    #[inline]
    pub fn set_spacing_percent(&mut self, value: SpacingPercent) -> &mut Self {
        self.spacing_percent = Some(value);
        self
    }

    #[inline]
    pub fn remove_spacing_percent(&mut self) {
        self.spacing_percent = None;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:spcPct" {
                    let mut obj = SpacingPercent::default();
                    obj.set_attributes(reader, e);
                    self.set_spacing_percent(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:spcAft" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:spcAft")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:spcAft
        write_start_tag(writer, "a:spcAft", vec![], false);

        // a:spcPct
        if let Some(v) = &self.spacing_percent {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:spcAft");
    }
}
