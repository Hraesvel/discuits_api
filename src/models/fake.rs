use std::borrow::Cow;

use crate::macros::concon;
use crate::time::TimeStamp;

#[concon(_)]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Fake {
    id: Cow<'static, str>,
    key: Cow<'static, str>,
}

impl Fake {
    pub fn new()-> Self {
        Self::default()
    }
}


mod test {
    use crate::models::fake::Fake;

    #[test]
    fn test_attr() {
        let _f = Fake::default();

        // dbg!(f.get_time_created());
    }
}
