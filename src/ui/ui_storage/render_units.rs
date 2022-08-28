use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders, Paragraph, Clear, Table, Row},
    layout::{Rect/*, Alignment*/},
    // layout::{Layout, Constraint, Direction, Rect},
    Terminal,
    // text,
    Frame,
    self
};

use crate::utils::CrusterError;

pub(crate) enum RenderUnits {
    WIDGET,
    PARAGRAPH,
    TABLE,
    CLEAR
}

// TUI's Block Render Unit
#[derive(Clone, Debug)]
pub(crate) struct BlockRenderUnit<'bru> {
    pub(crate) widget: Block<'bru>,
    pub(crate) clear_widget: Clear,
    pub(crate) rect_index: usize,
    pub(crate) is_active: bool
}

impl<'a> BlockRenderUnit<'a> {
    fn set_active_status(& mut self, status: bool) {
        self.is_active = status;
    }
}

// ---------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug)]
pub(crate) struct ParagraphRenderUnit<'pru> {
    pub(crate) widget: Paragraph<'pru>,
    pub(crate) rect_index: usize,
    pub(crate) clear_widget: Clear,
    pub(crate) is_active: bool,
    pub(crate) scroll: (u16, u16)
}

impl<'a> ParagraphRenderUnit<'a> {
    fn set_active_status(& mut self, status: bool) {
        self.is_active = status;
    }
}

// ---------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug)]
pub(crate) struct TableRenderUnit<'tru> {
    pub(crate) widget: Table<'tru>,
    pub(crate) clear_widget: Clear,
    pub(crate) rect_index: usize,
    pub(crate) is_active: bool
}

impl<'a> TableRenderUnit<'a> {
    fn set_active_status(& mut self, status: bool) {
        self.is_active = status;
    }
}

// ---------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug)]
pub(crate) struct ClearRenderUnit {
    pub(crate) widget: Clear,
    pub(crate) rect_index: usize,
    pub(crate) is_active: bool
}

impl<'a> ClearRenderUnit {
    fn set_active_status(& mut self, status: bool) {
        self.is_active = status;
    }
}

// ---------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug)]
pub(crate) enum RenderUnit<'ru_lt> {
    TUIBlock(BlockRenderUnit<'ru_lt>),
    TUIParagraph(ParagraphRenderUnit<'ru_lt>),
    TUITable(TableRenderUnit<'ru_lt>),
    TUIClear(ClearRenderUnit),
    PLACEHOLDER
}

impl Default for RenderUnit<'_> {
    fn default() -> Self {
        RenderUnit::TUIBlock(
            BlockRenderUnit {
                widget: Block::default(),
                clear_widget: Clear,
                rect_index: 0,
                is_active: true
            }
        )
    }
}

impl RenderUnit<'_> {
    pub(crate) fn new_block(widget: Block<'_>, rect_index: usize, is_active: bool) -> RenderUnit<'_> {
        RenderUnit::TUIBlock(BlockRenderUnit { widget, clear_widget: Clear, rect_index, is_active })
    }

    pub(crate) fn new_paragraph(widget: Paragraph<'_>, rect_index: usize, is_active: bool, scroll: (u16, u16)) -> RenderUnit<'_> {
        RenderUnit::TUIParagraph(ParagraphRenderUnit { widget, clear_widget: Clear, rect_index, is_active, scroll})
    }

    pub(crate) fn new_table(widget: Table<'_>, rect_index: usize, is_active: bool) -> RenderUnit<'_> {
        RenderUnit::TUITable(TableRenderUnit { widget, clear_widget: Clear, rect_index, is_active })
    }

    pub(crate) fn new_clear<'ru_lt>(rect_index: usize) -> RenderUnit<'ru_lt> {
        RenderUnit::TUIClear(ClearRenderUnit {widget: Clear, rect_index, is_active: true})
    }

    pub(crate) fn is_widget_active(&self) -> bool {
        return match self {
            RenderUnit::TUIBlock(block) => block.is_active,
            RenderUnit::TUIParagraph(paragraph) => paragraph.is_active,
            RenderUnit::TUITable(table) => table.is_active,
            RenderUnit::TUIClear(clear) => clear.is_active,
            _ => false
        };
    }

    pub(crate) fn enable(&mut self) {
        match self {
            RenderUnit::TUIBlock(block) => block.is_active = true,
            RenderUnit::TUIParagraph(paragraph) => paragraph.is_active = true,
            RenderUnit::TUITable(table) => table.is_active = true,
            RenderUnit::TUIClear(clear) => clear.is_active = true,
            _ => ()
        };
    }

    pub(crate) fn disable(&mut self) {
        match self {
            RenderUnit::TUIBlock(block) => block.is_active = false,
            RenderUnit::TUIParagraph(paragraph) => paragraph.is_active = false,
            RenderUnit::TUITable(table) => table.is_active = false,
            RenderUnit::TUIClear(clear) => clear.is_active = false,
            _ => ()
        };
    }

    pub(crate) fn set_rect_index(&mut self, new_index: usize) {
        match self {
            RenderUnit::TUIBlock(block) => block.rect_index = new_index,
            RenderUnit::TUIParagraph(paragraph) => paragraph.rect_index = new_index,
            RenderUnit::TUITable(table) => table.rect_index = new_index,
            RenderUnit::TUIClear(clear) => clear.rect_index = new_index,
            _ => ()
        };
    }

    pub(crate) fn paragraph_get_scroll(&self) -> Option<(u16, u16)> {
        return match self {
            RenderUnit::TUIParagraph(paragraph) => Some(paragraph.scroll),
            _ => None
        }
    }

    pub(crate) fn paragraph_set_scroll(&mut self, scroll: (u16, u16)) -> Result<(), CrusterError> {
        match self {
            RenderUnit::TUIParagraph(paragraph) => paragraph.scroll = scroll,
            _ => return Err(CrusterError::NotParagraphRenderUnit("Cannot assign 'scroll' to not a paragraph".to_string()))
        }

        Ok(())
    }

    pub(crate) fn paragraph_reset_scroll(&mut self) -> Result<(), CrusterError> {
        self.paragraph_set_scroll((0, 0))
    }
}