use std::io;

use quick_xml::Writer;
use quick_xml::events::{BytesDecl, Event};

use super::XlsxError;
use super::driver::write_new_line;
use crate::helper::const_str::ARC_APP;
use crate::structs::Spreadsheet;
use crate::structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), Some("yes")))).unwrap();
    write_new_line(&mut writer);

    // Properties
    spreadsheet
        .get_properties()
        .write_to_app(&mut writer, spreadsheet.get_sheet_collection_no_check());

    writer_mng.add_writer(ARC_APP, writer)
}
