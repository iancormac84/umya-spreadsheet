// a:sp3d
use super::PresetMaterialTypeValues;
use super::super::EnumValue;
use super::BevelTop;
use super::BevelBottom;
use writer::driver::*;
use reader::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Shape3DType {
    preset_material:EnumValue<PresetMaterialTypeValues>,
    bevel_top: Option<BevelTop>,
    bevel_bottom: Option<BevelBottom>,
}
impl Shape3DType {
    pub fn get_preset_material(&self)-> &PresetMaterialTypeValues {
        &self.preset_material.get_value()
    }

    pub fn set_preset_material(&mut self, value:PresetMaterialTypeValues)-> &mut Shape3DType {
        self.preset_material.set_value(value);
        self
    }

    pub fn get_bevel_top(&self) -> &Option<BevelTop> {
        &self.bevel_top
    }

    pub fn get_bevel_top_mut(&mut self) -> &mut Option<BevelTop> {
        &mut self.bevel_top
    }

    pub fn set_bevel_top(&mut self, value:BevelTop) {
        self.bevel_top = Some(value);
    }

    pub fn get_bevel_bottom(&self) -> &Option<BevelBottom> {
        &self.bevel_bottom
    }

    pub fn get_bevel_bottom_mut(&mut self) -> &mut Option<BevelBottom> {
        &mut self.bevel_bottom
    }

    pub fn set_bevel_bottom(&mut self, value:BevelBottom) {
        self.bevel_bottom = Some(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        e:&BytesStart
    ) {
        &mut self.preset_material.set_value_string(get_attribute(e, b"prstMaterial").unwrap());

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"a:bevelT" => {
                            let mut obj = BevelTop::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_bevel_top(obj);
                        },
                        b"a:bevelB" => {
                            let mut obj = BevelBottom::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_bevel_bottom(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:sp3d" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:sp3d"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }

    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:sp3d
        write_start_tag(writer, "a:sp3d", vec![
            ("prstMaterial", &self.preset_material.get_value_string())
        ], false);

        // a:bevelT
        match &self.bevel_top {
            Some(v) => {v.write_to(writer)},
            None => {},
        }

        // a:bevelB
        match &self.bevel_bottom {
            Some(v) => {v.write_to(writer)},
            None => {},
        }

        write_end_tag(writer, "a:sp3d");
    }
}