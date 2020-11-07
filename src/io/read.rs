use std::borrow::Cow;

pub trait Get {
    type Data;

    fn get(id: Cow<'static, str>) -> Self::Data;
}
