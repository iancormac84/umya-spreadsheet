use md5::Digest;
use quick_xml::{
    Reader,
    events::BytesStart,
};

use super::{
    BooleanValue,
    DoubleValue,
    Style,
    Stylesheet,
    UInt32Value,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    structs::Cells,
    traits::AdjustmentValue,
};

/// # Examples
/// ## set auto width
/// ```rust
/// use umya_spreadsheet::*;
/// let mut book = new_file();
/// let mut worksheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
/// worksheet.get_column_dimension_mut("A").set_auto_width(true);
/// ```
/// ## set manual width
/// ```rust
/// use umya_spreadsheet::*;
/// let mut book = new_file();
/// let mut worksheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
/// worksheet.get_column_dimension_mut("A").set_width(60f64);
/// ```
#[derive(Clone, Debug)]
pub struct Column {
    col_num:             UInt32Value,
    pub(crate) width:    DoubleValue,
    pub(crate) hidden:   BooleanValue,
    pub(crate) best_fit: BooleanValue,
    style:               Box<Style>,
    auto_width:          BooleanValue,
}

impl Default for Column {
    #[inline]
    fn default() -> Self {
        let mut width = DoubleValue::default();
        width.set_value(8.38f64);
        Self {
            col_num: UInt32Value::default(),
            width,
            hidden: BooleanValue::default(),
            best_fit: BooleanValue::default(),
            style: Box::new(Style::default()),
            auto_width: BooleanValue::default(),
        }
    }
}

impl Column {
    #[inline]
    #[must_use]
    pub fn col_num(&self) -> u32 {
        self.col_num.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use col_num()")]
    pub fn get_col_num(&self) -> u32 {
        self.col_num()
    }

    #[inline]
    pub fn set_col_num(&mut self, value: u32) -> &mut Self {
        self.col_num.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn width(&self) -> f64 {
        self.width.get_value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use width()")]
    pub fn get_width(&self) -> f64 {
        self.width()
    }

    #[inline]
    pub fn set_width(&mut self, value: f64) -> &mut Self {
        self.width.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn hidden(&self) -> bool {
        self.hidden.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use hidden()")]
    pub fn get_hidden(&self) -> bool {
        self.hidden()
    }

    #[inline]
    pub fn set_hidden(&mut self, value: bool) -> &mut Self {
        self.hidden.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn best_fit(&self) -> bool {
        self.best_fit.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use best_fit()")]
    pub fn get_best_fit(&self) -> bool {
        self.best_fit()
    }

    #[inline]
    pub fn set_best_fit(&mut self, value: bool) -> &mut Self {
        self.best_fit.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn style(&self) -> &Style {
        &self.style
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use style()")]
    pub fn get_style(&self) -> &Style {
        self.style()
    }

    #[inline]
    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use style()")]
    pub fn get_style_mut(&mut self) -> &mut Style {
        self.style_mut()
    }

    #[inline]
    pub fn set_style(&mut self, value: Style) -> &mut Self {
        self.style = Box::new(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn auto_width(&self) -> bool {
        self.auto_width.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use auto_width()")]
    pub fn get_auto_width(&self) -> bool {
        self.auto_width()
    }

    #[inline]
    pub fn set_auto_width(&mut self, value: bool) -> &mut Self {
        self.auto_width.set_value(value);
        self
    }

    pub(crate) fn calculation_auto_width(&mut self, cells: &Cells) -> &mut Self {
        if !self.auto_width() {
            return self;
        }

        let mut column_width_max = 0f64;

        // default font size len.
        let column_font_size = match self.style().get_font() {
            Some(font) => font.get_font_size().get_val(),
            None => 11f64,
        };

        let mut cell_list = cells.collection_by_column(self.col_num());
        cell_list.sort_by(|a, b| {
            a.coordinate()
                .get_row_num()
                .cmp(&b.coordinate().get_row_num())
        });
        for cell in cell_list {
            let column_width = cell.width_point(column_font_size);

            if column_width > column_width_max {
                column_width_max = column_width;
            }
        }

        // set default width if empty column.
        if column_width_max == 0f64 {
            column_width_max = 8.38f64;
        }

        self.set_width(column_width_max);
        self
    }

    #[inline]
    pub(crate) fn has_style(&self) -> bool {
        *self.style != Style::default()
    }

    #[inline]
    pub(crate) fn hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}",
                &self.width.get_value_string(),
                &self.hidden.value_string(),
                &self.best_fit.value_string(),
            ))
        )
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use hash_code()")]
    pub(crate) fn get_hash_code(&self) -> String {
        self.hash_code()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
        stylesheet: &Stylesheet,
    ) {
        set_string_from_xml!(self, e, width, "width");
        set_string_from_xml!(self, e, hidden, "hidden");
        set_string_from_xml!(self, e, best_fit, "bestFit");

        if let Some(v) = get_attribute(e, b"style") {
            let style = stylesheet.get_style(v.parse::<usize>().unwrap());
            self.set_style(style);
        }
    }
}
impl AdjustmentValue for Column {
    #[inline]
    fn adjustment_insert_value(&mut self, root_num: u32, offset_num: u32) {
        if self.col_num.value() >= root_num {
            self.col_num
                .set_value(self.col_num.value() + offset_num);
        }
    }

    #[inline]
    fn adjustment_remove_value(&mut self, root_num: u32, offset_num: u32) {
        if self.col_num.value() >= root_num {
            self.col_num
                .set_value(self.col_num.value() - offset_num);
        }
    }

    #[inline]
    fn is_remove_value(&self, root_num: u32, offset_num: u32) -> bool {
        self.col_num.value() >= root_num && self.col_num.value() < root_num + offset_num
    }
}
