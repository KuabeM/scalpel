use super::hex_convert::convert_hex2bin;
use byte_offset::*;
use bytes::BytesMut;
use errors::*;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
pub enum FillPattern {
    Random,
    Zero,
    One,
}

impl Default for FillPattern {
    fn default() -> Self {
        FillPattern::Zero
    }
}




#[derive(Debug,Clone,Copy)]
enum MetaInfo {
    IntelHex,
    Bin
}


#[derive(Debug,Clone)]
struct AnnotatedBytes {
    pub bytes : BytesMut,
}


impl AnnotatedBytes {
    pub fn save(path : Path, meta_in : MetaInfo) -> Result<()> {
        match self.meta {
            MetaInfo::Bin => {
                let mut file = OpenOptions::new()
                    .truncate(true)
                    .write(true)
                    .create(true)
                    .open(path)?;
                // :)
                // TODO write bytes to file
            }
            MetaInfo::IntelHex => {
                intelhex::write_bin_as_hex_to_file(path, self.bytes)?;
            }
        }
    }

    pub fn load(path : Path, meta_out : MetaInfo) -> Result<Self> {
        match meta_out {
            MetaInfo::Bin => {
                let mut file = OpenOptions::new()
                    .read(true)
                    .open(path)?;
                // :)
                // TODO read bytes from file
            }
            MetaInfo::IntelHex => {
                AnnotatedBytes {
                    bytes : intelhex::convert_hex2bin(path)?,
                }
            }
        }
    }
}


impl AnnotatedBytes {

    pub fn stance(&mut self, start: ByteOffset, size : ByteOffset) -> Result<()> {

        // split file in part before and after start index
        self.bytes = self.bytes.split_off(start.as_usize() - 1);
        // split off everything after size
        self.bytes.split_off(size.as_usize());
        Ok(())
    }

    pub fn stitch(
        files: Vec<(AnnotatedBytes, ByteOffset)>,
        fill_pattern: FillPattern,
        meta_out : MetaInfo,
    ) -> Result<AnnotatedBytes> {

        files
            .iter()
            .try_fold(AnnotatedBytes::empty(MetaInfo::Bin), |stitched, (elem, offset)| {
                // before reading, check file ending
                let content = elem.convert_to(MetaInfo::Bin)?;

                match fill_pattern {
                    FillPattern::Zero => stitched.bytes.resize(*offset, 0x00),
                    FillPattern::One => stitched.bytes.resize(*offset, 0xFF),
                    FillPattern::Random => {
                        let mut padding = vec![0; *offset - stitched.bytes.len()];
                        ::rand::thread_rng().try_fill(&mut padding[..])?;
                        stitched.bytes.extend_from_slice(&padding);
                    }
                }
                stitched.bytes.extend_from_slice(&new);
                Ok(stitched)
            })
    }

    pub fn graft(&mut self, replace : AnnotatedBytes, start: ByteOffset, size : ByteOffset, fill_pattern : FillPattern) -> Result<()> {
        // [ prefix replaceme postfix]

        // split file in part before and after start index
        let mut output = self.bytes.clone();
        let after = output.split_off(start);

        output.extend_from_slice(&replace.bytes);

        // fill missing bytes
        match fill_pattern {
            FillPattern::Zero => output.resize(before.len() + size, 0x0),
            FillPattern::One => output.resize(before.len() + size, 0xFF),
            FillPattern::Random => {
                let mut padding = vec![0; size - replace.bytes.len()];
                ::rand::thread_rng().try_fill(&mut padding[..])?;
                output.extend_from_slice(&padding);
            }
        }

        // append the end
        output.extend_from_slice(&after[size..]);

        self.bytes = output;

        Ok(())
    }
}


// annby.cut().and_then(|x| {x.convert_to() }).or_else(|x| { x.convert_to().and_then(|x| {x.cut()} )})?


struct TestDataGraft {
    idx : usize,
    datasets : Vec<()>,
}

impl TestDataGraft {
    pub fn new() -> Self {
        Self {
            idx : 0,
            datasets : vec![
                (),
                (),
                (),
            ]
        }
    }
}

impl Iterator for X {
    type Item = (input, expected_output);
    fn next() -> Option<Self::Item> {

    }
}



#[test]
fn graft_everything() {
    for item in X::new() {
        item.graft();
    }
}



fn run() -> Result<()> {

    // read

    let meta_out = unimplemented!();
    let meta_in = unimplemented!();
    
    let bytes_in = unimplemented!();

    let mut work = AnnotatedBytes::load(args.path_in, meta_in);

    match cmd {
        "stance" => {
            work.stance()?;
        },
        "graft" => {
            work.graft()?;
        },
        "stitch" => {
            work = AnnotatedBytes::stitch(args.files, args.fill_pattern)?;
        },
        "convert" => { 
        },
        _ => Err(format_err!("Noooope")),
    }

    work.save(args.path_out, meta_out)?;

    Ok(())
}



quick_main!(run);
