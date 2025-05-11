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

#[derive(Debug)]
pub struct DataRepresentationTemplate5_2 {
    pub template_0: DataRepresentationTemplate5_0,
    pub group_splitting_method_used: u8,
    pub missing_value_management_used: u8,
    pub primary_missing_value_substitute: u32,
    pub secondary_missing_value_substitute: u32,
    pub number_of_groups_of_data_values: u32,
    pub reference_for_group_widths: u8,
    pub number_of_bits_used_for_the_group_widths: u8,
    pub reference_for_group_lengths: u32,
    pub length_increment_for_the_group_lengths: u8,
    pub true_length_of_last_group: u32,
    pub number_of_bits_for_scaled_group_lengths: u8,
}

impl DataRepresentationTemplate5_2 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            template_0: DataRepresentationTemplate5_0::read(reader)?,
            group_splitting_method_used: reader.read_grib_value()?,
            missing_value_management_used: reader.read_grib_value()?,
            primary_missing_value_substitute: reader.read_grib_value()?,
            secondary_missing_value_substitute: reader.read_grib_value()?,
            number_of_groups_of_data_values: reader.read_grib_value()?,
            reference_for_group_widths: reader.read_grib_value()?,
            number_of_bits_used_for_the_group_widths: reader.read_grib_value()?,
            reference_for_group_lengths: reader.read_grib_value()?,
            length_increment_for_the_group_lengths: reader.read_grib_value()?,
            true_length_of_last_group: reader.read_grib_value()?,
            number_of_bits_for_scaled_group_lengths: reader.read_grib_value()?,
        })
    }
}

#[derive(Debug)]
pub struct DataRepresentationTemplate5_3 {
    pub template_2: DataRepresentationTemplate5_2,
    pub order_of_spatial_differencing: u8,
    pub number_of_octets_extra_descriptors: u8,
}

impl DataRepresentationTemplate5_3 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            template_2: DataRepresentationTemplate5_2::read(reader)?,
            order_of_spatial_differencing: reader.read_grib_value()?,
            number_of_octets_extra_descriptors: reader.read_grib_value()?,
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
