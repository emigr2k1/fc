use druid::widget::prelude::*;
use druid::{WidgetPod};

use crate::{
    data::AppData,
    widgets::shapes::rectangle::Rectangle,
};

pub struct Canvas {
    move_canvas_pressed: bool,
    last_mouse_pos: druid::Point,
    offset: druid::Vec2,
    scale: f64,
    scale_modifier_pressed: bool,

    rect: WidgetPod<AppData, Rectangle>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            move_canvas_pressed: false,
            last_mouse_pos: (0.0, 0.0).into(),
            offset: (0.0, 0.0).into(),
            scale: 1.0,
            scale_modifier_pressed: false,
            rect: WidgetPod::new(Rectangle::new()),
        }
    }
}

impl Widget<AppData> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        match event {
            druid::Event::MouseDown(m) => {
                if m.button.is_middle() {
                    self.last_mouse_pos = m.pos;
                    self.move_canvas_pressed = true;
                }
            },
            druid::Event::MouseUp(m) => {
                if m.button.is_middle() {
                    self.move_canvas_pressed = false;
                }
            },
            druid::Event::MouseMove(m) => if self.move_canvas_pressed {
                self.offset += m.pos - self.last_mouse_pos;
                self.last_mouse_pos = m.pos;
                ctx.request_paint();
            },
            druid::Event::Wheel(m) => {
                if m.wheel_delta.hypot2() == 0.0 {
                    return
                }

                let is_wheel_up = m.wheel_delta.y < 0.0;

                let scale_multiplier = match (is_wheel_up, self.scale_modifier_pressed) {
                    (true, true) => 1.01,
                    (true, false) => 1.2,
                    (false, true) => 0.99,
                    (false, false) => 0.8,
                };

                let prev_scale = self.scale;

                self.scale *= scale_multiplier;
                self.scale = self.scale.max(0.2).min(20.0);

                let mouse_pos = m.pos.to_vec2();
                // Get druid point under the mouse cursor.
                let druid_point = (mouse_pos - self.offset) / prev_scale;
                // Move `druid_point` to a position such that after the scale transformation, it
                // ends up again under the mouse cursor.
                // For that we calculate the position the point will get after scalation, and we
                // translate it to the mouse cursor position.
                self.offset = mouse_pos - (druid_point*self.scale);

                ctx.request_paint();
            },
            druid::Event::KeyDown(k) => {
                self.scale_modifier_pressed = k.code == druid::Code::ControlLeft;
            }
            druid::Event::KeyUp(k) => {
                self.scale_modifier_pressed = !(k.code == druid::Code::ControlLeft);
            }
            _ => {}
        }
        self.rect.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppData, env: &Env) {
        self.rect.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppData, data: &AppData, env: &Env) {
        self.rect.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppData,
        env: &Env,
    ) -> Size {
        let rect_size = self.rect.layout(ctx, bc, data, env);
        self.rect.set_layout_rect(ctx, data, env, druid::Rect::from_origin_size((20.0, 20.0), rect_size));
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, env: &Env) {
        let color = druid::Color::rgb(1.0, 0.1, 0.3);

        let size = ctx.size();
        let rect = druid::Rect::from_origin_size((0.0, 0.0), size);

        ctx.fill(&rect, &color);

        ctx.transform(druid::Affine::translate(self.offset));
        ctx.transform(druid::Affine::scale(self.scale));

        self.rect.paint(ctx, data, env);
    }
}
