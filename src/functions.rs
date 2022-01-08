//! Declarations of Postgres specific SQL functions
use diesel::sql_types::{Integer, Text};

sql_function!(
    /// See: [strpos()](https://www.postgresql.org/docs/9.1/functions-string.html)
    fn strpos (string: Text, substring: Text) -> Integer
);

sql_function!(
    /// See: [lower()](https://www.postgresql.org/docs/9.1/functions-string.html)
    fn lower (string: Text) -> Text
);
