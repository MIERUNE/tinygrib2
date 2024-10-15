use std::io::Read;

use super::GribRead;
use crate::Result;

#[derive(Debug)]
pub struct DataRepresentationTemplate5_0 {
    pub reference_value: f32,
    pub binary_scale_factor: i16,
    pub decimal_scale_factor: i16,
    pub bits_per_value: u8,
    pub type_of_original_field_values: u8,
}

impl DataRepresentationTemplate5_0 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            reference_value: reader.read_grib_value()?,
            binary_scale_factor: reader.read_grib_value()?,
            decimal_scale_factor: reader.read_grib_value()?,
            bits_per_value: reader.read_grib_value()?,
            type_of_original_field_values: reader.read_grib_value()?,
        })
    }
}

/// Template 5.200 (Run length packing with level values)
#[derive(Debug)]
pub struct DataRepresentationTemplate5_200 {
    pub number_of_bits: u8,
    pub mv: u16,
    pub mvl: u16,
    pub decimal_scale_factor: i8,
    pub mvl_scaled_representative_values: Vec<i16>,
}

impl DataRepresentationTemplate5_200 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut tmpl = Self {
            number_of_bits: reader.read_grib_value()?,
            mv: reader.read_grib_value()?,
            mvl: reader.read_grib_value()?,
            decimal_scale_factor: reader.read_grib_value()?,
            mvl_scaled_representative_values: Vec::new(),
        };
        tmpl.mvl_scaled_representative_values
            .reserve(tmpl.mvl.into());
        for _ in 0..tmpl.mvl {
            tmpl.mvl_scaled_representative_values
                .push(reader.read_grib_value()?);
        }
        Ok(tmpl)
    }
}
