pub const T_TOML: &str = r#"
[theme]
name="termplate"
description="a nice termplate description"
author="who?"
version="1.0.0"
git="https://github.com/your/repo"
depends=["waybar"]
config="theme.conf"

[script]
cleanup="./cleanup"

[kill]
exclude_bars=["eww"]
exclude_wallpapers=["swww"]
"#;


pub const T_CONF: &str = r#"
exec = $THEME_DIR/load $THEME_DIR

general {
    gaps_in=8
    gaps_out=15
    border_size=3
    col.active_border=rgba(cba6f7ff) rgba(89b4faff) rgba(94e2d5ff) 10deg
    col.inactive_border=0xff45475a
}

decoration {
    blur_new_optimizations = true
    drop_shadow = true
    shadow_range=100
    shadow_render_power=5
    col.shadow= 0x33000000
    col.shadow_inactive=0x22000000
    rounding=15
    blur=0
    blur_size=1
    blur_passes=1
}

animations {
    enabled=1
    bezier=overshot,0.13,0.99,0.29,1.1
    animation=windows,1,4,overshot,slide
    animation=border,1,10,default
    animation=fade,1,10,default
    animation=workspaces,1,6,overshot,slidevert
}

dwindle {
    col.group_border=0xff89dceb
    col.group_border_active=0xfff9e2af
}
"#;

pub const T_CLEANUP: &str = r#"
#!/usr/bin/bash

pkill cava
"#;

pub const T_LOAD: &str = r#"
#!/usr/bin/bash

THEME_DIR=$1

swww init
swww img $THEME_DIR/wallpapers/4.jpg --transition-type grow --transition-pos "$(hyprctl cursorpos)"

waybar
"#;

pub const A_CONF: &str = r#"
#====== autogenerated by hyprtheme ======#
source = ~/.config/hypr/themes/dist/dist.conf
"#;