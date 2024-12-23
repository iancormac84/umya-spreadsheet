use std::io;

use quick_xml::{
    Reader,
    events::Event,
};

use super::XlsxError;
use crate::{
    helper::const_str::ARC_CUSTOM,
    structs::Spreadsheet,
};

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> Result<(), XlsxError> {
    let r = io::BufReader::new(match arv.by_name(ARC_CUSTOM) {
        Ok(v) => v,
        Err(zip::result::ZipError::FileNotFound) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e.into());
        }
    });
    let mut reader = Reader::from_reader(r);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name().into_inner() == b"Properties" {
                    spreadsheet
                        .get_properties_mut()
                        .set_attributes_custom(&mut reader, e);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok(())
}
