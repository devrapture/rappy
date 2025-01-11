use console::Style;
use dialoguer::theme::ColorfulTheme;

pub fn get_theme() -> ColorfulTheme {
    ColorfulTheme {
        values_style: Style::new().cyan().dim(),
        ..ColorfulTheme::default()
    }
}
