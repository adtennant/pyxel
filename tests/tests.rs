use semver::Version;
use std::{
    fs::{read, File},
    time::Duration,
};

use pyxel::*;

const TEST_FILE_V0_4_8: &str = "resources/test_v0.4.8.pyxel";

fn check_v0_4_8(doc: Pyxel) {
    fn check_animation(
        animation: &Animation,
        base_tile: usize,
        frame_duration: Duration,
        frame_duration_multipliers: &[f64],
        length: usize,
        name: &str,
    ) {
        assert_eq!(base_tile, animation.base_tile());
        assert_eq!(frame_duration, animation.frame_duration());
        assert_eq!(
            frame_duration_multipliers,
            animation.frame_duration_multipliers().as_slice()
        );
        assert_eq!(length, animation.length());
        assert_eq!(name, animation.name());
    }

    fn check_layer(
        layer: &Layer,
        alpha: u8,
        blend_mode: BlendMode,
        hidden: bool,
        muted: bool,
        name: &str,
        soloed: bool,
        num_tile_refs: usize,
    ) {
        assert_eq!(alpha, layer.alpha());
        assert_eq!(blend_mode, layer.blend_mode());
        assert_eq!(hidden, layer.hidden());
        assert_eq!(muted, layer.muted());
        assert_eq!(name, layer.name());
        assert_eq!(soloed, layer.soloed());
        assert_eq!(num_tile_refs, layer.tile_refs().len());
    }

    // animations
    assert_eq!(3, doc.animations().len());

    // 0
    check_animation(
        &doc.animations()[0],
        0,
        Duration::from_millis(150),
        &[1., 2., 3., 4.],
        4,
        "Animation 1",
    );

    // 1
    check_animation(
        &doc.animations()[1],
        4,
        Duration::from_millis(100),
        &[1., 1.],
        2,
        "Animation 2",
    );

    // 2
    check_animation(
        &doc.animations()[2],
        6,
        Duration::from_millis(1000),
        &[1., 1.],
        2,
        "Animation 3",
    );

    // canvas

    // canvas.layers
    assert_eq!(11, doc.canvas().layers().len());

    // 0
    check_layer(
        &doc.canvas().layers()[0],
        255,
        BlendMode::Subtract,
        false,
        false,
        "Layer 10",
        false,
        4,
    );

    // 1
    check_layer(
        &doc.canvas().layers()[1],
        255,
        BlendMode::Screen,
        false,
        false,
        "Layer 9",
        true,
        8,
    );

    // 2
    check_layer(
        &doc.canvas().layers()[2],
        255,
        BlendMode::Overlay,
        false,
        true,
        "Layer 8",
        false,
        0,
    );

    // 3
    check_layer(
        &doc.canvas().layers()[3],
        255,
        BlendMode::Invert,
        true,
        false,
        "Layer 7",
        false,
        0,
    );

    // 4
    check_layer(
        &doc.canvas().layers()[4],
        255,
        BlendMode::Hardlight,
        false,
        false,
        "Layer 6",
        false,
        0,
    );

    // 5
    check_layer(
        &doc.canvas().layers()[5],
        255,
        BlendMode::Lighten,
        false,
        false,
        "Layer 5",
        false,
        0,
    );

    // 6
    check_layer(
        &doc.canvas().layers()[6],
        255,
        BlendMode::Darken,
        false,
        false,
        "Layer 4",
        false,
        0,
    );

    // 7
    check_layer(
        &doc.canvas().layers()[7],
        255,
        BlendMode::Difference,
        false,
        false,
        "Layer 3",
        false,
        0,
    );

    // 8
    check_layer(
        &doc.canvas().layers()[8],
        255,
        BlendMode::Add,
        false,
        false,
        "Layer 2",
        false,
        0,
    );

    // 9
    check_layer(
        &doc.canvas().layers()[9],
        255,
        BlendMode::Multiply,
        false,
        false,
        "Layer 1",
        false,
        0,
    );

    // 10
    check_layer(
        &doc.canvas().layers()[10],
        255,
        BlendMode::Normal,
        false,
        false,
        "Layer 0",
        false,
        0,
    );

    assert_eq!(128, doc.canvas().height());
    assert_eq!(16, doc.canvas().tile_height());
    assert_eq!(32, doc.canvas().tile_width());
    assert_eq!(256, doc.canvas().width());

    // name
    assert_eq!("test_v0.4.8", doc.name());

    // palette
    assert_eq!(15, doc.palette().colors().len());
    assert_eq!(4, doc.palette().height());
    assert_eq!(8, doc.palette().width());

    // tileset
    assert_eq!(false, doc.tileset().fixed_width());
    assert_eq!(16, doc.tileset().tile_height());
    assert_eq!(32, doc.tileset().tile_width());
    assert_eq!(8, doc.tileset().tiles_wide());
    assert_eq!(4, doc.tileset().images().len());

    // version
    assert_eq!(Version::parse("0.4.8").unwrap(), *doc.version());
}

#[test]
fn open_v0_4_8() {
    let doc = pyxel::open(TEST_FILE_V0_4_8).unwrap();
    check_v0_4_8(doc);
}

#[test]
fn load_v0_4_8() {
    let file = File::open(TEST_FILE_V0_4_8).unwrap();
    let doc = pyxel::load(file).unwrap();
    check_v0_4_8(doc);
}

#[test]
fn load_from_memory_v0_4_8() {
    let buf = read(TEST_FILE_V0_4_8).unwrap();
    let doc = pyxel::load_from_memory(&buf).unwrap();
    check_v0_4_8(doc);
}
