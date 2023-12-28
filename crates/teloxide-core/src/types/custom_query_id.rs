use serde::{Deserialize, Serialize};

/// A unique message identifier.
#[derive(Clone, Copy, Debug, derive_more::Display, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "CustomQueryIdRaw", into = "CustomQueryIdRaw")]
pub struct CustomQueryId(pub i32);

// N.B. we [de]serialize `CustomCustomQueryId` as `{"custom_query_id":n}`, which
// means that if you want just an integer, you need to special case it with
// something like `serde(with = "crate::types::option_query_id_as_int")]`
//
// (we can't change the default format of `CustomCustomQueryId` because
// it's returned by some methods and we can't change serialization there)

#[derive(Serialize, Deserialize)]
struct CustomQueryIdRaw {
    custom_query_id: i32,
}

impl From<CustomQueryIdRaw> for CustomQueryId {
    fn from(CustomQueryIdRaw { custom_query_id }: CustomQueryIdRaw) -> Self {
        CustomQueryId(custom_query_id)
    }
}

impl From<CustomQueryId> for CustomQueryIdRaw {
    fn from(CustomQueryId(custom_query_id): CustomQueryId) -> Self {
        CustomQueryIdRaw { custom_query_id }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::CustomQueryId;

    #[test]
    fn smoke_deser() {
        let json = r#"{"custom_query_id":123}"#;
        let mid: CustomQueryId = serde_json::from_str(json).unwrap();
        assert_eq!(mid, CustomQueryId(123));
    }

    #[test]
    fn smoke_ser() {
        let mid: CustomQueryId = CustomQueryId(123);
        let json = serde_json::to_string(&mid).unwrap();
        assert_eq!(json, r#"{"custom_query_id":123}"#);
    }
}
