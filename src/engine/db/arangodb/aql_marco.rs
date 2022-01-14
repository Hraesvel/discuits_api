macro_rules! filter {
    ( $ele:tt => $field:tt == $tar:tt ) => {
        if let Ok(b) = $tar.to_lowercase().parse::<bool>() {
            format!("FILTER {}.{} == {} \n", $ele, $field, b)
        } else {
            format!("FILTER {}.{} == {} \n", $ele, $field, $tar)
        }
    };
}

macro_rules! sort {
    ($ele:tt => $($field:tt),+ ASC ) => {{
        let mut exp = String::new();
        $(
            exp.push_str(&format!("{}.{}, ", $ele, $field));
        )+
        let (s, _) = exp.as_str().rsplit_once(',').unwrap();

        format!("SORT {} ASC\n", s)
    }};
    ($ele:tt => $($field:tt),+ DESC ) => {{
        let mut exp = String::new();
        $(
            exp.push_str(&format!("{}.{}, ", $ele, $field));
        )+
        let (s, _) = exp.as_str().rsplit_once(',').unwrap();
        format!("SORT {} DESC\n", s)
    }};
}

macro_rules! limit {
    ($count:expr) => {
        format!("LIMIT {}\n", $count)
    };
    ($offset:expr, $count:expr) => {
        format!("LIMIT {}, {}\n", $offset, $count)
    };
}

macro_rules! aql_ops {
    (limit : $($n:expr),+ ) => {
        limit!($($n),+)
    };
    (filter : $ele:tt => $field:tt == $tar:tt) => {
        filter!($ele => $field == $tar)
    }
}

macro_rules! aql_builder {
    (FOR $ele:tt IN $col:tt => $($fun:expr),* ) => {{
        let mut statement = format!("FOR {} in {}\n", $ele, $col);
        $(
            statement.push_str(&($fun));
        )*
        statement.push_str(&format!("RETURN {}", $ele));
        statement
        }};
}

#[cfg(test)]
mod test {

    #[test]
    fn test_aql_marco() {
        let m: String = aql_builder!(FOR "N" IN "COL" =>
            filter!("N" => "name" == "jim"),
            limit!(2, 5),
            sort!("N" => "name", "age" DESC)
        );
        println!("{}", m);
    }
}
