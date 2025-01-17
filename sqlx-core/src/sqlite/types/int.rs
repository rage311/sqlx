use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::sqlite::type_info::DataType;
use crate::sqlite::{Sqlite, SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef};
use crate::types::Type;

impl Type<Sqlite> for i32 {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Int)
    }
}

impl<'q> Encode<'q, Sqlite> for i32 {
    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
        args.push(SqliteArgumentValue::Int(*self));

        IsNull::No
    }
}

impl<'r> Decode<'r, Sqlite> for i32 {
    fn accepts(_ty: &SqliteTypeInfo) -> bool {
        true
    }

    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.int())
    }
}

impl Type<Sqlite> for i64 {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Int64)
    }
}

impl<'q> Encode<'q, Sqlite> for i64 {
    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
        args.push(SqliteArgumentValue::Int64(*self));

        IsNull::No
    }
}

impl<'r> Decode<'r, Sqlite> for i64 {
    fn accepts(_ty: &SqliteTypeInfo) -> bool {
        true
    }

    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.int64())
    }
}
