use imgui::*;
mod support;

const HEIGHT: f32 = 100.0;

struct State {
    esp_enabled: bool,
    show_boxes: bool,
    show_names: bool,
    show_health: bool,
    aimbot_enabled: bool,
    aim_at_head: bool,
    smooth_aim: bool,
    bunnyhop_enabled: bool,
    no_recoil: bool,
    third_person_view: bool,
    smooth_factor: f32,
    fov_changer: f32,
}

fn main() {
    println!("Hello, world!");

    let mut state = State {
        esp_enabled: false,
        show_boxes: false,
        show_names: false,
        show_health: false,
        aimbot_enabled: false,
        aim_at_head: false,
        smooth_aim: false,
        bunnyhop_enabled: false,
        no_recoil: false,
        third_person_view: false,
        smooth_factor: 2.0,
        fov_changer: 90.0,
    };

    let system = support::init(file!());
    system.main_loop(move |_, ui| {

        Window::new(im_str!("Cheat Menu"))
            .size([300.0, 400.0], Condition::FirstUseEver)
            .build(ui, || {
                TabBar::new(im_str!("cheat_tabs")).build(&ui, || {
                    TabItem::new(im_str!("ESP")).build(&ui, || {
                        ui.checkbox(im_str!("Enable ESP"), &mut state.esp_enabled);
                        ui.text(im_str!("ESP Settings:"));
                        ui.checkbox(im_str!("Show Boxes"), &mut state.show_boxes);
                        ui.checkbox(im_str!("Show Names"), &mut state.show_names);
                        ui.checkbox(im_str!("Show Health"), &mut state.show_health);
                    });
                    TabItem::new(im_str!("Aimbot")).build(&ui, || {
                        ui.checkbox(im_str!("Enable Aimbot"), &mut state.aimbot_enabled);
                        ui.text(im_str!("Aimbot Settings:"));
                        ui.checkbox(im_str!("Aim at Head"), &mut state.aim_at_head);
                        ui.checkbox(im_str!("Smooth Aim"), &mut state.smooth_aim);
                        Slider::new(im_str!("Smooth Factor"))
                            .range(1.0..=10.0)
                            .build(ui, &mut state.smooth_factor);
                    });
                    TabItem::new(im_str!("Misc")).build(&ui, || {
                        ui.checkbox(im_str!("Enable Bunnyhop"), &mut state.bunnyhop_enabled);
                        ui.checkbox(im_str!("No Recoil"), &mut state.no_recoil);
                        ui.text(im_str!("Misc Settings:"));
                        ui.checkbox(im_str!("Third Person View"), &mut state.third_person_view);
                        Slider::new(im_str!("FOV Changer"))
                            .range(60.0..=120.0)
                            .build(ui, &mut state.fov_changer);
                    });
                });
            });
    });
}
