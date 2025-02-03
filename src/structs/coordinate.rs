use std::fmt;

use super::{
    ColumnReference,
    RowReference,
};
use crate::{
    helper::coordinate::{
        coordinate_from_index_with_lock,
        index_from_coordinate,
    },
    traits::{
        AdjustmentCoordinate,
        AdjustmentValue,
    },
};

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coordinate {
    column: ColumnReference,
    row:    RowReference,
}

impl fmt::Display for Coordinate {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            coordinate_from_index_with_lock(
                self.column.num(),
                self.row.num(),
                self.column.is_lock(),
                self.row.is_lock(),
            )
        )
    }
}

impl Coordinate {
    #[inline]
    #[must_use]
    pub fn col_num(&self) -> u32 {
        self.column.num()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use col_num()")]
    pub fn get_col_num(&self) -> u32 {
        self.col_num()
    }

    #[inline]
    pub fn set_col_num(&mut self, value: u32) -> &mut Self {
        self.column.set_num(value);
        self
    }

    #[inline]
    pub(crate) fn offset_col_num(&mut self, value: i32) -> &mut Self {
        self.column.offset_num(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn row_num(&self) -> u32 {
        self.row.num()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use row_num()")]
    pub fn get_row_num(&self) -> u32 {
        self.row_num()
    }

    #[inline]
    pub fn set_row_num(&mut self, value: u32) -> &mut Self {
        self.row.set_num(value);
        self
    }

    #[inline]
    pub(crate) fn offset_row_num(&mut self, value: i32) -> &mut Self {
        self.row.offset_num(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn is_lock_col(&self) -> bool {
        self.column.is_lock()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use is_lock_col()")]
    pub fn get_is_lock_col(&self) -> bool {
        self.is_lock_col()
    }

    #[inline]
    pub fn set_is_lock_col(&mut self, value: bool) -> &mut Self {
        self.column.set_is_lock(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn is_lock_row(&self) -> bool {
        self.row.is_lock()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use is_lock_row()")]
    pub fn get_is_lock_row(&self) -> bool {
        self.is_lock_row()
    }

    #[inline]
    pub fn set_is_lock_row(&mut self, value: bool) -> &mut Self {
        self.row.set_is_lock(value);
        self
    }

    /// Change coordinates
    /// Formula is not updated.
    #[inline]
    pub fn set_coordinate<S: AsRef<str>>(&mut self, value: S) -> &mut Self {
        let (c, r, cl, rl) = index_from_coordinate(value.as_ref());

        self.column.set_num(c.unwrap());
        self.row.set_num(r.unwrap());
        self.column.set_is_lock(cl.unwrap());
        self.row.set_is_lock(rl.unwrap());

        self
    }

    #[inline]
    #[must_use]
    pub fn get_coordinate(&self) -> String {
        coordinate_from_index_with_lock(
            self.column.num(),
            self.row.num(),
            self.column.is_lock(),
            self.row.is_lock(),
        )
    }
}
impl AdjustmentCoordinate for Coordinate {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.column
            .adjustment_insert_value(root_col_num, offset_col_num);
        self.row
            .adjustment_insert_value(root_row_num, offset_row_num);
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.column
            .adjustment_remove_value(root_col_num, offset_col_num);
        self.row
            .adjustment_remove_value(root_row_num, offset_row_num);
    }

    #[inline]
    fn is_remove_coordinate(
        &self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) -> bool {
        self.column.is_remove_value(root_col_num, offset_col_num)
            || self.row.is_remove_value(root_row_num, offset_row_num)
    }
}
