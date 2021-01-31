use crate::{
    deserialization::{
        deserialize_as_degrees, deserialize_as_milliseconds, deserialize_map_as_vec,
        deserialize_multipliers,
    },
    error::PyxelError,
};

use derivative::Derivative;
use semver::Version;
use serde::Deserialize;
use std::{collections::BTreeMap, time::Duration};

/// An RGBA color
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
    /// The red component of this color.
    pub r: u8,
    /// The green component of this color.
    pub g: u8,
    /// The blue component of this color.
    pub b: u8,
    /// The alpha component of this color.
    pub a: u8,
}

impl std::str::FromStr for Color {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use hex::FromHex;

        let decoded = <[u8; 4]>::from_hex(s)?;

        Ok(Color {
            r: decoded[1],
            g: decoded[2],
            b: decoded[3],
            a: decoded[0],
        })
    }
}

impl<'de> serde::de::Deserialize<'de> for Color {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Color, D::Error> {
        deserializer.deserialize_str(ColorVisitor)
    }
}

struct ColorVisitor;

impl<'de> serde::de::Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a color in the format AARRGGBB")
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        use std::str::FromStr;
        Self::Value::from_str(value).map_err(serde::de::Error::custom)
    }
}

/// A Pyxel palette.
#[derive(Debug, Deserialize)]
pub struct Palette {
    #[serde(deserialize_with = "deserialize_map_as_vec")]
    colors: Vec<Option<Color>>,

    height: u8,

    #[serde(rename = "numColors")]
    num_colors: usize,

    width: u8,
}

impl Palette {
    /// Returns the colors that make up this palette.
    pub fn colors(&self) -> &Vec<Option<Color>> {
        &self.colors
    }

    /// Returns the height of this palette when displayed in the PyxelEdit UI.
    pub fn height(&self) -> u8 {
        self.height
    }

    /// Returns the width of this palette when displayed in the PyxelEdit UI.
    pub fn width(&self) -> u8 {
        self.width
    }
}

/// A reference to a tile in a Pyxel tileset.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct TileRef {
    index: usize,
    #[serde(deserialize_with = "deserialize_as_degrees")]
    rot: f64,

    #[serde(rename = "flipX")]
    flip_x: bool,
}

impl TileRef {
    /// Returns the index of the tile in the tileset.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the rotation of this tile in degrees.
    pub fn rot(&self) -> f64 {
        self.rot
    }

    /// Returns `true` if the tile is flipped horizontally.
    pub fn flip_x(&self) -> bool {
        self.flip_x
    }
}

/// A Pyxel blend mode.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum BlendMode {
    /// Normal blend mode
    #[serde(rename = "normal")]
    Normal,

    /// Multiply blend mode
    #[serde(rename = "multiply")]
    Multiply,

    /// Add blend mode
    #[serde(rename = "add")]
    Add,

    /// Difference blend mode
    #[serde(rename = "difference")]
    Difference,

    /// Darken blend mode
    #[serde(rename = "darken")]
    Darken,

    /// Lighten blend mode
    #[serde(rename = "lighten")]
    Lighten,

    /// Hard light blend mode
    #[serde(rename = "hardlight")]
    Hardlight,

    /// Invert blend mode
    #[serde(rename = "invert")]
    Invert,

    /// Overlay blend mode
    #[serde(rename = "overlay")]
    Overlay,

    /// Screen blend mode
    #[serde(rename = "screen")]
    Screen,

    /// Subtract blend mode
    #[serde(rename = "subtract")]
    Subtract,
}

#[cfg(feature = "images")]
fn default_image() -> image::DynamicImage {
    image::DynamicImage::new_rgba8(1, 1)
}

/// A Pyxel canvas layer.
#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
pub struct Layer {
    alpha: u8,

    #[serde(rename = "blendMode")]
    blend_mode: BlendMode,

    hidden: bool,
    muted: bool,
    name: String,
    soloed: bool,

    #[serde(rename = "tileRefs")]
    tile_refs: BTreeMap<usize, TileRef>,

