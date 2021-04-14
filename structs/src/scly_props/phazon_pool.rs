use auto_struct_macros::auto_struct;

use reader_writer::CStr;
use reader_writer::typenum::*;
use reader_writer::generic_array::GenericArray;
use crate::{ResId, SclyPropertyData};
use crate::res_id:: *;
use crate::scly_props::structs::{DamageInfo};

#[auto_struct(Readable, Writable)]
#[derive(Debug, Clone)]
pub struct PhazonPool<'r>
{
    #[auto_struct(expect = 17)]
    prop_count: u32,

    pub name: CStr<'r>,

    pub position: GenericArray<f32, U3>,
    pub rotation: GenericArray<f32, U3>,
    pub scale: GenericArray<f32, U3>,
    pub unknown1: u8,
    pub cmdl1: ResId<CMDL>,
    pub cmdl2: ResId<CMDL>,
    pub part1: ResId<PART>,
    pub part2: ResId<PART>,
    pub unknown2: u32,
    pub contact_damage: DamageInfo,
    pub force: GenericArray<f32, U3>,
    pub trigger_flags: u32,
    pub pool_starting_value: f32,
    pub phazon_beam_drain_per_sec: f32,
    pub time_until_regen: f32,
    pub auto_drain_and_no_regen: u8,
    pub time_until_auto_drain: f32,
}

impl<'r> SclyPropertyData for PhazonPool<'r>
{
    const OBJECT_TYPE: u8 = 0x87;
}
