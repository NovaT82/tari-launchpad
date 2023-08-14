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
    style::Color,
};
use rust_decimal::Decimal;

use crate::{
    component::{
        elements::{block_with_title, logo},
        normal::{
            chrono_button::{ChronoButton, ChronoGetter},
            mining::{
                amount::{AmountGetter, AmountIndicator},
                status_badge::{StatusBadge, StatusGetter},
            },
        },
        Component,
        ComponentEvent,
        Frame,
        Input,
        Pass,
    },
    state::{focus, AppState},
};

const LOGO: &str = r#"
╔╦╗┌─┐┬─┐┬  ╔╦╗┬┌┐┌┬┌┐┌┌─┐
 ║ ├─┤├┬┘│  ║║║│││││││││ ┬
 ╩ ┴ ┴┴└─┴  ╩ ╩┴┘└┘┴┘└┘└─┘
"#;

struct TariMiningGetter;

impl StatusGetter for TariMiningGetter {
    fn get_status(&self, state: &AppState) -> (&str, Color) {
        if state.state.config.session.is_miner_active() {
            ("(Running)", Color::Green)
        } else {
            ("(Ready to set)", Color::Cyan)
        }
    }
}

impl ChronoGetter for TariMiningGetter {
    fn get_duration(&self, _state: &AppState) -> Option<Duration> {
        None
    }

    fn get_label(&self, state: &AppState) -> &str {
        if state.state.config.session.is_miner_active() {
            "Pause"
        } else {
            "Start mining"
        }
    }
}

struct XtrGetter;

impl AmountGetter for XtrGetter {
    fn get_amount(&self, _state: &AppState) -> (Decimal, &str) {
        let amount = 0.into();
        (amount, "XTR")
    }
}

pub struct TariMiningWidget {
    status_badge: StatusBadge<TariMiningGetter>,
    tari_amount: AmountIndicator<XtrGetter>,
    button: ChronoButton<TariMiningGetter>,
}

impl TariMiningWidget {
    pub fn new() -> Self {
        Self {
            status_badge: StatusBadge::new(TariMiningGetter),
            tari_amount: AmountIndicator::new(XtrGetter),
            button: ChronoButton::new(TariMiningGetter),
        }
    }
}

impl Input for TariMiningWidget {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == focus::TARI_MINING {
            match event.pass() {
                Pass::Right | Pass::Next => {
                    state.focus_on(focus::MERGED_MINING);
                },
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Enter | Pass::Space => {
                    let session = &mut state.state.config.session;
                    session.miner_active = !session.miner_active;
                    state.update_state();
                },
                Pass::Tick => {
                    if state.state.config.session.is_miner_active() {
                        state.redraw();
                    }
                },
                _ => {},
            }
        }
    }
}

impl<B: Backend> Component<B> for TariMiningWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Tari Mining"), state.focus_on == focus::TARI_MINING);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let constraints = [
            Constraint::Length(1),
            Constraint::Length(3),
            // Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.status_badge.draw(f, v_chunks[0], state);

        let logo = logo(LOGO);
        f.render_widget(logo, v_chunks[1]);

        self.tari_amount.draw(f, v_chunks[2], state);

        self.button.draw(f, v_chunks[4], state);
    }
}
