use std::ops::Index;
use tui::layout::Rect;

pub(super) struct CrusterLayout {
    rects: [Rect; 7]
}

impl CrusterLayout {
    pub(super) fn new(f: &Rect) -> Self {
        let window_width = f.width;
        let window_height = f.height;

        CrusterLayout {
            // 0 - Rect for requests log,
            // 1 - Rect for requests
            // 2 - Rect for responses
            // 3 - Rect for statusbar
            // 4 - Rect for help menu
            // 5 - Rect for Proxy FullScreen
            // 6 - Rect for confirmation window
            rects: [
                // 0
                Rect::new(
                    f.x,
                    f.y,
                    window_width - 1,
                    window_height / 2
                ),
                // 1
                Rect::new(
                    f.x,
                    f.y + window_height / 2,
                    window_width / 2 - 1,
                    window_height / 2 - 2
                ),
                // 2
                Rect::new(
                    f.x + window_width / 2,
                    f.y + window_height / 2,
                    window_width / 2,
                    window_height / 2 - 2
                ),
                // 3
                Rect::new(
                    f.x,
                    f.y + window_height - 2,
                    window_width - 1,
                    2
                ),
                // 4
                Rect::new(
                    f.x + 5,
                    f.y + 5,
                    window_width - 10,
                    window_height - 10
                ),
                // 5
                Rect::new(
                    f.x,
                    f.y,
                    window_width - 1,
                    window_height - 2
                ),
                // 6
                Rect::new(
                    f.x + window_width / 4 + window_width / 8,
                    f.y + window_height / 4 + window_height / 8,
                    window_width / 4,
                    window_height / 4
                )
            ]
        }
    }
}

impl Index<usize> for CrusterLayout {
    type Output = Rect;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rects[index]
    }
}