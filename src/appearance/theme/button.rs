use data::appearance::theme::ButtonStyle;
use iced::widget::button::{Catalog, Status, Style, StyleFn};
use iced::{Background, Border, Color};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

fn default(theme: &Theme, status: Status) -> Style {
    primary(theme, status, false)
}

fn button(
    foreground: Color,
    foreground_hover: Color,
    foreground_pressed: Color,
    background: Color,
    background_hover: Color,
    background_pressed: Color,
    border_color: Option<Color>,
    border_color_pressed: Option<Color>,
    status: Status,
) -> Style {
    let border = |border_color: Option<Color>| Border {
        radius: 4.0.into(),
        color: border_color.unwrap_or(Color::TRANSPARENT),
        width: 1.0,
    };

    match status {
        Status::Active => Style {
            background: Some(Background::Color(background)),
            text_color: foreground,
            border: border(border_color),
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(Background::Color(background_hover)),
            text_color: foreground_hover,
            border: border(border_color),
            ..Default::default()
        },
        Status::Pressed => Style {
            background: Some(Background::Color(background_pressed)),
            text_color: foreground_pressed,
            border: border(border_color_pressed),
            ..Default::default()
        },
        Status::Disabled => {
            let active: Style = button(
                foreground,
                foreground_hover,
                foreground_pressed,
                background,
                background_hover,
                background_pressed,
                border_color,
                border_color_pressed,
                Status::Active,
            );

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                ..active
            }
        }
    }
}

pub fn sidebar_buffer(
    theme: &Theme,
    status: Status,
    is_focused: bool,
    is_open: bool,
) -> Style {
    let default_foreground = theme.styles().text.primary.color;
    let button_colors = theme.styles().buttons.primary;

    let button_style = ButtonStyle {
        selected: is_open && is_focused,
        hover: is_open && !is_focused,
    };
    let button_style_hover = ButtonStyle {
        selected: is_open,
        hover: !button_style.hover,
    };
    let button_style_pressed = ButtonStyle {
        selected: true,
        hover: false,
    };

    let foreground = None;
    let foreground_hover = None;
    let foreground_pressed = None;

    let background = *button_colors.background(button_style);
    let background_hover = *button_colors.background(button_style_hover);
    let background_pressed = *button_colors.background(button_style_pressed);

    let foreground = foreground.unwrap_or(default_foreground);
    let foreground_hover = foreground_hover.unwrap_or(default_foreground);
    let foreground_pressed = foreground_pressed.unwrap_or(default_foreground);

    let border_color_base = button_colors
        .border_active
        .unwrap_or(theme.styles().buffer.border_selected);
    let border_color = is_focused.then_some(border_color_base);
    let border_color_pressed = (!is_focused).then_some(border_color_base);

    button(
        foreground,
        foreground_hover,
        foreground_pressed,
        background,
        background_hover,
        background_pressed,
        border_color,
        border_color_pressed,
        status,
    )
}

pub fn primary(theme: &Theme, status: Status, selected: bool) -> Style {
    let default_foreground = theme.styles().text.primary.color;
    let button_colors = theme.styles().buttons.primary;

    let button_style = ButtonStyle {
        selected,
        hover: false,
    };
    let button_style_hover = ButtonStyle {
        selected: button_style.selected,
        hover: !button_style.hover,
    };
    let button_style_pressed = ButtonStyle {
        selected: !button_style_hover.selected,
        hover: button_style_hover.hover,
    };

    let foreground = None;
    let foreground_hover = None;
    let foreground_pressed = None;

    let background = *button_colors.background(button_style);
    let background_hover = *button_colors.background(button_style_hover);
    let background_pressed = *button_colors.background(button_style_pressed);

    let foreground = foreground.unwrap_or(default_foreground);
    let foreground_hover = foreground_hover.unwrap_or(default_foreground);
    let foreground_pressed = foreground_pressed.unwrap_or(default_foreground);

    let border_color = None;
    let border_color_pressed = None;

    button(
        foreground,
        foreground_hover,
        foreground_pressed,
        background,
        background_hover,
        background_pressed,
        border_color,
        border_color_pressed,
        status,
    )
}

pub fn secondary(theme: &Theme, status: Status, selected: bool) -> Style {
    let default_foreground = theme.styles().text.secondary.color;
    let button_colors = theme.styles().buttons.secondary;

    let button_style = ButtonStyle {
        selected,
        hover: false,
    };
    let button_style_hover = ButtonStyle {
        selected: button_style.selected,
        hover: !button_style.hover,
    };
    let button_style_pressed = ButtonStyle {
        selected: !button_style_hover.selected,
        hover: button_style_hover.hover,
    };

    let foreground = default_foreground;
    let foreground_hover = default_foreground;
    let foreground_pressed = default_foreground;

    let background = *button_colors.background(button_style);
    let background_hover = *button_colors.background(button_style_hover);
    let background_pressed = *button_colors.background(button_style_pressed);

    let border_color = None;
    let border_color_pressed = None;

    button(
        foreground,
        foreground_hover,
        foreground_pressed,
        background,
        background_hover,
        background_pressed,
        border_color,
        border_color_pressed,
        status,
    )
}

