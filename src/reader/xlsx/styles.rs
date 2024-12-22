use std::io;

use quick_xml::Reader;
use quick_xml::events::Event;

use super::XlsxError;
use crate::helper::const_str::PKG_STYLES;
use crate::structs::Spreadsheet;
use crate::structs::Stylesheet;
use crate::xml_read_loop;

pub fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> Result<(), XlsxError> {
    let r = io::BufReader::new(arv.by_name(PKG_STYLES)?);
    let mut reader = Reader::from_reader(r);
    reader.config_mut().trim_text(true);

    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() == b"styleSheet" {
                let mut obj = Stylesheet::default();
                obj.set_attributes(&mut reader, e);
                obj.make_style();
                spreadsheet.set_stylesheet(obj);
            }
        },
        Event::Eof => break
    );

    Ok(())
}
