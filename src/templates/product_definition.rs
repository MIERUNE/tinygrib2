use std::io::Read;

use super::GribRead;
use crate::Result;

/// Template 4.0 (analysis or forecast at a horizontal level or in a horizontal layer at a point in time)
#[derive(Debug)]
pub struct ProductDefinitionTemplate4_0 {
    pub parameter_category: u8,
    pub parameter_number: u8,
    pub type_of_generating_process: u8,
    pub background_process: u8,
    pub generating_process_identifier: u8,
    pub hours_after_data_cutoff: u16,
    pub minutes_after_data_cutoff: u8,
    pub indicator_of_unit_of_time_range: u8,
    pub forecast_time: i32,
    pub type_of_first_fixed_surface: u8,
    pub scale_factor_of_first_fixed_surface: i8,
    pub scaled_value_of_first_fixed_surface: u32,
    pub type_of_second_fixed_surface: u8,
    pub scale_factor_of_second_fixed_surface: i8,
    pub scaled_value_of_second_fixed_surface: u32,
}

impl ProductDefinitionTemplate4_0 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            parameter_category: reader.read_grib_value()?,
            parameter_number: reader.read_grib_value()?,
            type_of_generating_process: reader.read_grib_value()?,
            background_process: reader.read_grib_value()?,
            generating_process_identifier: reader.read_grib_value()?,
            hours_after_data_cutoff: reader.read_grib_value()?,
            minutes_after_data_cutoff: reader.read_grib_value()?,
            indicator_of_unit_of_time_range: reader.read_grib_value()?,
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

#[derive(Debug)]
pub struct ProductDefinitionTemplate4_1 {
    pub template_0: ProductDefinitionTemplate4_0,
    pub type_of_ensemble_forecast: u8,
    pub perturbation_number: u8,
    pub number_of_forecasts_in_ensemble: u8,
}

impl ProductDefinitionTemplate4_1 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            template_0: ProductDefinitionTemplate4_0::read(reader)?,
            type_of_ensemble_forecast: reader.read_grib_value()?,
            perturbation_number: reader.read_grib_value()?,
            number_of_forecasts_in_ensemble: reader.read_grib_value()?,
        })
    }
}

/// Template 4.8 (average, accumulation and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval)
#[derive(Debug)]
pub struct ProductDefinitionTemplate4_8 {
    pub template_0: ProductDefinitionTemplate4_0,
    pub interval: TimeInterval,
}

impl ProductDefinitionTemplate4_8 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            template_0: ProductDefinitionTemplate4_0::read(reader)?,
            interval: TimeInterval::read(reader)?,
        })
    }
}

#[derive(Debug)]
pub struct ProductDefinitionTemplate4_11 {
    pub template_1: ProductDefinitionTemplate4_1,
    pub interval: TimeInterval,
}

impl ProductDefinitionTemplate4_11 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            template_1: ProductDefinitionTemplate4_1::read(reader)?,
            interval: TimeInterval::read(reader)?,
        })
    }
}

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

#[derive(Debug)]
pub struct TimeInterval {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub time_ranges: Vec<TimeRange>,
}

impl TimeInterval {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            year: reader.read_grib_value()?,
            month: reader.read_grib_value()?,
            day: reader.read_grib_value()?,
            hour: reader.read_grib_value()?,
            minute: reader.read_grib_value()?,
            second: reader.read_grib_value()?,
            time_ranges: (0..reader.read_grib_value::<u8>()?)
                .map(|_| TimeRange::read(reader))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[derive(Debug)]
pub struct TimeRange {
    pub total_number_of_data_values_missing: u32,
    pub statistical_process: u8,
    pub type_of_time_increment: u8,
    pub indicator_of_unit_of_time: u8,
    pub length_of_the_time_range: u32,
    pub indicator_of_unit_of_length_of_time_range: u8,
    pub time_increment: u32,
}

impl TimeRange {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            total_number_of_data_values_missing: reader.read_grib_value()?,
            statistical_process: reader.read_grib_value()?,
            type_of_time_increment: reader.read_grib_value()?,
            indicator_of_unit_of_time: reader.read_grib_value()?,
            length_of_the_time_range: reader.read_grib_value()?,
            indicator_of_unit_of_length_of_time_range: reader.read_grib_value()?,
            time_increment: reader.read_grib_value()?,
        })
    }
}
