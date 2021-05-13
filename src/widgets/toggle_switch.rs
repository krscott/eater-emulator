use eframe::egui::{self, Response, Sense, Ui, Widget, WidgetInfo, WidgetType};

// Based on https://github.com/emilk/egui/blob/master/egui_demo_lib/src/apps/demo/toggle_switch.rs

fn toggle_switch_ui(ui: &mut Ui, on: &mut bool) -> Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(1.0, 2.0);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }

    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, *on, ""));

    let anim_on = ui.ctx().animate_bool(response.id, *on);

    let visuals = ui.style().interact_selectable(&response, *on);

    let rect = rect.expand(visuals.expansion);

    let radius = 0.5 * rect.width();

    ui.painter()
        .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);

    let circle_y = egui::lerp(
        (rect.top() + radius)..=(rect.bottom() - radius),
        1.0 - anim_on,
    );
    let center = egui::pos2(rect.center().x, circle_y);
    ui.painter()
        .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);

    response
}

pub fn toggle_switch(on: &mut bool) -> impl Widget + '_ {
    move |ui: &mut Ui| toggle_switch_ui(ui, on)
}
