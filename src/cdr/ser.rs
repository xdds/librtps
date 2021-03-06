use serde::{ Serialize, Serializer };

use std;
use std::error::Error;
use serde;

use byteorder::{LittleEndian, BigEndian, WriteBytesExt};

use std::fmt::{ Display, Formatter };
use std::fmt::Error as FmtError;

use std::io::Write;

use super::super::common_types::Endianness;

#[derive(Debug)]
pub struct CdrError{
    pub reason: String
}

impl Display for CdrError {
    fn fmt(&self, _: &mut Formatter) -> Result<(), FmtError> {
        unimplemented!();
    }
}

impl std::error::Error for CdrError {
    fn description(&self) -> &str {
        "NO"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

impl serde::ser::Error for CdrError {
    fn custom<T: Into<String>>(_: T) -> Self {
        CdrError{
            reason: "fdsa".to_string()
        }
    }
}

impl From<std::io::Error> for CdrError {
    fn from(err: std::io::Error) -> Self {
        CdrError{
            reason: format!("{:?}", err)
        }
    }
}

pub struct CdrSerializer<W> where W: Write {
    pub endianness: Endianness,
    pub write_handle: W
}

impl<W> CdrSerializer<W> where W: Write {
    pub fn new(w: W) -> Self {
        CdrSerializer {
            endianness: Endianness::Big,
            write_handle: w
        }
    }
}

impl<W: Write> Serializer for CdrSerializer<W> {
    type Error = CdrError;
    type SeqState = ();
    type TupleState = ();
    type TupleStructState = ();
    type TupleVariantState = ();
    type MapState = ();
    type StructState = ();
    type StructVariantState = ();

    fn serialize_bool(&mut self, _ /* v */: bool) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn serialize_i8(&mut self, val:i8) -> Result<(), Self::Error> {
        match self.write_handle.write_i8(val) {
            Ok(_) => Ok(()),
            Err(err) => Err(CdrError{
                reason: err.description().to_string()
            })
        }
    }

    fn serialize_i16(&mut self, _:i16) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn serialize_i32(&mut self, v:i32) -> Result<(), Self::Error> {
        match self.endianness {
            Endianness::Little => {
                match self.write_handle.write_i32::<LittleEndian>(v) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(CdrError{
                        reason: err.description().to_string()
                    })
                }
            },
            Endianness::Big => {
                match self.write_handle.write_i32::<BigEndian>(v) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(CdrError{
                        reason: err.description().to_string()
                    })
                }
            }
        }
    }

    fn serialize_i64(&mut self, _ /* v */: i64) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn serialize_isize(&mut self, _:isize) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn serialize_usize(&mut self, _:usize) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn serialize_u8(&mut self, v:u8) -> Result<(), Self::Error> {
        let buf = [v];

        match self.write_handle.write(&buf) {
            Ok(_) => Ok(()),
            Err(err) => Err(CdrError{
                reason: err.description().to_string()
            })
        }
    }

