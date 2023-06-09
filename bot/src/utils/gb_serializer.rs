use serde::{ser::{self, SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeMap, SerializeStruct, SerializeStructVariant}, Serialize};
use bson::{
    ser::{
        Error,
        Result
    },
    Bson,
    Binary,
    to_bson,
    spec::BinarySubtype, Document, Array
};

pub struct Serializer;

impl Serializer {
    pub fn new() -> Serializer {
        Serializer
    }
}

impl ser::Serializer for Serializer {
    type Ok = Bson;
    type Error = Error;

    type SerializeSeq = ArraySerializer;
    type SerializeTuple = TupleSerializer;
    type SerializeTupleStruct = TupleStructSerializer;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = StructSerializer;
    type SerializeStructVariant = StructVariantSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Ok(Bson::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i32(v as i32)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i32(v as i32)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Ok(Bson::Int32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(Bson::Int64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        Ok(Bson::Int32(v as i32))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        Ok(Bson::Int32(v as i32))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        Ok(Bson::Int64(v as i64))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {

        match i64::try_from(v) {
            Ok(ivalue) => Ok(Bson::Int64(ivalue)),
            Err(_) => Err(Error::UnsignedIntegerExceededRange(v)),
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Ok(Bson::Double(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let mut s = String::new();
        s.push(v);
        self.serialize_str(&s)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(Bson::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Ok(Bson::Binary(Binary {
            subtype: BinarySubtype::Generic,
            bytes: v.to_vec(),
        }))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(Bson::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(Bson::String(variant.to_string()))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: serde::Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: serde::Serialize {

        let mut newtype_variant = Document::new();
        newtype_variant.insert(variant, to_bson(value)?);
        Ok(newtype_variant.into())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(ArraySerializer { inner: Array::with_capacity(len.unwrap_or(0)) })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(TupleSerializer { inner: Array::with_capacity(len) })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(TupleStructSerializer { inner: Array::with_capacity(len) })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(TupleVariantSerializer { inner: Array::with_capacity(len), name: variant })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapSerializer { inner: Document::new(), next_key: None })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(StructSerializer { inner: Document::new() })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(StructVariantSerializer { inner: Document::new(), name: variant })
    }
}

pub struct ArraySerializer {
    inner: Array
}

impl SerializeSeq for ArraySerializer {
    type Ok = Bson;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()>
        where
            T: serde::Serialize {
        self.inner.push(to_bson(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Bson::Array(self.inner))
    }
}

pub struct TupleSerializer {
    inner: Array
}

impl SerializeTuple for TupleSerializer {
    type Ok = Bson;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()>
        where
            T: serde::Serialize {
        self.inner.push(to_bson(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Bson::Array(self.inner))
    }
}

pub struct TupleStructSerializer {
    inner: Array
}

impl SerializeTupleStruct for TupleStructSerializer {
    type Ok = Bson;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()>
        where
            T: serde::Serialize {
        self.inner.push(to_bson(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Bson::Array(self.inner))
    }
}

pub struct TupleVariantSerializer {
    inner: Array,
    name: &'static str,
}

impl SerializeTupleVariant for TupleVariantSerializer {
    type Ok = Bson;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()>
        where
            T: serde::Serialize {
        self.inner.push(to_bson(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let mut tuple_variant = Document::new();
        tuple_variant.insert(self.name, self.inner);
        Ok(tuple_variant.into())
    }
}

pub struct MapSerializer {
    inner: Document,
    next_key: Option<String>
}

impl SerializeMap for MapSerializer {
    type Ok = Bson;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
        where
            T: Serialize {
        self.next_key = match to_bson(key)? {
            Bson::String(s) => Some(s),
            other => return Err(Error::InvalidDocumentKey(other)),
        };
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize {
        let key = self.next_key.take().unwrap_or_default();
        self.inner.insert(key, to_bson(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Bson> {
        Ok(Bson::from(self.inner))
    }
}

pub struct StructSerializer {
    inner: Document,
}

impl SerializeStruct for StructSerializer {
    type Ok = Bson;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<()>
        where
            T: Serialize {
        self.inner.insert(key, to_bson(value)?);
        Ok(())
    }
    
    fn end(self) -> Result<Self::Ok> {
        Ok(Bson::from(self.inner))
    }
}

pub struct StructVariantSerializer {
    inner: Document,
    name: &'static str,
}

impl SerializeStructVariant for StructVariantSerializer {
    type Ok = Bson;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<()>
        where
            T: Serialize {
        self.inner.insert(key, to_bson(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let var = Bson::from(self.inner);

        let mut struct_variant = Document::new();
        struct_variant.insert(self.name, var);

        Ok(Bson::Document(struct_variant))
    }
}