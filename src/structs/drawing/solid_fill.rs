// a:solidFill
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event};

use super::rgb_color_model_hex::RgbColorModelHex;
use super::scheme_color::SchemeColor;
use crate::reader::driver::xml_read_loop;
use crate::writer::driver::{write_end_tag, write_start_tag};

#[derive(Clone, Default, Debug)]
pub struct SolidFill {
    scheme_color: Option<Box<SchemeColor>>,
    rgb_color_model_hex: Option<Box<RgbColorModelHex>>,
}

impl SolidFill {
    #[inline]
    #[must_use]
    pub fn get_scheme_color(&self) -> Option<&SchemeColor> {
        self.scheme_color.as_deref()
    }

    #[inline]
    pub fn get_scheme_color_mut(&mut self) -> Option<&mut SchemeColor> {
        self.scheme_color.as_deref_mut()
    }

    #[inline]
    pub fn set_scheme_color(&mut self, value: SchemeColor) {
        self.scheme_color = Some(Box::new(value));
    }

    #[inline]
    #[must_use]
    pub fn get_rgb_color_model_hex(&self) -> Option<&RgbColorModelHex> {
        self.rgb_color_model_hex.as_deref()
    }

    #[inline]
    pub fn get_rgb_color_model_hex_mut(&mut self) -> Option<&mut RgbColorModelHex> {
        self.rgb_color_model_hex.as_deref_mut()
    }

    #[inline]
    pub fn set_rgb_color_model_hex(&mut self, value: RgbColorModelHex) {
        self.rgb_color_model_hex = Some(Box::new(value));
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                b"a:schemeClr" => {
                    let mut scheme_color = SchemeColor::default();
                    scheme_color.set_attributes(reader, e, false);
                    self.set_scheme_color(scheme_color);
                }
                b"a:srgbClr" => {
                    let mut rgb_color_model_hex = RgbColorModelHex::default();
                    rgb_color_model_hex.set_attributes(reader, e, false);
                    self.set_rgb_color_model_hex(rgb_color_model_hex);
                }
                _ => (),
                }
            },
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                b"a:schemeClr" => {
                    let mut scheme_color = SchemeColor::default();
                    scheme_color.set_attributes(reader, e, true);
                    self.set_scheme_color(scheme_color);
                }
                b"a:srgbClr" => {
                    let mut rgb_color_model_hex = RgbColorModelHex::default();
                    rgb_color_model_hex.set_attributes(reader, e, true);
                    self.set_rgb_color_model_hex(rgb_color_model_hex);
                }
                _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:solidFill" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:solidFill")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:solidFill
        write_start_tag(writer, "a:solidFill", vec![], false);
        if let Some(color) = &self.scheme_color {
            color.write_to(writer);
        }

        if let Some(hex) = &self.rgb_color_model_hex {
            hex.write_to(writer);
        }
        write_end_tag(writer, "a:solidFill");
    }
}
