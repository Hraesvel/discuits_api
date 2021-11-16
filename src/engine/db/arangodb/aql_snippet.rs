pub(crate) const GET: &str = r#"RETURN DOCUMENT(CONCAT(@collection, "/", @id))"#;

pub(crate) const GET_ALL: &str = "FOR doc IN @@collection \
                                  RETURN doc";

pub(crate) const FILTER: &str = "FOR doc IN @@collection \
                                 FILTER doc.@k == @v \
                                 RETURN doc";

pub(crate) const INSERT: &str = "INSERT @doc INTO @@collection \
                                OPTIONS {overwrite: false} \
                                RETURN NEW";

pub(crate) const UPSERT_EDGE: &str = "UPSERT( {_from: @doc._from, _to: @doc._to} ) \
                                INSERT(@doc) \
                                UPDATE({}) in @@collection \
                                return NEW";

pub(crate) const UPSERT: &str = "UPSERT( _key: @key ) \
                                INSERT(@doc) \
                                UPDATE(@doc) in @@collection \
                                return NEW";

pub(crate) const REMOVE: &str = "REMOVE @key IN @@collection RETURN OLD";