    fn serialize_u16(&mut self, v:u16) -> Result<(), Self::Error> {
        match self.endianness {
            Endianness::Little => {
                match self.write_handle.write_u16::<LittleEndian>(v) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(CdrError{
                        reason: err.description().to_string()
                    })
                }
            },
            Endianness::Big => {
                match self.write_handle.write_u16::<BigEndian>(v) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(CdrError{
                        reason: err.description().to_string()
                    })
                }
            }
        }
    }

    fn serialize_u32(&mut self, v:u32) -> Result<(), Self::Error> {
        match self.endianness {
            Endianness::Little => {
                match self.write_handle.write_u32::<LittleEndian>(v) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(CdrError{
                        reason: "dddd".to_string()
                    })
                }
            },
            Endianness::Big => {
                match self.write_handle.write_u32::<BigEndian>(v) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(CdrError{
                        reason: "dddd".to_string()
                    })
                }
            }
        }

    }

    fn serialize_u64(&mut self, v: u64) -> Result<(), Self::Error> {
        match self.endianness {
            Endianness::Little => {
                try!(self.write_handle.write_u64::<LittleEndian>(v));
                Ok(())
            },
            Endianness::Big => {
                try!(self.write_handle.write_u64::<BigEndian>(v));
                Ok(())
            }
        }
    }

    fn serialize_f32(&mut self, _ /* v */: f32) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn serialize_f64(&mut self, _ /* v */: f64) -> Result<(), Self::Error> {
        unimplemented!();
    }


    fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error> {
        match self.write_handle.write_all(value.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(CdrError{
                reason: err.description().to_string()
            })
        }
    }

    fn serialize_unit(&mut self) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_unit_struct(&mut self, _: &'static str) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_unit_variant(&mut self, _: &'static str, _: usize, _: &'static str) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn serialize_newtype_struct<T: Serialize>(&mut self, _: &'static str, _: T) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_newtype_variant<T: Serialize>(&mut self, _: &'static str, _: usize, _: &'static str, _: T) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn serialize_none(&mut self) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_some<V> (&mut self, _ /* value */: V) -> Result<(), Self::Error> /* where V: Serializer */ {
        unimplemented!();
    }

    fn serialize_seq(&mut self, seq_len: Option<usize>) -> Result<(), Self::Error> {
        match seq_len {
            Some(len) => self.serialize_u32(len as u32),
            None => unimplemented!()
        }
    }

    fn serialize_seq_fixed_size(&mut self, _: usize) -> Result<(), Self::Error> {
        Ok(())
    }

    fn serialize_seq_elt<T>(&mut self, _: &mut Self::SeqState, value: T) -> Result<(), Self::Error> where T: Serialize {
        value.serialize(self)
    }

    fn serialize_seq_end(&mut self, _: Self::SeqState) -> Result<(), Self::Error> {
        Ok(())
    }

    fn serialize_map(&mut self, _: Option<usize>) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_map_key<T: Serialize>(&mut self, _: &mut Self::MapState, _: T) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_map_value<T: Serialize>(&mut self, _: &mut Self::MapState, _: T) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_map_end(&mut self, _: Self::MapState) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn serialize_char(&mut self, _: char) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn serialize_bytes(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        match self.write_handle.write_all(buf) {
            Ok(_) => Ok(()),
            Err(reason) => Err(CdrError{ reason: format!("serialize_bytes err {:?}", reason).to_string() })
        }
    }

    fn serialize_tuple(&mut self, _: usize) -> Result<Self::TupleState, Self::Error> {
        unimplemented!();
    }
    fn serialize_tuple_elt<T: Serialize>(&mut self, _: &mut Self::TupleState, _: T) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_tuple_end(&mut self, _: Self::TupleState) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_tuple_struct(&mut self, _: &'static str, _: usize) -> Result<Self::TupleStructState, Self::Error> {
        unimplemented!();
    }
    fn serialize_tuple_struct_elt<T: Serialize>(&mut self, _: &mut Self::TupleStructState, _: T) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_tuple_struct_end(&mut self, _: Self::TupleStructState) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_tuple_variant(&mut self, _: &'static str, _: usize, _: &'static str, _: usize) -> Result<Self::TupleVariantState, Self::Error> {
        unimplemented!();
    }
    fn serialize_tuple_variant_elt<T: Serialize>(&mut self, _: &mut Self::TupleVariantState, _: T) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_tuple_variant_end(&mut self, _: Self::TupleVariantState) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn serialize_struct(&mut self, _: &'static str, _: usize) -> Result<Self::StructState, Self::Error> {
        Ok(())
    }
    fn serialize_struct_elt<V: Serialize>(&mut self, _: &mut Self::StructState, _: &'static str, visitor: V) -> Result<(), Self::Error> {
        visitor.serialize(self)
    }
    fn serialize_struct_end(&mut self, _: Self::StructState) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_struct_variant(&mut self, _: &'static str, _: usize, _: &'static str, _: usize) -> Result<Self::StructVariantState, Self::Error> {
        unimplemented!();
    }
    fn serialize_struct_variant_elt<V: Serialize>(&mut self, _: &mut Self::StructVariantState, _: &'static str, _: V) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn serialize_struct_variant_end(&mut self, _: Self::StructVariantState) -> Result<(), Self::Error> {
        unimplemented!();
    }
}
