use std::io::Write;

use diesel::{
    backend::Backend,
    deserialize::{FromSql, Result as DeResult},
    expression::AsExpression,
    serialize::{Output, Result as SerResult, ToSql},
};

use {MemberID, Tag};

macro_rules! newtype_impls {
    ($t:ident, $inner:ty) => {
        impl<T> AsExpression<T> for $t
        where
            $inner: AsExpression<T>,
        {
            type Expression = <$inner as AsExpression<T>>::Expression;
            fn as_expression(self) -> Self::Expression {
                self.0.as_expression()
            }
        }

        impl<A, DB> FromSql<A, DB> for $t
        where
            $inner: FromSql<A, DB>,
            DB: Backend,
        {
            fn from_sql(bytes: Option<&DB::RawValue>) -> DeResult<Self> {
                FromSql::from_sql(bytes).map($t)
            }
        }

        impl<A, DB> ToSql<A, DB> for $t
        where
            $inner: ToSql<A, DB>,
            DB: Backend,
        {
            fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> SerResult {
                self.0.to_sql(out)
            }
        }
    };
}

newtype_impls!(MemberID, u32);
newtype_impls!(Tag, String);
