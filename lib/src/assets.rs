use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::texture::{CompressedImageFormats, ImageSampler, ImageType};

const BOLD_FONT: &[u8] = include_bytes!("../../assets/Poppins/Poppins-Bold.ttf");
const MEDIUM_FONT: &[u8] = include_bytes!("../../assets/Poppins/Poppins-Medium.ttf");
const LIGHT_FONT: &[u8] = include_bytes!("../../assets/Poppins/Poppins-Light.ttf");

const XIAOWEI_REGULAR_FONT: &[u8] = include_bytes!("../../assets/XiaoWei/ZCOOLXiaoWei-Regular.ttf");

const BOARD_LINE_THIN_CIRCLE: &[u8] = include_bytes!("../../assets/board_line_thin_circle.png");
const COG: &[u8] = include_bytes!("../../assets/cog.png");
const COG_PRESSED: &[u8] = include_bytes!("../../assets/cog_pressed.png");
const LAUNCH_SCREEN: &[u8] = include_bytes!("../../assets/launch_screen.png");
const LAUNCH_SCREEN_IPAD: &[u8] = include_bytes!("../../assets/launch_screen_ipad.png");
const MODE_SLIDER: &[u8] = include_bytes!("../../assets/mode_slider.png");
const POP_DARK_CIRCLE: &[u8] = include_bytes!("../../assets/pop_dark_circle.png");
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

#[derive(Clone, Default)]
pub struct ImageWithDimensions {
    pub handle: Handle<Image>,
    pub width: f32,
    pub height: f32,
}

impl ImageWithDimensions {
    fn load(images: &mut ResMut<Assets<Image>>, bytes: &[u8]) -> Self {
        let image = Image::from_buffer(
            bytes,
            ImageType::Extension("png"),
            CompressedImageFormats::all(),
            true,
            ImageSampler::Default,
            RenderAssetUsages::all(),
        )
        .unwrap();

        let width = image.width() as f32;
        let height = image.height() as f32;
        let handle = images.add(image);

        Self {
            handle,
            width,
            height,
        }
    }
}

#[derive(Clone, Default, Resource)]
pub struct Images {
    pub board_line_thin_circle: ImageWithDimensions,
    pub cog: ImageWithDimensions,
    pub cog_pressed: ImageWithDimensions,
    pub launch_screen: ImageWithDimensions,
    pub launch_screen_ipad: ImageWithDimensions,
    pub mode_slider: ImageWithDimensions,
    pub pop_dark_circle: ImageWithDimensions,
    pub slice_active_1: ImageWithDimensions,
    pub slice_active_2: ImageWithDimensions,
    pub slice_active_3: ImageWithDimensions,
    pub slice_active_4: ImageWithDimensions,
    pub slice_active_5: ImageWithDimensions,
    pub slice_active_6: ImageWithDimensions,
    pub slice_active_7: ImageWithDimensions,
    pub slice_active_8: ImageWithDimensions,
    pub slice_active_9: ImageWithDimensions,
    pub slice_disabled_1: ImageWithDimensions,
    pub slice_disabled_2: ImageWithDimensions,
    pub slice_disabled_3: ImageWithDimensions,
    pub slice_disabled_4: ImageWithDimensions,
    pub slice_disabled_5: ImageWithDimensions,
    pub slice_disabled_6: ImageWithDimensions,
    pub slice_disabled_7: ImageWithDimensions,
    pub slice_disabled_8: ImageWithDimensions,
    pub slice_disabled_9: ImageWithDimensions,
    pub toggle_select_1: ImageWithDimensions,
    pub toggle_select_2: ImageWithDimensions,
    pub toggle_select_3: ImageWithDimensions,
    pub toggle_select_4: ImageWithDimensions,
    pub toggle_select_5: ImageWithDimensions,
    pub toggle_selected: ImageWithDimensions,
    pub toggle_deselect_1: ImageWithDimensions,
    pub toggle_deselect_2: ImageWithDimensions,
    pub toggle_deselect_3: ImageWithDimensions,
    pub toggle_deselect_4: ImageWithDimensions,
    pub toggle_deselect_5: ImageWithDimensions,
    pub toggle_deselected: ImageWithDimensions,
    pub top_label: ImageWithDimensions,
    pub wall: ImageWithDimensions,
    pub wall_ipad: ImageWithDimensions,
    pub wheel: ImageWithDimensions,
}

