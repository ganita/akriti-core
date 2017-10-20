
#[derive(Debug, Clone)]
pub enum VAlign {
    Top,
    Bottom,
    Center,
    Baseline,
    Axis
}

#[derive(Debug, Clone)]
pub struct TableVAlign {
    align: VAlign,
    row_number: Option<i32>
}

impl TableVAlign {
    pub fn new(align: VAlign, row_number: Option<i32>) -> TableVAlign {
        TableVAlign {
            align,
            row_number
        }
    }

    pub fn align(&self) -> &VAlign {
        &self.align
    }

    pub fn row_number(&self) -> &Option<i32> {
        &self.row_number
    }
}