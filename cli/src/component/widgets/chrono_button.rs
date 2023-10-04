// Copyright 2023. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use std::time::Duration;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    component::{elements::block_with_title, Component, ComponentEvent, Frame, Input, Pass},
    state::{AppState, Focus},
};

pub trait ChronoGetter {
    /// How long the mining is active.
    fn get_duration(&self, _state: &AppState) -> Option<Duration> {
        None
    }

    fn get_label(&self, state: &AppState) -> &str;
}

/// A button with a clock.
pub struct ChronoButton<G> {
    getter: G,
    focus: Focus,
}

impl<G> ChronoButton<G> {
    pub fn new(getter: G, focus: Focus) -> Self {
        Self { getter, focus }
    }
}

pub enum ButtonAction {
    Clicked,
}

impl<G> Input for ChronoButton<G> {
    type Output = ButtonAction;

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        let mut action = None;
        if state.focus_on == self.focus && event.pass() == Pass::Enter {
            action = Some(ButtonAction::Clicked);
        }
        action
    }
}

impl<B: Backend, G> Component<B> for ChronoButton<G>
where G: ChronoGetter
{
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(3), Constraint::Min(0)];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);

        let caption;
        let label = self.getter.get_label(state);
        if let Some(dur) = self.getter.get_duration(state) {
            let total = dur.as_secs();
            let seconds = total % 60;
            let total = total / 60;
            let minutes = total % 60;
            let hours = total / 60;
            caption = format!("  {:02}:{:02}:{:02} | {}  ", hours, minutes, seconds, label);
        } else {
            caption = format!("  {}  ", label);
        }

        let width = caption.len() as u16 + 2; // caption + border
        let constraints = [Constraint::Min(width), Constraint::Min(0)];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(v_chunks[0]);
        let block_color = {
            if state.focus_on == self.focus {
                Color::Magenta
            } else {
                Color::White
            }
        };
        let block = block_with_title(None, false).border_style(Style::default().fg(block_color));
        let inner_rect = block.inner(h_chunks[0]);
        f.render_widget(block, h_chunks[0]);

        let line = Line::from(vec![Span::styled(
            // "  Set up & start mining  ",
            // "  Start mining  ",
            caption,
            Style::default(), //.bg(Color::Magenta),
        )]);
        let text = vec![line];
        let p = Paragraph::new(text);
        f.render_widget(p, inner_rect);
    }
}
