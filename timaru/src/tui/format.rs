use chrono::Datelike;
use tui::{
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::{schedule::Schedule, task::Task};

impl Schedule {
    pub fn as_widget_paragraph(&self) -> Paragraph {
        let mut text = vec![
            Spans::from(Span::raw(format!(
                "# {}-{}-{}",
                self.date.day(),
                self.date.month(),
                self.date.year()
            ))),
            Spans::from(Span::raw("")),
        ];

        for task in self.tasks.values() {
            text.push(Spans::from(task.as_tui_span()));
        }

        Paragraph::new(text)
    }
}

impl Task {
    #[inline]
    pub fn as_tui_span(&self) -> Span {
        Span::raw(self.as_string())
    }
}