pub fn picker(theme: &Theme, status: Status, is_selected: bool) -> Style {
    let default_foreground = theme.styles().text.primary.color;
    let button_colors = theme.styles().buttons.primary;

    let button_style = ButtonStyle {
        selected: false,
        hover: is_selected,
    };
    let button_style_hover = ButtonStyle {
        selected: is_selected,
        hover: !button_style.hover,
    };
    let button_style_pressed = ButtonStyle {
        selected: !button_style.selected,
        hover: button_style.hover,
    };

    let foreground = None;
    let foreground_hover = None;
    let foreground_pressed = None;

    let background = *button_colors.background(button_style);
    let background_hover = *button_colors.background(button_style_hover);
    let background_pressed = *button_colors.background(button_style_pressed);

    let foreground = foreground.unwrap_or(default_foreground);
    let foreground_hover = foreground_hover.unwrap_or(default_foreground);
    let foreground_pressed = foreground_pressed.unwrap_or(default_foreground);

    let border_color = None;
    let border_color_pressed = None;

    button(
        foreground,
        foreground_hover,
        foreground_pressed,
        background,
        background_hover,
        background_pressed,
        border_color,
        border_color_pressed,
        status,
    )
}

pub fn reaction(
    theme: &Theme,
    status: Status,
    already_reacted: bool,
    selection: bool,
) -> Style {
    let default_foreground = theme.styles().text.secondary.color;
    let button_colors = theme.styles().buttons.secondary;

    let button_style = ButtonStyle {
        selected: selection,
        hover: false,
    };
    let button_style_hover = ButtonStyle {
        selected: button_style.selected,
        hover: !button_style.hover,
    };
    let button_style_pressed = ButtonStyle {
        selected: true,
        hover: !button_style_hover.hover,
    };

    let foreground = None;
    let foreground_hover = None;
    let foreground_pressed = None;

    let background = *button_colors.background(button_style);
    let background_hover = *button_colors.background(button_style_hover);
    let background_pressed = *button_colors.background(button_style_pressed);

    let foreground = foreground.unwrap_or(default_foreground);
    let foreground_hover = foreground_hover.unwrap_or(default_foreground);
    let foreground_pressed = foreground_pressed.unwrap_or(default_foreground);

    let border_color_base =
        |foreground| button_colors.border_active.unwrap_or(foreground);
    let border_color = already_reacted.then_some(border_color_base(foreground));
    let border_color_pressed =
        (!already_reacted).then_some(border_color_base(foreground_pressed));

    button(
        foreground,
        foreground_hover,
        foreground_pressed,
        background,
        background_hover,
        background_pressed,
        border_color,
        border_color_pressed,
        status,
    )
}

pub fn bare(_theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed | Status::Hovered => Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        },
        Status::Disabled => {
            let active = bare(_theme, Status::Active);

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                border: Border {
                    color: Color {
                        a: 0.2,
                        ..active.text_color
                    },
                    ..Default::default()
                },
                ..active
            }
        }
    }
}

pub fn reply_preview(theme: &Theme, status: Status) -> Style {
    let text = theme.styles().text;
    Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: match status {
            Status::Active | Status::Pressed => text.secondary.color,
            Status::Hovered => text.primary.color,
            Status::Disabled => Color {
                a: 0.2,
                ..text.secondary.color
            },
        },
        ..Default::default()
    }
}

pub fn preview_card(theme: &Theme, status: Status) -> Style {
    let default_foreground = theme.styles().text.primary.color;
    let button_colors = theme.styles().buttons.secondary;

    let button_style = ButtonStyle {
        selected: false,
        hover: false,
    };
    let button_style_hover = ButtonStyle {
        selected: button_style.selected,
        hover: !button_style.hover,
    };

    let foreground = None;
    let foreground_hover = None;

    let background = *button_colors.background(button_style);
    let background_hover = *button_colors.background(button_style_hover);

    let foreground = foreground.unwrap_or(default_foreground);
    let foreground_hover = foreground_hover.unwrap_or(default_foreground);

    let border = Border {
        radius: 4.0.into(),
        width: 1.0,
        color: theme.styles().general.border,
    };

    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(background)),
            text_color: foreground,
            border,
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(Background::Color(background_hover)),
            text_color: foreground_hover,
            border,
            ..Default::default()
        },
        // Not possible to disable this button style.
        Status::Disabled => self::primary(theme, status, false),
    }
}