    #[cfg(not(feature = "images"))]
    #[serde(skip)]
    image_data: Vec<u8>,

    #[cfg(feature = "images")]
    #[derivative(Debug = "ignore")]
    #[serde(default = "default_image", skip)]
    image: image::DynamicImage,
}

impl Layer {
    /// Returns the alpha value of this layer.
    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    /// Returns the blend mode for this layer.
    pub fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }

    /// Returns `true` if this layer is hidden in the PyxelEdit UI.
    pub fn hidden(&self) -> bool {
        self.hidden
    }

    /// Returns `true` if this layer is muted in the PyxelEdit UI.
    pub fn muted(&self) -> bool {
        self.muted
    }

    /// Returns the name of this layer.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns `true` if this layer is soloed in the PyxelEdit UI.
    pub fn soloed(&self) -> bool {
        self.soloed
    }

    /// Returns the tilerefs for this layer.
    pub fn tile_refs(&self) -> &BTreeMap<usize, TileRef> {
        &self.tile_refs
    }

    /// Returns the raw bytes of the image for this layer.
    #[cfg(not(feature = "images"))]
    pub fn image_data(&self) -> &Vec<u8> {
        &self.image_data
    }

    /// Returns the image for this layer.
    #[cfg(feature = "images")]
    pub fn image(&self) -> &image::DynamicImage {
        &self.image
    }
}

/// A Pyxel canvas.
#[derive(Debug, Deserialize)]
pub struct Canvas {
    #[serde(deserialize_with = "deserialize_map_as_vec")]
    layers: Vec<Layer>,
    height: i32,

    #[serde(rename = "numLayers")]
    num_layers: usize,

    #[serde(rename = "tileHeight")]
    tile_height: u16,

    #[serde(rename = "tileWidth")]
    tile_width: u16,

    width: i32,
}

impl Canvas {
    /// Returns the layers of this canvas.
    pub fn layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    /// Returns the height of this canvas in pixels.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Returns the height of the tiles in this canvas in pixels.
    pub fn tile_height(&self) -> u16 {
        self.tile_height
    }

    /// Returns the width of the tiles in this canvas in pixels.
    pub fn tile_width(&self) -> u16 {
        self.tile_width
    }

    /// Returns the width of this canvas in pixels.
    pub fn width(&self) -> i32 {
        self.width
    }
}

/// A Pyxel tileset.
#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
pub struct Tileset {
    #[serde(rename = "fixedWidth")]
    fixed_width: bool,

    #[serde(rename = "numTiles")]
    num_tiles: usize,

    #[serde(rename = "tileHeight")]
    tile_height: u16,

    #[serde(rename = "tileWidth")]
    tile_width: u16,

    #[serde(rename = "tilesWide")]
    tiles_wide: u8,

    #[cfg(not(feature = "images"))]
    #[serde(skip)]
    image_data: Vec<Vec<u8>>,

    #[cfg(feature = "images")]
    #[derivative(Debug = "ignore")]
    #[serde(skip)]
    images: Vec<image::DynamicImage>,
}

impl Tileset {
    /// Returns `true` if this tileset is fixed width when displayed in the PyxelEdit UI.
    pub fn fixed_width(&self) -> bool {
        self.fixed_width
    }

    /// Returns the tile height in pixels of the tiles in this tileset.
    pub fn tile_height(&self) -> u16 {
        self.tile_height
    }

    /// Returns the tile width in pixels of the tiles in this tileset.
    pub fn tile_width(&self) -> u16 {
        self.tile_width
    }

    /// Returns the width of this tileset when displayed in the PyxelEdit UI.
    pub fn tiles_wide(&self) -> u8 {
        self.tiles_wide
    }

    /// Returns raw bytes of the images for the tiles in this tileset.
    #[cfg(not(feature = "images"))]
    pub fn image_data(&self) -> &Vec<Vec<u8>> {
        &self.image_data
    }

