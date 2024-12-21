use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::{make_file_from_writer, write_end_tag, write_new_line, write_start_tag};
use super::XlsxError;
use crate::helper::const_str::{
    CONTENT_TYPES, CONTYPES_NS, PRNTR_SETTINGS_TYPE, REL_TYPE, VML_DRAWING_TYPE, WORKBOOK,
};
use crate::structs::Spreadsheet;
use crate::structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let is_light = writer_mng.get_is_light();
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer
        .write_event(Event::Decl(BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            Some("yes"),
        )))
        .unwrap();
    write_new_line(&mut writer);

    // Types
    write_start_tag(&mut writer, "Types", vec![("xmlns", CONTYPES_NS)], false);

    // Default rels
    write_start_tag(
        &mut writer,
        "Default",
        vec![("Extension", "rels"), ("ContentType", REL_TYPE)],
        true,
    );

    // Default xml
    write_start_tag(
        &mut writer,
        "Default",
        vec![("Extension", "xml"), ("ContentType", "application/xml")],
        true,
    );

    // Default bin
    if writer_mng.has_extension("bin") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "bin"), ("ContentType", PRNTR_SETTINGS_TYPE)],
            true,
        );
    }

    // Default vml
    if writer_mng.has_extension("vml") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "vml"), ("ContentType", VML_DRAWING_TYPE)],
            true,
        );
    }

    // Default png
    if writer_mng.has_extension("png") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "png"), ("ContentType", "image/png")],
            true,
        );
    }

    // Default jpg
    if writer_mng.has_extension("jpg") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "jpg"), ("ContentType", "image/jpeg")],
            true,
        );
    }

    // Default jpeg
    if writer_mng.has_extension("jpeg") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "jpeg"), ("ContentType", "image/jpeg")],
            true,
        );
    }

    // Default tiff
    if writer_mng.has_extension("tiff") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "tiff"), ("ContentType", "image/tiff")],
            true,
        );
    }

    // Default emf
    if writer_mng.has_extension("emf") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "emf"), ("ContentType", "image/x-emf")],
            true,
        );
    }

    // Default xlsx
    if writer_mng.has_extension("xlsx") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "xlsx"), ("ContentType", WORKBOOK)],
            true,
        );
    }

    // Override
    for (part_name, content_type) in writer_mng.make_context_type_override(spreadsheet) {
        write_start_tag(
            &mut writer,
            "Override",
            vec![("PartName", &part_name), ("ContentType", &content_type)],
            true,
        );
    }

    write_end_tag(&mut writer, "Types");
    make_file_from_writer(
        CONTENT_TYPES,
        writer_mng.get_arv_mut(),
        writer,
        None,
        is_light,
    )?;
    Ok(())
}
