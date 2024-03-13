use std::env;
use std::process;
use jpeg_converter::dct_mcu::C_QUANTIZATION_TABLE;
use jpeg_converter::dct_mcu::Y_QUANTIZATION_TABLE;
use jpeg_converter::encoding;
use jpeg_converter::Args;
use jpeg_converter::BitCode;
use jpeg_converter::BmpImage;
use jpeg_converter::RLEncoded;
use jpeg_converter::ShiftedYCR;
use jpeg_converter::SubsampledYCR;
use jpeg_converter::Writer;
use jpeg_converter::YcbcrImage;
use jpeg_converter::MCU;

fn main() {
    let files = Args::get(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let rgb_vec = BmpImage::get_from_file(files.in_file_path);
    let ycbcr_image = YcbcrImage::get_from_rgb(rgb_vec);
    /*
    println!("Y component size = {}", &ycbcr_image.y_component.len());
    println!("Y comp width : {}", &ycbcr_image.y_component[0].len());
    println!("Cb component size = {}", &ycbcr_image.cb_component.len());
    println!("Cb comp width : {}", &ycbcr_image.cb_component[0].len());
    println!("Cr component size = {}", &ycbcr_image.cr_component.len());
    println!("Cr comp width : {}", &ycbcr_image.cr_component[0].len());
    */

    let subsampled = SubsampledYCR::no_subsample(ycbcr_image);
    let shifted = ShiftedYCR::shift(subsampled);
    let quantized_image_mcu = MCU::get_mcus(shifted).process_image().unwrap();

    let mut writer = Writer::new(&files.out_file_path);
    
    writer.write_soi();
    let y_quant_table_id: u8 = 0;
    let c_quant_table_id: u8 = 1;
    writer.write_quant_table(&Y_QUANTIZATION_TABLE, y_quant_table_id);
    writer.write_quant_table(&C_QUANTIZATION_TABLE, c_quant_table_id);
    writer.write_sof(&quantized_image_mcu, y_quant_table_id as u32, c_quant_table_id as u32);
    writer.write_sos();


    let dc_lum_codes = BitCode::calculate_huffman_codes(encoding::DC_LUMINANCE_CODES_PER_BITSIZE.to_vec(), encoding::DC_LUMINANCE_VALUES.to_vec());
    let dc_chrom_codes = BitCode::calculate_huffman_codes(encoding::DC_CHROMINANCE_CODES_PER_BITSIZE.to_vec(), encoding::DC_CHROMINANCE_VALUES.to_vec());
    let ac_lum_codes = BitCode::calculate_huffman_codes(encoding::AC_LUMINANCE_CODES_PER_BITSIZE.to_vec(), encoding::AC_LUMINANCE_VALUES.to_vec());
    let ac_chrom_codes = BitCode::calculate_huffman_codes(encoding::AC_CHROMINANCE_CODES_PER_BITSIZE.to_vec(), encoding::AC_CHROMINANCE_VALUES.to_vec());

    let mut last_dc_lum: i8 = 0;
    let mut last_dc_cb: i8 = 0;
    let mut last_dc_cr: i8 = 0;

    println!("y MCU len = {}", quantized_image_mcu.y_mcu.len());
    println!("cb MCU len = {}", quantized_image_mcu.cb_mcu.len());
    println!("cr MCU len = {}", quantized_image_mcu.cr_mcu.len());

    for i in 0..quantized_image_mcu.y_mcu.len() {
       if i == 0 {
            let y_encoded = RLEncoded::new(quantized_image_mcu.y_mcu[i].zig_zag_traversal());
            last_dc_lum = y_encoded.dc_coef;
            let dc_bitcode = BitCode::get_dc_bitcode(y_encoded.dc_coef, &dc_lum_codes);
            let mut ac_bitcodes: Vec<BitCode> = Vec::new();
            for pair in y_encoded.ac_coefs {
                ac_bitcodes.push(BitCode::get_ac_bitcode(pair, &ac_lum_codes));
            }
            writer.write_bitcode(&dc_bitcode);
            for bc in ac_bitcodes {
                writer.write_bitcode(&bc);
            }

            let cb_encoded = RLEncoded::new(quantized_image_mcu.cb_mcu[i].zig_zag_traversal());
            last_dc_cb = cb_encoded.dc_coef;
            let dc_bitcode = BitCode::get_dc_bitcode(cb_encoded.dc_coef, &dc_chrom_codes);
            let mut ac_bitcodes: Vec<BitCode> = Vec::new();
            for pair in cb_encoded.ac_coefs {
                ac_bitcodes.push(BitCode::get_ac_bitcode(pair, &ac_chrom_codes));
            }
            writer.write_bitcode(&dc_bitcode);
            for bc in ac_bitcodes {
                writer.write_bitcode(&bc);
            }

            let cr_encoded = RLEncoded::new(quantized_image_mcu.cr_mcu[i].zig_zag_traversal());
            last_dc_cr = cr_encoded.dc_coef;
            let dc_bitcode = BitCode::get_dc_bitcode(cr_encoded.dc_coef, &dc_chrom_codes);
            let mut ac_bitcodes: Vec<BitCode> = Vec::new();
            for pair in cr_encoded.ac_coefs {
                ac_bitcodes.push(BitCode::get_ac_bitcode(pair, &ac_chrom_codes));
            }
            writer.write_bitcode(&dc_bitcode);
            for bc in ac_bitcodes {
                writer.write_bitcode(&bc);
            }
        } else {
            let y_encoded = RLEncoded::new(quantized_image_mcu.y_mcu[i].zig_zag_traversal());
            let dc_bitcode = BitCode::get_dc_bitcode(y_encoded.dc_coef - last_dc_lum, &dc_lum_codes);
            let mut ac_bitcodes: Vec<BitCode> = Vec::new();
            for pair in y_encoded.ac_coefs {
                ac_bitcodes.push(BitCode::get_ac_bitcode(pair, &ac_lum_codes));
            }
            writer.write_bitcode(&dc_bitcode);
            for bc in ac_bitcodes {
                writer.write_bitcode(&bc);
            }
            last_dc_lum = y_encoded.dc_coef;

            let cb_encoded = RLEncoded::new(quantized_image_mcu.cb_mcu[i].zig_zag_traversal());
            let dc_bitcode = BitCode::get_dc_bitcode(cb_encoded.dc_coef - last_dc_cb, &dc_chrom_codes);
            let mut ac_bitcodes: Vec<BitCode> = Vec::new();
            for pair in cb_encoded.ac_coefs {
                ac_bitcodes.push(BitCode::get_ac_bitcode(pair, &ac_chrom_codes));
            }
            writer.write_bitcode(&dc_bitcode);
            for bc in ac_bitcodes {
                writer.write_bitcode(&bc);
            }
            last_dc_cb = cb_encoded.dc_coef;

            let cr_encoded = RLEncoded::new(quantized_image_mcu.cr_mcu[i].zig_zag_traversal());
            let dc_bitcode = BitCode::get_dc_bitcode(cr_encoded.dc_coef - last_dc_cr, &dc_chrom_codes);
            let mut ac_bitcodes: Vec<BitCode> = Vec::new();
            for pair in cr_encoded.ac_coefs {
                ac_bitcodes.push(BitCode::get_ac_bitcode(pair, &ac_chrom_codes));
            }
            writer.write_bitcode(&dc_bitcode);
            for bc in ac_bitcodes {
                writer.write_bitcode(&bc);
            }
            last_dc_cr = cr_encoded.dc_coef;
        }
    }

    writer.write_rest();

    writer.write_eoi();
}
