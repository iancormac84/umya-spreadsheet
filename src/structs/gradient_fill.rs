// gradientFill
use std::{
    fmt::Write,
    io::Cursor,
};

use md5::Digest;
use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    DoubleValue,
    GradientStop,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct GradientFill {
    degree:        DoubleValue,
    gradient_stop: Vec<GradientStop>,
}

impl GradientFill {
    #[inline]
    #[must_use]
    pub fn degree(&self) -> f64 {
        self.degree.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use degree()")]
    pub fn get_degree(&self) -> f64 {
        self.degree()
    }

    #[inline]
    pub fn set_degree(&mut self, value: f64) -> &mut Self {
        self.degree.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn gradient_stop(&self) -> &[GradientStop] {
        &self.gradient_stop
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use gradient_stop()")]
    pub fn get_gradient_stop(&self) -> &[GradientStop] {
        self.gradient_stop()
    }

    #[inline]
    pub fn gradient_stop_mut(&mut self) -> &mut Vec<GradientStop> {
        &mut self.gradient_stop
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use gradient_stop_mut()")]
    pub fn get_gradient_stop_mut(&mut self) -> &mut Vec<GradientStop> {
        self.gradient_stop_mut()
    }

    #[inline]
    pub fn set_gradient_stop(&mut self, value: GradientStop) -> &mut Self {
        self.gradient_stop.push(value);
        self
    }

    pub(crate) fn hash_code(&self) -> String {
        let mut value = String::new();
        for stop in &self.gradient_stop {
            write!(value, "{}", stop.hash_code()).unwrap();
        }
        format!(
            "{:x}",
            md5::Md5::digest(format!("{}{}", &self.degree.value_string(), value,))
        )
    }

    #[deprecated(since = "3.0.0", note = "Use hash_code()")]
    pub(crate) fn get_hash_code(&self) -> String {
        self.hash_code()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, degree, "degree");

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"stop" {
                    let mut obj = GradientStop::default();
                    obj.set_attributes(reader, e);
                    self.set_gradient_stop(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"gradientFill" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "gradientFill")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // gradientFill
        write_start_tag(
            writer,
            "gradientFill",
            vec![("degree", &self.degree.value_string()).into()],
            false,
        );

        // stop
        for stop in &self.gradient_stop {
            stop.write_to(writer);
        }

        write_end_tag(writer, "gradientFill");
    }
}
