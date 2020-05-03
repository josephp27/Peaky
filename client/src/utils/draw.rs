use crate::utils::helper::get_canvas;
use crate::utils::settings::Settings;

pub fn draw(settings: Settings) {
    let canvas = get_canvas(settings.width, settings.height, settings.scalar);
    loop {}
}