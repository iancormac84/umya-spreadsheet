// a:extLst
use crate::reader::driver::xml_read_loop;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct ExtensionList {}
impl ExtensionList {
    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:extLst" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:extLst")
        );
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn write_to(&self, _writer: &mut Writer<Cursor<Vec<u8>>>) {}
}
