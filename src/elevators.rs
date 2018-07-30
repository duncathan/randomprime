#[derive(Clone, Copy, Debug)]
pub struct Elevator {
    pub pak_name: &'static str,
    pub name: &'static str,
    pub mlvl: u32,
    pub mrea: u32,
    pub scly_id: u32,

    pub room_strg: u32,
    pub hologram_strg: u32,
    pub control_strg: u32,

    pub default_dest: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct SpawnRoom
{
    pub pak_name: &'static str,
    pub name: &'static str,
    pub mlvl: u32,
    pub mrea: u32,
}

impl SpawnRoom
{
    pub fn from_room_idx(idx: usize) -> SpawnRoom
    {
        if idx == 20 {
            SpawnRoom {
                pak_name: "Metroid4.pak",
                name: "Landing Site",
                mlvl: 0x39f2de28,
                mrea: 0xb2701146,
            }
        } else {
            let elv = &ELEVATORS[idx];
            SpawnRoom {
                pak_name: elv.pak_name,
                name: elv.name,
                mlvl: elv.mlvl,
                mrea: elv.mrea,
            }
        }
    }

    pub fn frigate_spawn_room() -> SpawnRoom
    {
        SpawnRoom {
            pak_name: "Metroid1.pak",
            name: "...",
            mlvl: 0x158EFE17,
            mrea: 0xD1241219,
        }
    }
}

pub const ELEVATORS: &'static [Elevator] = &[
    Elevator {
        pak_name: "Metroid2.pak",
        name: "Chozo Ruins West",// "Transport to Tallon Overworld North",
        mlvl: 0x83f6ff6f,
        mrea: 0x3e6b2bb7,
        scly_id: 0x007d,

        room_strg: 0xF747143D,
        hologram_strg: 0xD3F29D19,
        control_strg: 0x3C6FF426,

        default_dest: 6,
    },
    Elevator {
        pak_name: "Metroid2.pak",
        name: "Chozo Ruins North",// "Transport to Magmoor Caverns North",
        mlvl: 0x83f6ff6f,
        mrea: 0x8316edf5,
        scly_id: 0x180027,

        room_strg: 0x71D36693,
        hologram_strg: 0xB4B44968,
        control_strg: 0xC610DFE6,

        default_dest: 14,
    },
    Elevator {
        pak_name: "Metroid2.pak",
        name: "Chozo Ruins East",// "Transport to Tallon Overworld East",
        mlvl: 0x83f6ff6f,
        mrea: 0xa5fa69a1,
        scly_id: 0x3e002c,

        room_strg: 0x1CE1DDBC,
        hologram_strg: 0x598EF87A,
        control_strg: 0xFCD69EB0,

        default_dest: 8,
    },
    Elevator {
        pak_name: "Metroid2.pak",
        name: "Chozo Ruins South",// "Transport to Tallon Overworld South",
        mlvl: 0x83f6ff6f,
        mrea: 0x236e1b0f,
        scly_id: 0x3f0028,

        room_strg: 0x9A75AF12,
        hologram_strg: 0x48F39203,
        control_strg: 0x411CF27E,

        default_dest: 10,
    },

    Elevator {
        pak_name: "Metroid3.pak",
        name: "Phendrana Drifts North",// "Transport to Magmoor Caverns West",
        mlvl: 0xa8be6291,
        mrea: 0xc00e3781,
        scly_id: 0x002d,

        room_strg: 0xF7D14F4D,
        hologram_strg: 0x38F9BAC5,
        control_strg: 0x2DDB22E1,

        default_dest: 15,
    },
    Elevator {
        pak_name: "Metroid3.pak",
        name: "Phendrana Drifts South",// "Transport to Magmoor Caverns South",
        mlvl: 0xa8be6291,
        mrea: 0xdd0b0739,
        scly_id: 0x1d005a,

        room_strg: 0xEAD47FF5,
        hologram_strg: 0x0CEE0B66,
        control_strg: 0x993CEFE8,

        default_dest: 18,
    },

    Elevator {
        pak_name: "Metroid4.pak",
        name: "Tallon Overworld North",// "Transport to Chozo Ruins West",
        mlvl: 0x39f2de28,
        mrea: 0x11a02448,
        scly_id: 0xe0005,

        room_strg: 0x9EE2172A,
        hologram_strg: 0x04685AE9,
        control_strg: 0x73A833EB,

        default_dest: 0,
    },

    // XXX Two?
    /* Elevator {
        pak_name: "Metroid4.pak",
        mlvl: 0x39f2de28,
        mrea: 0x2398e906,
        scly_id: 0x1002d1, // Artifact Temple

        room_strg: 0xFFFFFFFF,
        hologram_strg: 0x00000000,
        control_strg: 0xFFFFFFFF,
    }, */

