use crate::VecBTreeMap;
use core::fmt;
use core::marker::PhantomData;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

struct VecBTreeMapVisitor<K, V> {
    marker: PhantomData<fn() -> VecBTreeMap<K, V>>,
}

impl<K, V> VecBTreeMapVisitor<K, V> {
    const fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for VecBTreeMapVisitor<K, V>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    type Value = VecBTreeMap<K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map created by VecBTreeMap")
    }

    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut m = VecBTreeMap::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some((k, v)) = seq.next_element()? {
            if let Some((k, v)) = m.push(k, v) {
                m.insert(k, v);
            }
        }
        Ok(m)
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut m = VecBTreeMap::with_capacity(map.size_hint().unwrap_or(0));
        while let Some((k, v)) = map.next_entry()? {
            if let Some((k, v)) = m.push(k, v) {
                m.insert(k, v);
            }
        }
        Ok(m)
    }
}

impl<'de, K, V> Deserialize<'de> for VecBTreeMap<K, V>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(VecBTreeMapVisitor::new())
    }
}

impl<K, V> Serialize for VecBTreeMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut m = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            m.serialize_entry(k, v)?;
        }
        m.end()
    }
}
