use auto_struct_macros::auto_struct;

use reader_writer::CStr;
use reader_writer::typenum::*;
use reader_writer::generic_array::GenericArray;
use crate::SclyPropertyData;

#[auto_struct(Readable, Writable)]
#[derive(Debug, Clone)]
pub struct Generator<'r>
{
    #[auto_struct(expect = 8)]
    prop_count: u32,

    pub name: CStr<'r>,
    pub spawn_count: u32,
    pub no_reuse_followers: u8,
    pub no_inherit_xf: u8,
    pub offset: GenericArray<f32, U3>,
    pub active: u8,
    pub min_scale: f32,
    pub max_scale: f32,
}

impl<'r> SclyPropertyData for Generator<'r>
{
    const OBJECT_TYPE: u8 = 0x0A;
}