    Elevator {
        pak_name: "Metroid4.pak",
        name: "Artifact Temple",
        mlvl: 0x39f2de28,
        mrea: 0x2398e906,
        scly_id: 0x1002da,

        room_strg: 0xFFFFFFFF,
        hologram_strg: 0xFFFFFFFF,
        control_strg: 0xFFFFFFFF,

        default_dest: 19,
    },
    Elevator {
        pak_name: "Metroid4.pak",
        name: "Tallon Overworld East",// "Transport to Chozo Ruins East",
        mlvl: 0x39f2de28,
        mrea: 0x8a31665e,
        scly_id: 0x160038,

        room_strg: 0x0573553C,
        hologram_strg: 0x55A27CA9,
        control_strg: 0x51DCA8D9,

        default_dest: 2,
    },
    Elevator {
        pak_name: "Metroid4.pak",
        name: "Tallon Overworld West",// "Transport to Magmoor Caverns East",
        mlvl: 0x39f2de28,
        mrea: 0x15d6ff8b,
        scly_id: 0x170032,

        room_strg: 0xF92C2264,
        hologram_strg: 0xD658ADBD,
        control_strg: 0x8EA61E34,

        default_dest: 16,
    },
    Elevator {
        pak_name: "Metroid4.pak",
        name: "Tallon Overworld South",// "Transport to Chozo Ruins South",
        mlvl: 0x39f2de28,
        mrea: 0xca514f0,
        scly_id: 0x290024,

        room_strg: 0x630EA5FC,
        hologram_strg: 0xCC401AA8,
        control_strg: 0xEC16C417,

        default_dest: 3,
    },
    Elevator {
        pak_name: "Metroid4.pak",
        name: "Tallon Overworld South",// "Transport to Phazon Mines East",
        mlvl: 0x39f2de28,
        mrea: 0x7d106670,
        scly_id: 0x2b0023,

        room_strg: 0xF2525512,
        hologram_strg: 0x4921B661,
        control_strg: 0x294EC2B2,

        default_dest: 12,
    },

    Elevator {
        pak_name: "metroid5.pak",
        name: "Phazon Mines East",// "Transport to Tallon Overworld South",
        mlvl: 0xb1ac4d65,
        mrea: 0x430e999c,
        scly_id: 0x001c,

        room_strg: 0x8D7B16B4,
        hologram_strg: 0xB60F6ADF,
        control_strg: 0xA00EF446,

        default_dest: 11,
    },
    Elevator {
        pak_name: "metroid5.pak",
        name: "Phazon Mines West",// "Transport to Magmoor Caverns South",
        mlvl: 0xb1ac4d65,
        mrea: 0xe2c2cf38,
        scly_id: 0x190011,

        room_strg: 0x47C4108D,
        hologram_strg: 0xDFD2AE6D,
        control_strg: 0x1D8BB16C,

        default_dest: 17,
    },

    Elevator {
        pak_name: "Metroid6.pak",
        name: "Magmoor Caverns North",// "Transport to Chozo Ruins North",
        mlvl: 0x3ef8237c,
        mrea: 0x3beaadc9,
        scly_id: 0x001f,

        room_strg: 0x1BEFC19B,
        hologram_strg: 0x8EA3FD98,
        control_strg: 0x0D3EC7DC,

        default_dest: 1,
    },
    Elevator {
        pak_name: "Metroid6.pak",
        name: "Magmoor Caverns West",// "Transport to Phendrana Drifts North",
        mlvl: 0x3ef8237c,
        mrea: 0xdca9a28b,
        scly_id: 0xd0022,

        room_strg: 0xE0E1C4DA,
        hologram_strg: 0x4F2D2258,
        control_strg: 0xD0A81E59,

        default_dest: 4,
    },
    Elevator {
        pak_name: "Metroid6.pak",
        name: "Magmoor Caverns East",// "Transport to Tallon Overworld West",
        mlvl: 0x3ef8237c,
        mrea: 0x4c3d244c,
        scly_id: 0x100020,

        room_strg: 0xBD4E14B9,
        hologram_strg: 0x58DA42EA,
        control_strg: 0x4BE9A4CC,

        default_dest: 9,
    },
    Elevator {
        pak_name: "Metroid6.pak",
        name: "Magmoor Caverns South",// "Transport to Phazon Mines West",
        mlvl: 0x3ef8237c,
        mrea: 0xef2f1440,
        scly_id: 0x1a0024,

        room_strg: 0xFF5F6594,
        hologram_strg: 0x28E3D615,
        control_strg: 0x2FAF7EDA,

        default_dest: 13,
    },
    Elevator {
        pak_name: "Metroid6.pak",
        name: "Magmoor Caverns South",// "Transport to Phendrana Drifts South",
        mlvl: 0x3ef8237c,
        mrea: 0xc1ac9233,
        scly_id: 0x1b0028,

        room_strg: 0x66DEBE97,
        hologram_strg: 0x61805AFF,
        control_strg: 0x6F30E3D4,

        default_dest: 5,
    },

    Elevator {
        pak_name: "Metroid7.pak",
        name: "Crater Entry Point",
        mlvl: 0xc13b09d1,
        mrea: 0x93668996,
        scly_id: 0x0098,

        room_strg: 0xFFFFFFFF,
        hologram_strg: 0xFFFFFFFF,
        control_strg: 0xFFFFFFFF,

        default_dest: 7,
    },
    /* Elevator {
        pak_name: "Metroid7.pak",
        mlvl: 0xc13b09d1,
        mrea: 0x1a666c55,
        scly_id: 0xb0182,// Metroid Prime Lair

        room_strg: 0xFFFFFFFF,
        hologram_strg: 0x00000000,
        control_strg: 0xFFFFFFFF,
    }, */

];


