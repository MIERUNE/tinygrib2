use std::io::Read;

use bitstream_io::{BigEndian, BitRead};
use byteorder::ReadBytesExt;
use itertools::Itertools;

use crate::templates::data_representation::DataRepresentationTemplate5_200;
use crate::templates::read_octets;
use crate::{Error, Result};

use super::{DataRepresentationTemplate5_0, DataRepresentationTemplate5_3};

/// Template 7.0: Grid point data - simple packing
///
/// NAN is represented as i32::MIN
pub fn read_data_7_0<R: Read>(
    reader: &mut R,
    number_of_values: u32,
    tmpl: &DataRepresentationTemplate5_0,
) -> Result<Vec<i32>> {
    let mut reader = bitstream_io::BitReader::<_, BigEndian>::new(reader);
    let mut values = Vec::with_capacity(number_of_values as usize);
    for _ in 0..number_of_values as usize {
        let v: u32 = reader.read_var(tmpl.bits_per_value as u32)?;
        // TODO: handle NA value?
        values.push(v as i32);
    }
    Ok(values)
}

/// Template 7.3: Grid point data - complex packing and spatial differencing
///
/// NAN is represented as i32::MIN
pub fn read_data_7_3<R: Read>(
    mut reader: &mut R,
    tmpl: &DataRepresentationTemplate5_3,
) -> Result<Vec<i32>> {
    let tmpl2 = &tmpl.template_2;
    let tmpl0 = &tmpl2.template_0;
    assert_eq!(
        tmpl.order_of_spatial_differencing, 2,
        "Only 2nd order is supported"
    );
    assert_eq!(tmpl.number_of_octets_extra_descriptors, 2);
    let z1: i32 = read_octets(&mut reader, tmpl.number_of_octets_extra_descriptors)?;
    let z2: i32 = read_octets(&mut reader, tmpl.number_of_octets_extra_descriptors)?;
    let z_min: i32 = read_octets(&mut reader, tmpl.number_of_octets_extra_descriptors)?;
    let ng = tmpl2.number_of_groups_of_data_values;
    let mut reader = bitstream_io::BitReader::<_, BigEndian>::new(&mut reader);
    let group_refs = (0..ng)
        .map(|_| reader.read_var::<u32>(tmpl0.bits_per_value as u32))
        .collect::<std::io::Result<Vec<u32>>>()?;
    reader.byte_align();
    let group_widths = (0..ng)
        .map(|_| reader.read_var::<u32>(tmpl2.number_of_bits_used_for_the_group_widths as u32))
        .collect::<std::io::Result<Vec<u32>>>()?;
    reader.byte_align();
    let group_lengths = (0..ng)
        .map(|_| reader.read_var::<u32>(tmpl2.number_of_bits_for_scaled_group_lengths as u32))
        .collect::<std::io::Result<Vec<u32>>>()?;
    reader.byte_align();
    let mut values: Vec<i32> = vec![];
    for (gi, ((gref, gw), gl)) in group_refs
        .into_iter()
        .zip_eq(group_widths)
        .zip_eq(group_lengths)
        .enumerate()
    {
        let group_width = tmpl2.reference_for_group_widths as u32 + gw;
        let group_length = if (gi as u32) < ng - 1 {
            tmpl2.reference_for_group_lengths
                + (tmpl2.length_increment_for_the_group_lengths as u32 * gl)
        } else {
            tmpl2.true_length_of_last_group
        };
        for _ in 0..group_length {
            let v = reader.read_var::<u32>(group_width)?;
            let value = z_min + gref as i32 + v as i32;
            values.push(value);
        }
    }
    values[0] = z1;
    values[1] = z2;
    for i in 2..values.len() {
        values[i] = values[i] + (2 * values[i - 1]) - values[i - 2];
    }
    Ok(values)
}

/// Template 7.200 (Run length packing with level values)
///
/// NAN is represented as i32::MIN
pub fn read_data_7_200<R: Read>(
    reader: &mut R,
    size: usize,
    number_of_values: u32,
    drs_template: &DataRepresentationTemplate5_200,
) -> Result<Vec<i32>> {
    if drs_template.number_of_bits != 8 {
        return Err(Error::UnsupportedData(format!(
            "Only supports 8 bits in our 7.200 implementation, but got {}",
            drs_template.number_of_bits
        )));
    }
    let mut values: Vec<i32> = Vec::with_capacity(number_of_values as usize);
    let mut lv = reader.read_u8()?;
    let mut p = 0;
    while p < size {
        p += 1;
        let mut run_length: u32 = 1;
        let mut m: u32 = 1;
        let mut next = 0;
        while p < size {
            next = reader.read_u8()?;
            if next as u16 > drs_template.mv {
                run_length += (next as u16 - drs_template.mv - 1) as u32 * m;
                m *= (255 - drs_template.mv) as u32;
                p += 1;
            } else {
                break;
            }
        }
        let value = match lv {
            0 => i32::MIN,
            _ => drs_template.mvl_scaled_representative_values[(lv - 1) as usize] as i32,
        };
        for _ in 0..run_length {
            values.push(value);
        }
        lv = next;
    }
    Ok(values)
}
