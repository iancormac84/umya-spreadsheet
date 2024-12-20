use crate::xml_read_loop;

use super::vml::spreadsheet::Anchor;
use super::Coordinate;
use super::RichText;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use reader::driver::*;
use structs::vml::Shape;
use traits::AdjustmentCoordinate;

#[derive(Clone, Default, Debug)]
pub struct Comment {
    coordinate: Coordinate,
    author: Box<str>,
    text: RichText,
    shape: Shape,
}

impl Comment {
    #[inline]
    pub fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    #[inline]
    pub fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.coordinate
    }

    #[inline]
    pub fn get_author(&self) -> &str {
        &self.author
    }

    #[inline]
    pub fn set_author<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.author = value.into().into_boxed_str();
        self
    }

    #[inline]
    pub fn get_text(&self) -> &RichText {
        &self.text
    }

    #[inline]
    pub fn get_text_mut(&mut self) -> &mut RichText {
        &mut self.text
    }

    #[inline]
    pub fn set_text(&mut self, value: RichText) -> &mut Self {
        self.text = value;
        self
    }

    #[inline]
    pub fn get_anchor(&self) -> &Anchor {
        self.shape.get_client_data().get_anchor()
    }

    #[inline]
    pub fn get_anchor_mut(&mut self) -> &mut Anchor {
        self.shape.get_client_data_mut().get_anchor_mut()
    }

    #[inline]
    pub fn set_anchor(&mut self, value: Anchor) -> &mut Self {
        self.shape.get_client_data_mut().set_anchor(value);
        self
    }

    #[inline]
    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    #[inline]
    pub fn get_shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }

    #[inline]
    pub fn set_shape(&mut self, value: Shape) -> &mut Self {
        self.shape = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        authors: &[String],
    ) {
        let coordinate = get_attribute(e, b"ref").unwrap();
        self.get_coordinate_mut().set_coordinate(coordinate);

        let author_id = get_attribute(e, b"authorId")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let author = authors.get(author_id).unwrap();
        self.set_author(author);

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"text" {
                    self.get_text_mut().set_attributes_text(reader, e);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"comment" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "comment")
        );
    }
}
impl AdjustmentCoordinate for Comment {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.coordinate.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
        self.shape.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.coordinate.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
        self.shape.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    #[inline]
    fn is_remove_coordinate(
        &self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) -> bool {
        self.coordinate.is_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        )
    }
}
