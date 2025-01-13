use std::path::PathBuf;
use std::sync::Arc;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Debug)]
pub struct Identifier {
    namespace: String,
    value: String,
}

impl Identifier {
    pub fn render(&self) -> String {
        format!("{}:{}", self.namespace, self.value)
    }

    pub fn new(namespace: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            value: value.into(),
        }
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.render())
    }
}

#[derive(Clone)]
pub struct Pair<T: Clone, K: Clone> {
    pub first: T,
    pub second: K,
}

impl<T: Clone, K: Clone> Pair<T, K> {
    pub fn new(first: T, second: K) -> Self {
        Self { first, second }
    }
}

pub trait Buildable: Clone {
    fn build(&self) -> Arc<Self> {
        Arc::new(self.clone())
    }
}

pub trait Generatable {
    fn generate(&self, path_buf: impl Into<PathBuf>);
}

#[derive(Clone)]
pub struct SemVer {
    major: i32,
    minor: i32,
    patch: i32,
    beta: bool,
}

impl SemVer {
    pub fn new(major: i32, minor: i32, patch: i32) -> Self {
        Self {
            major,
            minor,
            patch,
            beta: false,
        }
    }

    pub fn new_beta(major: i32, minor: i32, patch: i32) -> Self {
        Self {
            major,
            minor,
            patch,
            beta: true,
        }
    }

    pub fn render(&self) -> String {
        format!(
            "{}.{}.{}{}",
            self.major,
            self.minor,
            self.patch,
            if self.beta { "-beta" } else { "" }
        )
    }

    pub fn render_commas(&self) -> String {
        format!(
            "{}, {}, {}",
            self.major,
            self.minor,
            self.patch,
        )
    }

    pub fn current() -> Self {
        Self {
            major: 1,
            minor: 21,
            patch: 40,
            beta: false
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct RangeDescriptor<T> where T: Clone {
    pub min: T,
    pub max: T
}

impl<T> RangeDescriptor<T> where T: Clone {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

pub fn vec_into<T>(vec: Vec<impl Into<T>>) -> Vec<T>
where T: Clone {
    let new_vec = vec.into_iter().map(|e| e.into()).collect();

    new_vec
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[serde(transparent)]
pub struct MolangStatement(String);

impl MolangStatement {
    pub fn new(src: impl Into<String>) -> Self {
        MolangStatement(src.into())
    }
}


#[derive(Clone, Debug)]
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorCode {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    MinecoinGold,
    MaterialQuartz,
    MaterialIron,
    MaterialNetherite,
    MaterialRedstone,
    MaterialCopper,
    MaterialGold,
    MaterialEmerald,
    MaterialDiamond,
    MaterialLapis,
    MaterialAmethyst,
    MaterialResin
}

impl ColorCode {
    /// The color code for use in strings. (in form of `§X`)
    pub fn str_code(&self) -> &str {
        match self {
            ColorCode::Black => "§0",
            ColorCode::DarkBlue => "§1",
            ColorCode::DarkGreen => "§2",
            ColorCode::DarkAqua => "§3",
            ColorCode::DarkRed => "§4",
            ColorCode::DarkPurple => "§5",
            ColorCode::Gold => "§6",
            ColorCode::Gray => "§7",
            ColorCode::DarkGray => "§8",
            ColorCode::Blue => "§9",
            ColorCode::Green => "§a",
            ColorCode::Aqua => "§b",
            ColorCode::Red => "§c",
            ColorCode::LightPurple => "§d",
            ColorCode::Yellow => "§e",
            ColorCode::White => "§f",
            ColorCode::MinecoinGold => "§g",
            ColorCode::MaterialQuartz => "§h",
            ColorCode::MaterialIron => "§i",
            ColorCode::MaterialNetherite => "§j",
            ColorCode::MaterialRedstone => "§m",
            ColorCode::MaterialCopper => "§n",
            ColorCode::MaterialGold => "§p",
            ColorCode::MaterialEmerald => "§q",
            ColorCode::MaterialDiamond => "§s",
            ColorCode::MaterialLapis => "§t",
            ColorCode::MaterialAmethyst => "§u",
            ColorCode::MaterialResin => "§v",
        }
    }
}