    /// Returns the images for the tiles in this tileset.
    #[cfg(feature = "images")]
    pub fn images(&self) -> &Vec<image::DynamicImage> {
        &self.images
    }
}

/// A Pyxel animation.
#[derive(Debug, Deserialize)]
pub struct Animation {
    #[serde(rename = "baseTile")]
    base_tile: usize,

    #[serde(
        deserialize_with = "deserialize_as_milliseconds",
        rename = "frameDuration"
    )]
    frame_duration: Duration,

    #[serde(
        deserialize_with = "deserialize_multipliers",
        rename = "frameDurationMultipliers"
    )]
    frame_duration_multipliers: Vec<f64>,

    length: usize,
    name: String,
}

impl Animation {
    /// Returns the canvas tile this animation starts on.
    pub fn base_tile(&self) -> usize {
        self.base_tile
    }

    /// Returns the base frame duration for this animation.
    pub fn frame_duration(&self) -> Duration {
        self.frame_duration
    }

    /// Returns the frame duration multipliers for this animation.
    pub fn frame_duration_multipliers(&self) -> &Vec<f64> {
        &self.frame_duration_multipliers
    }

    /// Returns the number of frames in this animation.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns the name of this animation.
    pub fn name(&self) -> &String {
        &self.name
    }
}

/// A Pyxel document.
#[derive(Debug, Deserialize)]
pub struct Pyxel {
    #[serde(deserialize_with = "deserialize_map_as_vec")]
    animations: Vec<Animation>,
    canvas: Canvas,
    name: String,
    palette: Palette,
    tileset: Tileset,
    version: Version,
}

impl Pyxel {
    /// Returns the animations for this document.
    pub fn animations(&self) -> &Vec<Animation> {
        &self.animations
    }

    /// Returns the canvas for this document.
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    /// Returns the name of this document.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the palette for this document.
    pub fn palette(&self) -> &Palette {
        &self.palette
    }

    /// Returns the tileset for this document.
    pub fn tileset(&self) -> &Tileset {
        &self.tileset
    }

    /// Returns the version of PyxelEdit this document was created with.
    pub fn version(&self) -> &Version {
        &self.version
    }
}

#[cfg(not(feature = "images"))]
fn load_image_data_from_zip<R: std::io::Read + std::io::Seek>(
    zip: &mut zip::ZipArchive<R>,
    path: &str,
) -> Result<Vec<u8>, PyxelError> {
    use std::io::Read;

    let mut file = zip.by_name(path)?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    Ok(buf)
}

#[cfg(feature = "images")]
fn load_image_from_zip<R: std::io::Read + std::io::Seek>(
    zip: &mut zip::ZipArchive<R>,
    path: &str,
) -> Result<image::DynamicImage, PyxelError> {
    use std::io::Read;

    let mut file = zip.by_name(path)?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let image = image::load_from_memory_with_format(&buf, image::ImageFormat::PNG)?;
    Ok(image)
}

