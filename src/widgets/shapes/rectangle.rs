use std::sync::Arc;

use druid::widget::prelude::*;
use druid::{Data, WidgetPod, widget::TextBox};

#[derive(Clone, Data)]
struct Assignment {
    ident: String,
    value: String,
}

pub struct Rectangle {
    text_widget: WidgetPod<Arc<String>, TextBox<Arc<String>>>,
    text: Arc<String>,
    text_size: Size,
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            text_widget: WidgetPod::new(TextBox::new().with_placeholder("Write here")),
            text: Arc::new(String::from("hey")),
            text_size: (0.0, 0.0).into(),
        }
    }
}

impl Widget<crate::AppData> for Rectangle {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut crate::AppData, env: &Env) {
        self.text_widget.event(ctx, event, &mut self.text, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &crate::AppData, env: &Env) {
        self.text_widget.lifecycle(ctx, event, &self.text, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &crate::AppData, _data: &crate::AppData, env: &Env) {
        self.text_widget.update(ctx, &self.text, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &crate::AppData, env: &Env) -> Size {
        self.text_size = (100.0, 100.0).into();
        self.text_widget.set_layout_rect(ctx, &self.text, env, druid::Rect::from_origin_size((0.0, 0.0), self.text_size));
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &crate::AppData, env: &Env) {
        let color = druid::Color::rgb(1.0, 1.0, 1.0);

        let size = self.text_size + (10.0, 10.0).into();
        let rect = druid::Rect::from_origin_size((-5.0, -5.0), size);

        ctx.fill(&rect, &color);
        ctx.stroke(&rect, &druid::Color::rgb(0.0, 0.0, 0.0), 3.0);
        self.text_widget.paint(ctx, &self.text, env);
    }
}

