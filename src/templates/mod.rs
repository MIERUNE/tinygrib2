pub mod data;
pub mod data_representation;
pub mod grid_definition;
pub mod product_definition;

use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;
use std::io::Result;

pub use data::*;
pub use data_representation::*;
pub use grid_definition::*;
pub use product_definition::*;

pub trait FromGribValue: Sized {
    fn from_grib_reader(reader: impl ReadBytesExt) -> Result<Self>;
}

impl FromGribValue for u8 {
    fn from_grib_reader(mut reader: impl ReadBytesExt) -> Result<Self> {
        reader.read_u8()
    }
}

impl FromGribValue for i8 {
    fn from_grib_reader(mut reader: impl ReadBytesExt) -> Result<Self> {
        Ok(match reader.read_u8()? {
            u if u < 0x80 => u as i8,
            u => -((u & 0x7F) as i8),
        })
    }
}

impl FromGribValue for u16 {
    fn from_grib_reader(mut reader: impl ReadBytesExt) -> Result<Self> {
        reader.read_u16::<BigEndian>()
    }
}

impl FromGribValue for i16 {
    fn from_grib_reader(mut reader: impl ReadBytesExt) -> Result<Self> {
        Ok(match reader.read_u16::<BigEndian>()? {
            u if u < 0x8000 => u as i16,
            u => -((u & 0x7fff) as i16),
        })
    }
}

impl FromGribValue for f32 {
    fn from_grib_reader(mut reader: impl ReadBytesExt) -> Result<Self> {
        reader.read_f32::<BigEndian>()
    }
}

impl FromGribValue for u32 {
    fn from_grib_reader(mut reader: impl ReadBytesExt) -> Result<Self> {
        reader.read_u32::<BigEndian>()
    }
}

impl FromGribValue for i32 {
    fn from_grib_reader(mut reader: impl ReadBytesExt) -> Result<Self> {
        Ok(match reader.read_u32::<BigEndian>()? {
            u if u < 0x80000000 => u as i32,
            u => -((u & 0x7FFFFFFF) as i32),
        })
    }
}

impl FromGribValue for u64 {
    fn from_grib_reader(mut reader: impl ReadBytesExt) -> Result<Self> {
        reader.read_u64::<BigEndian>()
    }
}

pub trait GribRead: ReadBytesExt {
    fn read_grib_value<T: FromGribValue>(&mut self) -> Result<T> {
        T::from_grib_reader(self)
    }
}

impl<T: Read> GribRead for T {}

pub fn read_octets<R: ReadBytesExt>(mut reader: R, n: u8) -> std::io::Result<i32> {
    Ok(match n {
        1 => i8::from_grib_reader(reader)? as i32,
        2 => i16::from_grib_reader(reader)? as i32,
        3 => match reader.read_u24::<byteorder::BigEndian>()? {
            u if u < 0x800000 => u as i32,
            u => -((u & 0x7FFFFF) as i32),
        },
        4 => i32::from_grib_reader(reader)?,
        _ => unreachable!(),
    })
}
