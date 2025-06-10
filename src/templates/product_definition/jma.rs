use crate::{Result, templates::GribRead};
use std::io::Read;

use crate::templates::{ProductDefinitionTemplate4_0, ProductDefinitionTemplate4_8};

#[derive(Debug)]
pub struct ProductDefinitionTemplate4_50000 {
    pub template_0: ProductDefinitionTemplate4_0,
    pub base_product1: u8,
    pub hour_difference1: u16,
    pub minute_difference1: u8,
    pub base_product2: u8,
    pub hour_difference2: u16,
    pub minute_difference2: u8,
}

impl ProductDefinitionTemplate4_50000 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            template_0: ProductDefinitionTemplate4_0::read(reader)?,
            base_product1: reader.read_grib_value()?,
            hour_difference1: reader.read_grib_value()?,
            minute_difference1: reader.read_grib_value()?,
            base_product2: reader.read_grib_value()?,
            hour_difference2: reader.read_grib_value()?,
            minute_difference2: reader.read_grib_value()?,
        })
    }
}

#[derive(Debug)]
pub struct ProductDefinitionTemplate4_50011 {
    pub template_8: ProductDefinitionTemplate4_8,
    pub rader_operating_info1: u64,
    pub rader_operating_info2: u64,
    pub rader_operating_info3: u64,
}

impl ProductDefinitionTemplate4_50011 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            template_8: ProductDefinitionTemplate4_8::read(reader)?,
            rader_operating_info1: reader.read_grib_value()?,
            rader_operating_info2: reader.read_grib_value()?,
            rader_operating_info3: reader.read_grib_value()?,
        })
    }
}

#[derive(Debug)]
pub struct ProductDefinitionTemplate4_50031 {
    pub parameter_category: u8,
    pub parameter_number: u8,
    pub type_of_generating_process: u8,
    pub background_process: u8,
    pub generating_process_identifier: u8,

    pub tc_number: u16,
    pub typhoon_number: u16,

    pub indicator_of_unit_of_time_range_start: u8,
    pub start_time: i32,

    pub indicator_of_unit_of_time_range_forecast: u8,
    pub forecast_time: i32,

    pub type_of_first_fixed_surface: u8,
    pub scale_factor_of_first_fixed_surface: i8,
    pub scaled_value_of_first_fixed_surface: u32,
    pub type_of_second_fixed_surface: u8,
    pub scale_factor_of_second_fixed_surface: i8,
    pub scaled_value_of_second_fixed_surface: u32,
}

impl ProductDefinitionTemplate4_50031 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            parameter_category: reader.read_grib_value()?,
            parameter_number: reader.read_grib_value()?,
            type_of_generating_process: reader.read_grib_value()?,
            background_process: reader.read_grib_value()?,
            generating_process_identifier: reader.read_grib_value()?,
            tc_number: reader.read_grib_value()?,
            typhoon_number: reader.read_grib_value()?,
            indicator_of_unit_of_time_range_start: reader.read_grib_value()?,
            start_time: reader.read_grib_value()?,
            indicator_of_unit_of_time_range_forecast: reader.read_grib_value()?,
            forecast_time: reader.read_grib_value()?,
            type_of_first_fixed_surface: reader.read_grib_value()?,
            scale_factor_of_first_fixed_surface: reader.read_grib_value()?,
            scaled_value_of_first_fixed_surface: reader.read_grib_value()?,
            type_of_second_fixed_surface: reader.read_grib_value()?,
            scale_factor_of_second_fixed_surface: reader.read_grib_value()?,
            scaled_value_of_second_fixed_surface: reader.read_grib_value()?,
        })
    }
}
