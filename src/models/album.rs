use std::borrow::Cow;

use crate::io::{read::Get, write::Write};

struct Album<'a> {
    id: &'static str,
    barcode: Cow<'a, str>,
    cat_no: Cow<'a, str>,
    name: Cow<'a, str>,
    description: Cow<'a, str>,
}

impl<'a> Get for Album<'a> {
    type Data = std::io::Result<()>;

    fn get(id: Cow<'static, str>) -> Self::Data {
        Ok(())
    }
}

