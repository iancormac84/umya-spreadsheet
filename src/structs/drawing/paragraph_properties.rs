// a:pPr
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    super::EnumValue,
    LineSpacing,
    RunProperties,
    TextAlignmentTypeValues,
};
use crate::{
    StringValue,
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

#[derive(Clone, Default, Debug)]
pub struct ParagraphProperties {
    right_to_left:          StringValue,
    alignment:              EnumValue<TextAlignmentTypeValues>,
    default_run_properties: Option<Box<RunProperties>>,
    line_spacing:           Option<LineSpacing>,
}

impl ParagraphProperties {
    #[inline]
    #[must_use]
    pub fn get_right_to_left(&self) -> Option<&str> {
        self.right_to_left.value()
    }

    #[inline]
    pub fn set_right_to_left<S: Into<String>>(&mut self, value: S) -> &mut ParagraphProperties {
        self.right_to_left.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_alignment(&self) -> &TextAlignmentTypeValues {
        self.alignment.value()
    }

    #[inline]
    pub fn set_alignment(&mut self, value: TextAlignmentTypeValues) -> &mut ParagraphProperties {
        self.alignment.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_default_run_properties(&self) -> Option<&RunProperties> {
        self.default_run_properties.as_deref()
    }

    #[inline]
    pub fn get_default_run_properties_mut(&mut self) -> Option<&mut RunProperties> {
        self.default_run_properties.as_deref_mut()
    }

    #[inline]
    pub fn set_default_run_properties(&mut self, value: RunProperties) -> &mut ParagraphProperties {
        self.default_run_properties = Some(Box::new(value));
        self
    }

    #[inline]
    #[must_use]
    pub fn get_line_spacing(&self) -> Option<&LineSpacing> {
        self.line_spacing.as_ref()
    }

    #[inline]
    pub fn get_line_spacing_mut(&mut self) -> Option<&mut LineSpacing> {
        self.line_spacing.as_mut()
    }

    #[inline]
    pub fn set_line_spacing(&mut self, value: LineSpacing) -> &mut Self {
        self.line_spacing = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        if let Some(v) = get_attribute(e, b"rtl") {
            self.set_right_to_left(v);
        }
        set_string_from_xml!(self, e, alignment, "algn");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                b"a:defRPr" => {
                    let mut obj = RunProperties::default();
                    obj.set_attributes(reader, e, false);
                    self.set_default_run_properties(obj);
                }
                b"a:lnSpc" => {
                    let mut obj = LineSpacing::default();
                    obj.set_attributes(reader, e);
                    self.set_line_spacing(obj);
                }
                _ => (),
                }
            },
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:defRPr" {
                    let mut obj = RunProperties::default();
                    obj.set_attributes(reader, e, true);
                    self.set_default_run_properties(obj);
                }
            },
            Event::End(ref e) => {
                if  e.name().into_inner() == b"a:pPr" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:pPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:pPr
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if let Some(v) = self.right_to_left.value() {
            attributes.push(("rtl", v).into());
        }
        if self.alignment.has_value() {
            attributes.push(("algn", self.alignment.value_string()).into());
        }

        let empty_flag = self.default_run_properties.is_none() && self.line_spacing.is_none();
        write_start_tag(writer, "a:pPr", attributes, empty_flag);

        if !empty_flag {
            // a:defRPr
            if let Some(v) = &self.default_run_properties {
                v.write_to_def_rpr(writer);
            }

            // a:lnSpc
            if let Some(v) = &self.line_spacing {
                v.write_to(writer);
            }

            write_end_tag(writer, "a:pPr");
        }
    }
}
