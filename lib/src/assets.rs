use bevy::prelude::*;
use bevy::render::texture::{CompressedImageFormats, ImageSampler, ImageType};

const BOLD_FONT: &[u8] = include_bytes!("../../assets/Poppins/Poppins-Bold.ttf");
const MEDIUM_FONT: &[u8] = include_bytes!("../../assets/Poppins/Poppins-Medium.ttf");
const LIGHT_FONT: &[u8] = include_bytes!("../../assets/Poppins/Poppins-Light.ttf");

const XIAOWEI_REGULAR_FONT: &[u8] = include_bytes!("../../assets/XiaoWei/ZCOOLXiaoWei-Regular.ttf");

const COG: &[u8] = include_bytes!("../../assets/cog.png");
const COG_PRESSED: &[u8] = include_bytes!("../../assets/cog_pressed.png");
const LAUNCH_SCREEN: &[u8] = include_bytes!("../../assets/launch_screen.png");
const LAUNCH_SCREEN_IPAD: &[u8] = include_bytes!("../../assets/launch_screen_ipad.png");
const MODE_SLIDER: &[u8] = include_bytes!("../../assets/mode_slider.png");
const SLICE_1: &[u8] = include_bytes!("../../assets/slice_1.png");
const SLICE_2: &[u8] = include_bytes!("../../assets/slice_2.png");
const SLICE_3: &[u8] = include_bytes!("../../assets/slice_3.png");
const SLICE_4: &[u8] = include_bytes!("../../assets/slice_4.png");
const SLICE_5: &[u8] = include_bytes!("../../assets/slice_5.png");
const SLICE_6: &[u8] = include_bytes!("../../assets/slice_6.png");
const SLICE_7: &[u8] = include_bytes!("../../assets/slice_7.png");
const SLICE_8: &[u8] = include_bytes!("../../assets/slice_8.png");
const SLICE_9: &[u8] = include_bytes!("../../assets/slice_9.png");
const SLICE_DISABLED_1: &[u8] = include_bytes!("../../assets/slice_disabled_1.png");
const SLICE_DISABLED_2: &[u8] = include_bytes!("../../assets/slice_disabled_2.png");
const SLICE_DISABLED_3: &[u8] = include_bytes!("../../assets/slice_disabled_3.png");
const SLICE_DISABLED_4: &[u8] = include_bytes!("../../assets/slice_disabled_4.png");
const SLICE_DISABLED_5: &[u8] = include_bytes!("../../assets/slice_disabled_5.png");
const SLICE_DISABLED_6: &[u8] = include_bytes!("../../assets/slice_disabled_6.png");
const SLICE_DISABLED_7: &[u8] = include_bytes!("../../assets/slice_disabled_7.png");
const SLICE_DISABLED_8: &[u8] = include_bytes!("../../assets/slice_disabled_8.png");
const SLICE_DISABLED_9: &[u8] = include_bytes!("../../assets/slice_disabled_9.png");
const TOGGLE_SELECT_1: &[u8] = include_bytes!("../../assets/toggle_select_1.png");
const TOGGLE_SELECT_2: &[u8] = include_bytes!("../../assets/toggle_select_2.png");
const TOGGLE_SELECT_3: &[u8] = include_bytes!("../../assets/toggle_select_3.png");
const TOGGLE_SELECT_4: &[u8] = include_bytes!("../../assets/toggle_select_4.png");
const TOGGLE_SELECT_5: &[u8] = include_bytes!("../../assets/toggle_select_5.png");
const TOGGLE_SELECTED: &[u8] = include_bytes!("../../assets/toggle_selected.png");
const TOGGLE_DESELECT_1: &[u8] = include_bytes!("../../assets/toggle_deselect_1.png");
const TOGGLE_DESELECT_2: &[u8] = include_bytes!("../../assets/toggle_deselect_2.png");
const TOGGLE_DESELECT_3: &[u8] = include_bytes!("../../assets/toggle_deselect_3.png");
const TOGGLE_DESELECT_4: &[u8] = include_bytes!("../../assets/toggle_deselect_4.png");
const TOGGLE_DESELECT_5: &[u8] = include_bytes!("../../assets/toggle_deselect_5.png");
const TOGGLE_DESELECTED: &[u8] = include_bytes!("../../assets/toggle_deselected.png");
const TOP_LABEL: &[u8] = include_bytes!("../../assets/top_label.png");
const WALL: &[u8] = include_bytes!("../../assets/wall.png");
const WALL_IPAD: &[u8] = include_bytes!("../../assets/wall_ipad.png");
const WHEEL: &[u8] = include_bytes!("../../assets/wheel.png");

const FORTUNE: &[u8] = include_bytes!("../../assets/fortune.txt");

#[derive(Clone, Default, Resource)]
pub struct Fonts {
    pub bold: Handle<Font>,
    pub medium: Handle<Font>,
    pub light: Handle<Font>,

    pub scroll: Handle<Font>,
}

impl Fonts {
    pub fn load(mut fonts: ResMut<Assets<Font>>) -> Self {
        Self {
            bold: fonts.add(Font::try_from_bytes(Vec::from(BOLD_FONT)).unwrap()),
            medium: fonts.add(Font::try_from_bytes(Vec::from(MEDIUM_FONT)).unwrap()),
            light: fonts.add(Font::try_from_bytes(Vec::from(LIGHT_FONT)).unwrap()),
            scroll: fonts.add(Font::try_from_bytes(Vec::from(XIAOWEI_REGULAR_FONT)).unwrap()),
        }
    }
}

