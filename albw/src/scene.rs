use std::{fmt, path::Path};

use serde::{de, ser::SerializeTuple, Deserialize, Deserializer, Serialize, Serializer};

use crate::{actors::Actors, files::sarc::Sarc, File, Result};

#[derive(Debug)]
pub struct Scene {
    actors: Actors,
    stage: File<Stage>,
}

impl Scene {
    pub(crate) fn new(stage: File<Stage>, actors: File<Sarc>) -> Self {
        Self {
            actors: Actors::new(actors),
            stage,
        }
    }

    pub fn actors(&self) -> &Actors {
        &self.actors
    }

    pub fn actors_mut(&mut self) -> &mut Actors {
        &mut self.actors
    }

    pub fn stage(&self) -> &File<Stage> {
        &self.stage
    }

    pub fn stage_mut(&mut self) -> &mut File<Stage> {
        &mut self.stage
    }

    pub fn into_files(self) -> (Option<File<Sarc>>, File<Stage>) {
        (self.actors.into_archive(), self.stage)
    }

    pub fn dump<P>(self, path: P) -> Result<()>
        where
            P: AsRef<Path>,
    {
        let path = path.as_ref();
        self.actors.dump(path)?;
        self.stage.serialize().dump(path)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct Stage {
    rails: Vec<Rail>,
    system: Vec<Obj>,
    objs: Vec<Obj>,
}

impl Stage {
    pub fn add_obj(&mut self, obj: Obj) {
        self.objs.push(obj);
    }

    pub fn get_obj_mut(&mut self, unq: u16) -> Option<&mut Obj> {
        if let Some(i) = self.objs.iter().position(|obj| obj.unq == unq) {
            self.objs.get_mut(i)
        } else {
            None
        }
    }

    pub fn get_system_mut(&mut self, unq: u16) -> Option<&mut Obj> {
        if let Some(i) = self.system.iter().position(|sys| sys.unq == unq) {
            self.system.get_mut(i)
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "UPPERCASE")]
pub struct Obj {
    pub arg: Arg,
    pub clp: i16,
    pub flg: Flg,
    pub id: i16,
    pub lnk: Vec<Lnk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nme: Option<String>,
    pub ril: Vec<(i32, i32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ser: Option<u16>,
    pub srt: Transform,
    pub typ: i32,
    pub unq: u16,
}

impl Obj {

    /// Generate a new Warp object
    pub fn warp(activation_flag: u16, clp: i16, ser: u16, translate: Vec3, unq: u16, spawn: i32, scene: i32, scene_index: i32) -> Self {
        Self {
            arg: Arg {
                0: spawn,
                1: 0,
                2: 0,
                3: 0,
                4: 4,
                5: 0,
                6: activation_flag,
                7: 0,
                8: 0,
                9: 0,
                10: scene,
                11: scene_index,
                12: 0,
                13: 0.0
            },
            clp,
            flg: (0, 0, 0, 0),
            id: 469,
            lnk: vec![],
            nme: None,
            ril: vec![],
            ser: Some(ser),
            srt: Transform {
                scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
                rotate: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                translate,
            },
            typ: 6,
            unq,
        }
    }


    /// Generate a new Obj to act as a Dungeon Reward trigger
    pub fn dungeon_reward(reward_flag: u16, clp: i16, ser: u16, translate: Vec3, typ: i32, unq: u16) -> Self {
        Self {
            arg: Arg(0, 0, 0, 0, 4, 0, reward_flag, 0, 0, 0, 0, 0, 0, 0.0),
            clp,
            flg: (0, 0, 0, 0),
            id: 16, // AreaSwitchCylinder
            lnk: vec![],
            nme: None,
            ril: vec![],
            ser: Some(ser),
            srt: Transform {
                scale: Vec3 { x: 1.0, y: 2.0, z: 1.0 },
                rotate: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                translate,
            },
            typ,
            unq,
        }
    }

    pub fn arg_mut(&mut self) -> &mut Arg {
        &mut self.arg
    }

    pub fn lnk_mut(&mut self) -> &mut Vec<Lnk> {
        &mut self.lnk
    }

    pub fn set_clp(&mut self, clp: i16) {
        self.clp = clp;
    }

    pub fn srt_mut(&mut self) -> &mut Transform {
        &mut self.srt
    }

    pub fn set_active_flag<F>(&mut self, flag: F)
        where
            F: Into<Option<Flag>>,
    {
        let (kind, flag) = match flag.into() {
            Some(flag) => flag.into_pair(),
            None => (0, 0),
        };
        self.arg.4 = kind;
        self.arg.6 = flag;
    }

    pub fn set_inactive_flag<F>(&mut self, flag: F)
        where
            F: Into<Option<Flag>>,
    {
        let (kind, flag) = match flag.into() {
            Some(flag) => flag.into_pair(),
            None => (0, 0),
        };
        self.arg.5 = kind;
        self.arg.7 = flag;
    }

    pub fn flg_mut(&mut self) -> &mut Flg {
        &mut self.flg
    }

    pub fn set_enable_flag<F>(&mut self, flag: F)
        where
            F: Into<Option<Flag>>,
    {
        let (kind, flag) = match flag.into() {
            Some(flag) => flag.into_pair(),
            None => (0, 0),
        };
        self.flg.0 = kind;
        self.flg.2 = flag;
    }

    pub fn set_disable_flag<F>(&mut self, flag: F)
        where
            F: Into<Option<Flag>>,
    {
        let (kind, flag) = match flag.into() {
            Some(flag) => flag.into_pair(),
            None => (0, 0),
        };
        self.flg.1 = kind;
        self.flg.3 = flag;
    }

    pub fn enable(&mut self) {
        self.flg.0 = 0;
        self.flg.1 = 0;
        self.flg.2 = 0;
        self.flg.3 = 0;
    }

    pub fn disable(&mut self) {
        self.flg.1 = 4;
        self.flg.3 = 1;
    }

    pub fn clear_enable_flag(&mut self) {
        self.flg.0 = 0;
        self.flg.2 = 0;
    }

    pub fn clear_disable_flag(&mut self) {
        self.flg.1 = 0;
        self.flg.3 = 0;
    }

    pub fn clear_active_args(&mut self) {
        self.arg.4 = 0;
        self.arg.6 = 0;
    }

    pub fn clear_inactive_args(&mut self) {
        self.arg.5 = 0;
        self.arg.7 = 0;
    }

    pub fn set_id(&mut self, id: i16) {
        self.id = id;
    }

    pub fn set_typ(&mut self, typ: i32) {
        self.typ = typ;
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.srt.scale.x = x;
        self.srt.scale.y = y;
        self.srt.scale.z = z;
    }

    pub fn add_to_translate(&mut self, x: f32, y: f32, z: f32) {
        self.srt.translate.x += x;
        self.srt.translate.y += y;
        self.srt.translate.z += z;
    }

    pub fn set_translate(&mut self, x: f32, y: f32, z: f32) {
        self.srt.translate.x = x;
        self.srt.translate.y = y;
        self.srt.translate.z = z;
    }

    pub fn redirect(&mut self, spawn_point: i32, scene: i32, scene_index: i32) {
        self.arg.0 = spawn_point;
        self.arg.10 = scene;
        self.arg.11 = scene_index;
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arg(
    pub i32, //i16,
    pub i32, //i16,
    pub i32, //i16,
    pub i32, //i16,
    pub u8,
    pub u8,
    pub u16,
    pub u16,
    pub i32, //i8,
    pub i32, //i8,
    pub i32, //i16,
    pub i32, //i16,
    pub i32, //i16,
    pub f32,
);

pub type Flg = (u8, u8, u16, u16);

pub type Lnk = (u16, i16, i16);

#[derive(Debug)]
pub struct Transform {
    pub scale: Vec3,
    pub rotate: Vec3,
    pub translate: Vec3,
}

impl<'de> Deserialize<'de> for Transform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct TransformVisitor;

        impl<'de> de::Visitor<'de> for TransformVisitor {
            type Value = Transform;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a nine-element tuple")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: de::SeqAccess<'de>,
            {
                Ok(Transform {
                    scale: Vec3 {
                        x: seq
                            .next_element()?
                            .ok_or_else(|| de::Error::missing_field("S.x"))?,
                        y: seq
                            .next_element()?
                            .ok_or_else(|| de::Error::missing_field("S.y"))?,
                        z: seq
                            .next_element()?
                            .ok_or_else(|| de::Error::missing_field("S.z"))?,
                    },
                    rotate: Vec3 {
                        x: seq
                            .next_element()?
                            .ok_or_else(|| de::Error::missing_field("R.x"))?,
                        y: seq
                            .next_element()?
                            .ok_or_else(|| de::Error::missing_field("R.y"))?,
                        z: seq
                            .next_element()?
                            .ok_or_else(|| de::Error::missing_field("R.z"))?,
                    },
                    translate: Vec3 {
                        x: seq
                            .next_element()?
                            .ok_or_else(|| de::Error::missing_field("T.x"))?,
                        y: seq
                            .next_element()?
                            .ok_or_else(|| de::Error::missing_field("T.y"))?,
                        z: seq
                            .next_element()?
                            .ok_or_else(|| de::Error::missing_field("T.z"))?,
                    },
                })
            }
        }

        deserializer.deserialize_tuple(9, TransformVisitor)
    }
}

impl Serialize for Transform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = serializer.serialize_tuple(9)?;
        s.serialize_element(&self.scale.x)?;
        s.serialize_element(&self.scale.y)?;
        s.serialize_element(&self.scale.z)?;
        s.serialize_element(&self.rotate.x)?;
        s.serialize_element(&self.rotate.y)?;
        s.serialize_element(&self.rotate.z)?;
        s.serialize_element(&self.translate.x)?;
        s.serialize_element(&self.translate.y)?;
        s.serialize_element(&self.translate.z)?;
        s.end()
    }
}

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "UPPERCASE")]
pub struct Rail {
    arg: RailArg,
    pnt: Vec<Point>,
    rng: bool,
    unq: u16,
}

type RailArg = (i32, i32, i32, i32, f32, f32);

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "UPPERCASE")]
pub struct Point {
    arg: RailArg,
    ctl: [f32; 6],
    lnk: Vec<Lnk>,
    srt: Transform,
}

#[derive(Debug)]
pub enum Flag {
    Course(u16),
    Event(u16),
}

impl Flag {
    fn into_pair(self) -> (u8, u16) {
        match self {
            Flag::Course(flag) => (3, flag),
            Flag::Event(flag) => (4, flag),
        }
    }
}