/// Load a Pyxel document from a reader.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// # fn main() -> Result<(), pyxel::PyxelError> {
/// let file = File::open("resources/doc.pyxel")?;
/// let doc = pyxel::load(file)?;
/// # Ok(())
/// # }
/// ```
pub fn load<R: std::io::Read + std::io::Seek>(r: R) -> Result<Pyxel, PyxelError> {
    let mut archive = zip::ZipArchive::new(r)?;
    let data = archive.by_name("docData.json")?;

    let mut pyxel: Pyxel = serde_json::from_reader(data)?;

    for i in 0..pyxel.canvas().num_layers {
        #[cfg(not(feature = "images"))]
        {
            let image_data = load_image_data_from_zip(&mut archive, &format!("layer{}.png", i))?;
            pyxel.canvas.layers[i].image_data = image_data;
        }
        #[cfg(feature = "images")]
        {
            let image = load_image_from_zip(&mut archive, &format!("layer{}.png", i))?;
            pyxel.canvas.layers[i].image = image;
        }
    }

    for i in 0..pyxel.tileset().num_tiles {
        #[cfg(not(feature = "images"))]
        {
            let image_data = load_image_data_from_zip(&mut archive, &format!("tile{}.png", i))?;
            pyxel.tileset.image_data.insert(i, image_data);
        }
        #[cfg(feature = "images")]
        {
            let image = load_image_from_zip(&mut archive, &format!("tile{}.png", i))?;
            pyxel.tileset.images.insert(i, image);
        }
    }

    Ok(pyxel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::BTreeMap, fs::File, str::FromStr};

    #[test]
    fn convert_color_from_aarrggbb() {
        let c = Color::from_str("ffaabbcc").unwrap();
        assert_eq!(170, c.r);
        assert_eq!(187, c.g);
        assert_eq!(204, c.b);
        assert_eq!(255, c.a);
    }

    const TEST_FILE: &str = "resources/test_v0.4.8.pyxel";

    #[test]
    fn load_palette_colors() {
        let file = File::open(TEST_FILE).unwrap();
        let doc = load(file).unwrap();

        assert_eq!(
            &vec![
                Some(Color {
                    r: 190,
                    g: 53,
                    b: 53,
                    a: 255
                }),
                Some(Color {
                    r: 249,
                    g: 155,
                    b: 151,
                    a: 255
                }),
                Some(Color {
                    r: 145,
                    g: 95,
                    b: 51,
                    a: 255
                }),
                Some(Color {
                    r: 209,
                    g: 127,
                    b: 48,
                    a: 255
                }),
                Some(Color {
                    r: 247,
                    g: 238,
                    b: 89,
                    a: 255
                }),
                Some(Color {
                    r: 89,
                    g: 205,
                    b: 54,
                    a: 255
                }),
                Some(Color {
                    r: 131,
                    g: 240,
                    b: 220,
                    a: 255
                }),
                Some(Color {
                    r: 117,
                    g: 161,
                    b: 236,
                    a: 255
                }),
                Some(Color {
                    r: 65,
                    g: 55,
                    b: 205,
                    a: 255
                }),
                Some(Color {
                    r: 204,
                    g: 89,
                    b: 198,
                    a: 255
                }),
                Some(Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255
                }),
                Some(Color {
                    r: 202,
                    g: 202,
                    b: 202,
                    a: 255
                }),
                Some(Color {
                    r: 142,
                    g: 142,
                    b: 142,
                    a: 255
                }),
                Some(Color {
                    r: 91,
                    g: 91,
                    b: 91,
                    a: 255
                }),
                Some(Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255
                })
            ],
            doc.palette().colors()
        );
    }

    #[test]
    fn load_canvas_layer_tilerefs() {
        let file = File::open(TEST_FILE).unwrap();
        let doc = load(file).unwrap();

        let mut tile_refs = BTreeMap::new();
        tile_refs.insert(
            56,
            TileRef {
                index: 0,
                rot: 0.0,
                flip_x: false,
            },
        );
        tile_refs.insert(
            57,
            TileRef {
                index: 0,
                rot: 90.0,
                flip_x: false,
            },
        );
        tile_refs.insert(
            58,
            TileRef {
                index: 0,
                rot: 180.0,
                flip_x: false,
            },
        );
        tile_refs.insert(
            59,
            TileRef {
                index: 0,
                rot: 270.0,
                flip_x: false,
            },
        );
        tile_refs.insert(
            60,
            TileRef {
                index: 0,
                rot: 0.0,
                flip_x: true,
            },
        );
        tile_refs.insert(
            61,
            TileRef {
                index: 0,
                rot: 90.0,
                flip_x: true,
            },
        );
        tile_refs.insert(
            62,
            TileRef {
                index: 0,
                rot: 180.0,
                flip_x: true,
            },
        );
        tile_refs.insert(
            63,
            TileRef {
                index: 0,
                rot: 270.0,
                flip_x: true,
            },
        );

        assert_eq!(&tile_refs, doc.canvas().layers()[1].tile_refs());
    }
}