#[derive(Clone, Default, Resource)]
pub struct Fortune {
    pub lines: Vec<&'static str>,
}

impl Fortune {
    pub fn load() -> Self {
        Self {
            lines: FORTUNE
                .split(|&c| c == b'\n')
                .map(|slice| std::str::from_utf8(slice).unwrap())
                .filter(|string| !string.is_empty())
                .collect(),
        }
    }
}

#[derive(Clone, Default, Resource)]
pub struct Images {
    pub cog: Handle<Image>,
    pub cog_pressed: Handle<Image>,
    pub launch_screen: Handle<Image>,
    pub launch_screen_ipad: Handle<Image>,
    pub mode_slider: Handle<Image>,
    pub slice_active_1: Handle<Image>,
    pub slice_active_2: Handle<Image>,
    pub slice_active_3: Handle<Image>,
    pub slice_active_4: Handle<Image>,
    pub slice_active_5: Handle<Image>,
    pub slice_active_6: Handle<Image>,
    pub slice_active_7: Handle<Image>,
    pub slice_active_8: Handle<Image>,
    pub slice_active_9: Handle<Image>,
    pub slice_disabled_1: Handle<Image>,
    pub slice_disabled_2: Handle<Image>,
    pub slice_disabled_3: Handle<Image>,
    pub slice_disabled_4: Handle<Image>,
    pub slice_disabled_5: Handle<Image>,
    pub slice_disabled_6: Handle<Image>,
    pub slice_disabled_7: Handle<Image>,
    pub slice_disabled_8: Handle<Image>,
    pub slice_disabled_9: Handle<Image>,
    pub toggle_select_1: Handle<Image>,
    pub toggle_select_2: Handle<Image>,
    pub toggle_select_3: Handle<Image>,
    pub toggle_select_4: Handle<Image>,
    pub toggle_select_5: Handle<Image>,
    pub toggle_selected: Handle<Image>,
    pub toggle_deselect_1: Handle<Image>,
    pub toggle_deselect_2: Handle<Image>,
    pub toggle_deselect_3: Handle<Image>,
    pub toggle_deselect_4: Handle<Image>,
    pub toggle_deselect_5: Handle<Image>,
    pub toggle_deselected: Handle<Image>,
    pub top_label: Handle<Image>,
    pub wall: Handle<Image>,
    pub wall_ipad: Handle<Image>,
    pub wheel: Handle<Image>,
}

impl Images {
    pub fn load(mut images: ResMut<Assets<Image>>) -> Self {
        Self {
            cog: images.add(load_png(COG)),
            cog_pressed: images.add(load_png(COG_PRESSED)),
            launch_screen: images.add(load_png(LAUNCH_SCREEN)),
            launch_screen_ipad: images.add(load_png(LAUNCH_SCREEN_IPAD)),
            mode_slider: images.add(load_png(MODE_SLIDER)),
            slice_active_1: images.add(load_png(SLICE_1)),
            slice_active_2: images.add(load_png(SLICE_2)),
            slice_active_3: images.add(load_png(SLICE_3)),
            slice_active_4: images.add(load_png(SLICE_4)),
            slice_active_5: images.add(load_png(SLICE_5)),
            slice_active_6: images.add(load_png(SLICE_6)),
            slice_active_7: images.add(load_png(SLICE_7)),
            slice_active_8: images.add(load_png(SLICE_8)),
            slice_active_9: images.add(load_png(SLICE_9)),
            slice_disabled_1: images.add(load_png(SLICE_DISABLED_1)),
            slice_disabled_2: images.add(load_png(SLICE_DISABLED_2)),
            slice_disabled_3: images.add(load_png(SLICE_DISABLED_3)),
            slice_disabled_4: images.add(load_png(SLICE_DISABLED_4)),
            slice_disabled_5: images.add(load_png(SLICE_DISABLED_5)),
            slice_disabled_6: images.add(load_png(SLICE_DISABLED_6)),
            slice_disabled_7: images.add(load_png(SLICE_DISABLED_7)),
            slice_disabled_8: images.add(load_png(SLICE_DISABLED_8)),
            slice_disabled_9: images.add(load_png(SLICE_DISABLED_9)),
            toggle_select_1: images.add(load_png(TOGGLE_SELECT_1)),
            toggle_select_2: images.add(load_png(TOGGLE_SELECT_2)),
            toggle_select_3: images.add(load_png(TOGGLE_SELECT_3)),
            toggle_select_4: images.add(load_png(TOGGLE_SELECT_4)),
            toggle_select_5: images.add(load_png(TOGGLE_SELECT_5)),
            toggle_selected: images.add(load_png(TOGGLE_SELECTED)),
            toggle_deselect_1: images.add(load_png(TOGGLE_DESELECT_1)),
            toggle_deselect_2: images.add(load_png(TOGGLE_DESELECT_2)),
            toggle_deselect_3: images.add(load_png(TOGGLE_DESELECT_3)),
            toggle_deselect_4: images.add(load_png(TOGGLE_DESELECT_4)),
            toggle_deselect_5: images.add(load_png(TOGGLE_DESELECT_5)),
            toggle_deselected: images.add(load_png(TOGGLE_DESELECTED)),
            top_label: images.add(load_png(TOP_LABEL)),
            wall: images.add(load_png(WALL)),
            wall_ipad: images.add(load_png(WALL_IPAD)),
            wheel: images.add(load_png(WHEEL)),
        }
    }
}

fn load_png(bytes: &[u8]) -> Image {
    Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::all(),
        true,
        ImageSampler::Default,
    )
    .unwrap()
}