impl Images {
    pub fn load(mut images: ResMut<Assets<Image>>) -> Self {
        Self {
            board_line_thin_circle: ImageWithDimensions::load(&mut images, BOARD_LINE_THIN_CIRCLE),
            cog: ImageWithDimensions::load(&mut images, COG),
            cog_pressed: ImageWithDimensions::load(&mut images, COG_PRESSED),
            launch_screen: ImageWithDimensions::load(&mut images, LAUNCH_SCREEN),
            launch_screen_ipad: ImageWithDimensions::load(&mut images, LAUNCH_SCREEN_IPAD),
            mode_slider: ImageWithDimensions::load(&mut images, MODE_SLIDER),
            pop_dark_circle: ImageWithDimensions::load(&mut images, POP_DARK_CIRCLE),
            slice_active_1: ImageWithDimensions::load(&mut images, SLICE_1),
            slice_active_2: ImageWithDimensions::load(&mut images, SLICE_2),
            slice_active_3: ImageWithDimensions::load(&mut images, SLICE_3),
            slice_active_4: ImageWithDimensions::load(&mut images, SLICE_4),
            slice_active_5: ImageWithDimensions::load(&mut images, SLICE_5),
            slice_active_6: ImageWithDimensions::load(&mut images, SLICE_6),
            slice_active_7: ImageWithDimensions::load(&mut images, SLICE_7),
            slice_active_8: ImageWithDimensions::load(&mut images, SLICE_8),
            slice_active_9: ImageWithDimensions::load(&mut images, SLICE_9),
            slice_disabled_1: ImageWithDimensions::load(&mut images, SLICE_DISABLED_1),
            slice_disabled_2: ImageWithDimensions::load(&mut images, SLICE_DISABLED_2),
            slice_disabled_3: ImageWithDimensions::load(&mut images, SLICE_DISABLED_3),
            slice_disabled_4: ImageWithDimensions::load(&mut images, SLICE_DISABLED_4),
            slice_disabled_5: ImageWithDimensions::load(&mut images, SLICE_DISABLED_5),
            slice_disabled_6: ImageWithDimensions::load(&mut images, SLICE_DISABLED_6),
            slice_disabled_7: ImageWithDimensions::load(&mut images, SLICE_DISABLED_7),
            slice_disabled_8: ImageWithDimensions::load(&mut images, SLICE_DISABLED_8),
            slice_disabled_9: ImageWithDimensions::load(&mut images, SLICE_DISABLED_9),
            toggle_select_1: ImageWithDimensions::load(&mut images, TOGGLE_SELECT_1),
            toggle_select_2: ImageWithDimensions::load(&mut images, TOGGLE_SELECT_2),
            toggle_select_3: ImageWithDimensions::load(&mut images, TOGGLE_SELECT_3),
            toggle_select_4: ImageWithDimensions::load(&mut images, TOGGLE_SELECT_4),
            toggle_select_5: ImageWithDimensions::load(&mut images, TOGGLE_SELECT_5),
            toggle_selected: ImageWithDimensions::load(&mut images, TOGGLE_SELECTED),
            toggle_deselect_1: ImageWithDimensions::load(&mut images, TOGGLE_DESELECT_1),
            toggle_deselect_2: ImageWithDimensions::load(&mut images, TOGGLE_DESELECT_2),
            toggle_deselect_3: ImageWithDimensions::load(&mut images, TOGGLE_DESELECT_3),
            toggle_deselect_4: ImageWithDimensions::load(&mut images, TOGGLE_DESELECT_4),
            toggle_deselect_5: ImageWithDimensions::load(&mut images, TOGGLE_DESELECT_5),
            toggle_deselected: ImageWithDimensions::load(&mut images, TOGGLE_DESELECTED),
            top_label: ImageWithDimensions::load(&mut images, TOP_LABEL),
            wall: ImageWithDimensions::load(&mut images, WALL),
            wall_ipad: ImageWithDimensions::load(&mut images, WALL_IPAD),
            wheel: ImageWithDimensions::load(&mut images, WHEEL),
        }
    }
}
