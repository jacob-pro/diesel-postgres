use diesel::pg::Pg;
use diesel::query_builder::{AsQuery, AstPass, Query, QueryFragment};
use diesel::query_dsl::LoadQuery;
use diesel::sql_types::BigInt;
use diesel::{PgConnection, QueryResult, RunQueryDsl};

// https://diesel.rs/guides/extending-diesel/

#[derive(QueryId)]
pub struct CountedLimit<T> {
    query: T,
    limit: i64,
    offset: i64,
}

impl<T> QueryFragment<Pg> for CountedLimit<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") AS x LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.limit)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}

impl<T: Query> Query for CountedLimit<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for CountedLimit<T> {}

impl<T> CountedLimit<T> {
    pub fn offset(self, offset: i64) -> Self {
        CountedLimit { offset, ..self }
    }

    pub fn load_with_total<U>(self, conn: &PgConnection) -> QueryResult<CountedLimitResult<U>>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let db_result = self.load::<(U, i64)>(conn)?;
        let total = db_result
            .get(0)
            .map(|(_, total)| total.to_owned())
            .unwrap_or(0);
        let results = db_result.into_iter().map(|(record, _)| record).collect();
        Ok(CountedLimitResult {
            results,
            count: total,
        })
    }
}

pub trait CountingLimit: AsQuery + Sized {
    fn counted_limit(self, limit: i64) -> CountedLimit<Self::Query> {
        CountedLimit {
            query: self.as_query(),
            limit,
            offset: 0,
        }
    }
}

impl<T: AsQuery> CountingLimit for T {}

#[derive(Debug)]
pub struct CountedLimitResult<T> {
    pub results: Vec<T>,
    pub count: i64,
}
