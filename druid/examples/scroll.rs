// Copyright 2019 the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! Shows a scroll widget, and also demonstrates how widgets that paint
//! outside their bounds can specify their paint region.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use druid::kurbo::Circle;
use druid::piet::RadialGradient;
use druid::widget::prelude::*;
use druid::widget::{Flex, Padding};
use druid::{AppLauncher, Data, Insets, LocalizedString, Rect, WidgetExt, WindowDesc};

pub fn main() {
    let window = WindowDesc::new(build_widget())
        .title(LocalizedString::new("scroll-demo-window-title").with_placeholder("Scroll demo"));
    AppLauncher::with_window(window)
        .log_to_console()
        .launch(0u32)
        .expect("launch failed");
}

fn build_widget() -> impl Widget<u32> {
    let mut col = Flex::column();
    for i in 0..30 {
        col.add_child(Padding::new(3.0, OverPainter(i)));
    }
    col.scroll()
}

/// A widget that paints outside of its bounds.
struct OverPainter(u64);

const INSETS: Insets = Insets::uniform(50.);

impl<T: Data> Widget<T> for OverPainter {
    fn event(&mut self, _: &mut EventCtx, _: &Event, _: &mut T, _: &Env) {}

    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &T, _: &Env) {}

    fn update(&mut self, _: &mut UpdateCtx, _: &T, _: &T, _: &Env) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _: &T, _: &Env) -> Size {
        ctx.set_paint_insets(INSETS);
        bc.constrain(Size::new(100., 100.))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _: &T, env: &Env) {
        let rect = Rect::ZERO.with_size(ctx.size());
        let color = env.get_debug_color(self.0);
        let radius = (rect + INSETS).size().height / 2.0;
        let circle = Circle::new(rect.center(), radius);
        let grad = RadialGradient::new(1.0, (color, color.with_alpha(0.0)));
        ctx.fill(circle, &grad);
        ctx.stroke(rect, &color, 2.0);
    }
}
