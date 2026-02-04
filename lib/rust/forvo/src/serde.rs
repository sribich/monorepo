use core::fmt::Display;

use railgun_error::Error;
use railgun_error::Location;
use serde::Serialize;
use serde::Serializer;

#[derive(Error)]
pub enum SerializationError {
    #[error(display("{type} can not be path serialized."))]
    UnsupportedType { r#type: String, location: Location },
    #[error(display("{msg}"))]
    Serde { msg: String },
}

impl serde::ser::Error for SerializationError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        SerdeContext {
            msg: msg.to_string(),
        }
        .build()
    }
}

pub struct ForvoRequestSerializer {
    output: String,
}

pub fn to_path<T>(value: T) -> core::result::Result<String, SerializationError>
where
    T: Serialize,
{
    let mut serializer = ForvoRequestSerializer {
        output: String::new(),
    };

    value.serialize(&mut serializer)?;

    Ok(serializer.output)
}

impl Serializer for &mut ForvoRequestSerializer {
    type Error = SerializationError;
    type Ok = ();
    type SerializeMap = serde::ser::Impossible<(), SerializationError>;
    type SerializeSeq = serde::ser::Impossible<(), SerializationError>;
    type SerializeStruct = Self;
    type SerializeStructVariant = serde::ser::Impossible<(), SerializationError>;
    type SerializeTuple = serde::ser::Impossible<(), SerializationError>;
    type SerializeTupleStruct = serde::ser::Impossible<(), SerializationError>;
    type SerializeTupleVariant = serde::ser::Impossible<(), SerializationError>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output += if v { "true" } else { "false" };

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();

        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();

        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();

        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output += v;

        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        UnsupportedTypeContext {
            r#type: "bytes".to_owned(),
        }
        .fail()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        UnsupportedTypeContext {
            r#type: "none".to_owned(),
        }
        .fail()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        UnsupportedTypeContext {
            r#type: "unit".to_owned(),
        }
        .fail()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        UnsupportedTypeContext {
            r#type: "unit_struct".to_owned(),
        }
        .fail()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        UnsupportedTypeContext {
            r#type: "unit_variant".to_owned(),
        }
        .fail()
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        UnsupportedTypeContext {
            r#type: "newtype_struct".to_owned(),
        }
        .fail()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        UnsupportedTypeContext {
            r#type: "newtype_variant".to_owned(),
        }
        .fail()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        UnsupportedTypeContext {
            r#type: "seq".to_owned(),
        }
        .fail()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        UnsupportedTypeContext {
            r#type: "tuple".to_owned(),
        }
        .fail()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        UnsupportedTypeContext {
            r#type: "tuple_struct".to_owned(),
        }
        .fail()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        UnsupportedTypeContext {
            r#type: "tuple_variant".to_owned(),
        }
        .fail()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        UnsupportedTypeContext {
            r#type: "map".to_owned(),
        }
        .fail()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        UnsupportedTypeContext {
            r#type: "struct_variant".to_owned(),
        }
        .fail()
    }
}

impl serde::ser::SerializeStruct for &mut ForvoRequestSerializer {
    type Error = SerializationError;
    type Ok = ();

    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> core::result::Result<(), SerializationError>
    where
        T: ?Sized + Serialize,
    {
        self.output += "/";
        key.serialize(&mut **self)?;
        self.output += "/";
        value.serialize(&mut **self)
    }

    fn end(self) -> core::result::Result<(), SerializationError> {
        Ok(())
    }
}
