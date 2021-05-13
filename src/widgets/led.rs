use eframe::egui::{
    vec2, Align2, Color32, Response, Sense, TextStyle, Ui, Widget, WidgetInfo, WidgetType,
};

fn led_ui(ui: &mut Ui, on: &mut bool, color: Color32, label: String) -> Response {
    let desired_size = ui.spacing().interact_size.y * vec2(1.0, 1.0);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }

    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, *on, &label));

    let anim_on = ui.ctx().animate_bool(response.id, *on);

    let visuals = ui.style().interact_selectable(&response, *on);

    let rect = rect.expand(visuals.expansion);

    let center = rect.center();
    let radius = 0.5 * rect.height();

    let fill_color = lerp_color_rgb(Color32::BLACK, color, anim_on);

    ui.painter()
        .circle(center, radius, fill_color, visuals.fg_stroke);

    ui.painter().text(
        center,
        Align2::CENTER_CENTER,
        &label,
        TextStyle::Button,
        ui.style().visuals.text_color(),
    );

    response
}

pub fn led<S: Into<String>>(on: &mut bool, color: Color32, label: S) -> impl Widget + '_ {
    let label: String = label.into();
    move |ui: &mut Ui| led_ui(ui, on, color, label)
}

fn color_to_f32_tuple(color: Color32) -> (f32, f32, f32, f32) {
    (
        f32::from(color.r()),
        f32::from(color.g()),
        f32::from(color.b()),
        f32::from(color.a()),
    )
}

fn sq_lerp(a: f32, b: f32, t: f32) -> f32 {
    ((1.0 - t) * a * a + t * b * b).sqrt()
}

fn lerp_color_rgb(color_a: Color32, color_b: Color32, t: f32) -> Color32 {
    let (ra, ga, ba, _) = color_to_f32_tuple(color_a);
    let (rb, gb, bb, _) = color_to_f32_tuple(color_b);

    let r = sq_lerp(ra, rb, t);
    let g = sq_lerp(ga, gb, t);
    let b = sq_lerp(ba, bb, t);

    Color32::from_rgb(r as u8, g as u8, b as u8)
}
