use std::{error::Error, io::Write};

use serde::Serialize;
use serde_json::{
    ser::{CompactFormatter, Formatter, PrettyFormatter},
    Serializer,
};

use crate::{SeedWorld, VERSION};

impl<Metadata: Serialize> SeedWorld<Metadata> {
    pub fn package<W: Write>(&self, w: W) -> Result<(), Box<dyn Error>> {
        self.package_with_formatter(w, CompactFormatter)
    }
    pub fn package_pretty<W: Write>(&self, w: W) -> Result<(), Box<dyn Error>> {
        self.package_with_formatter(w, PrettyFormatter::new())
    }
    fn package_with_formatter<W, F>(&self, w: W, formatter: F) -> Result<(), Box<dyn Error>>
    where
        W: Write,
        F: Formatter,
    {
        // TODO choose compression
        let mut builder = tar::Builder::new(xz2::write::XzEncoder::new(w, 9));
        // let mut builder = tar::Builder::new(brotli::CompressorWriter::new(w, 4096, 11, 22));
        // let mut builder = tar::Builder::new(w);
        let mut header = base_header();

        header.set_path("format_version")?;
        header.set_size(VERSION.len().try_into()?);
        header.set_cksum();
        builder.append(&header, VERSION.as_bytes())?;

        let mut seed = Vec::with_capacity(128); // TODO estimate capacity
        self.serialize(&mut Serializer::with_formatter(&mut seed, formatter))?;

        header.set_path("seed")?;
        header.set_size(seed.len().try_into()?);
        header.set_cksum();
        builder.append(&mut header, seed.as_slice())?;

        builder.into_inner()?.finish()?.flush()?;

        Ok(())
    }
}

fn base_header() -> tar::Header {
    let mut header = tar::Header::new_old();
    header.set_mode(0o644);
    header
}
