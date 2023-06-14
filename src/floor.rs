pub fn calculate_panels(room_x: f32, room_y: f32, panel_x: f32, panel_y: f32) -> i32 {
    let room_area: f32 = room_x * room_y;
    let panel_area: f32 = panel_x * panel_y;

    let num_panels_float: f32 = room_area / panel_area;

    num_panels_float.ceil() as i32
}