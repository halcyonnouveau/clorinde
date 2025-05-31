use postgres_types::{FromSql, IsNull, ToSql, Type, to_sql_checked};
use std::error::Error;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Element {
    #[default]
    Anemo,
    Cryo,
    Dendro,
    Electro,
    Geo,
    Hydro,
    Pyro,
    Physical,
}

impl ToSql for Element {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut bytes::BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        let s = match self {
            Element::Anemo => "Anemo",
            Element::Cryo => "Cryo",
            Element::Dendro => "Dendro",
            Element::Electro => "Electro",
            Element::Geo => "Geo",
            Element::Hydro => "Hydro",
            Element::Pyro => "Pyro",
            Element::Physical => "Physical",
        };
        out.extend_from_slice(s.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "element"
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for Element {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = std::str::from_utf8(raw)?;
        dbg!(s);
        match s {
            "Anemo" => Ok(Element::Anemo),
            "Cryo" => Ok(Element::Cryo),
            "Dendro" => Ok(Element::Dendro),
            "Electro" => Ok(Element::Electro),
            "Geo" => Ok(Element::Geo),
            "Hydro" => Ok(Element::Hydro),
            "Pyro" => Ok(Element::Pyro),
            "Physical" => Ok(Element::Physical),
            _ => Err(format!("Unknown element: {}", s).into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "element"
    }
}
