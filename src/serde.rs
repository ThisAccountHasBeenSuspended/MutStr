use serde::de::Visitor;

struct MutStrVisitor;

impl<'de> Visitor<'de> for MutStrVisitor {
    type Value = mutstr;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an `&str`")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(mutstr::from(v))
    }
}

impl<'de> serde::Deserialize<'de> for mutstr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(MutStrVisitor)
    }
}

impl serde::Serialize for mutstr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
mod serde_implementation {
    use super::mutstr;

    #[derive(Default, serde::Deserialize, serde::Serialize)]
    struct MyStruct {
        name: mutstr,
    }
    
    #[test]
    fn from_to() {
        let raw = r#"{"name":"Nick"}"#;
        let result = serde_json::from_str::<MyStruct>(raw).unwrap();
        assert_eq!(result.name.as_str(), "Nick");
        assert_eq!(serde_json::to_string(&result).unwrap(), raw);
    }
}