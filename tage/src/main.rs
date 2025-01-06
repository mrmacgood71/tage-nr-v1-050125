use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{alpha1, alphanumeric1, i32, space1};
use nom::error::Error;
use nom::multi::separated_list1;
use nom::{bytes::complete::tag, IResult};

#[derive(Debug, PartialEq)]
struct Query {
    select: Option<SelectQuery>,
    insert: Option<InsertQuery>,
}
#[derive(Debug, PartialEq)]
struct SelectQuery {
    table: String,
    columns: Vec<String>,
}
#[derive(Debug, PartialEq)]
struct InsertQuery {
    table: String,
    columns: Vec<String>,
    values: Vec<Value>,
}

#[derive(Debug, PartialEq)]
struct Value {
    i_value: Option<i32>,
    s_value: Option<String>,
}

// cmu database group (YT, Andy Pavlo)- inspiration what can I to solve
// + Tony saro
fn main() {
    println!("Hello, world!");
}

fn parse(input: &str) -> IResult<&str, Query> {
    alt((parse_select, parse_insert))(input)
}

fn parse_select(input: &str) -> IResult<&str, Query> {
    let (input, _) = tag("select")(input)?;
    let (input, _) = space1(input)?;
    let (input, columns) = parse_columns(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("from")(input)?;
    let (input, _) = space1(input)?;
    let (input, table) = alphanumeric1(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        Query {
            select: Some(SelectQuery {
                table: table.to_string(),
                columns: columns.iter().map(ToString::to_string).collect(),
            }),
            insert: None,
        },
    ))
}

fn parse_insert(input: &str) -> IResult<&str, Query> {
    let (input, _) = tag("insert")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("into")(input)?;
    let (input, _) = space1(input)?;
    let (input, table) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, columns) = parse_columns(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("values")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, values) = parse_values(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        Query {
            select: None,
            insert: Some(InsertQuery {
                table: table.to_string(),
                columns: columns.iter().map(ToString::to_string).collect(),
                values,
            }),
        },
    ))
}

fn parse_columns(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alphanumeric1)(input)
}

fn parse_values(input: &str) -> IResult<&str, Vec<Value>> {
    separated_list1(tag(", "), parse_value)(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let res = i32(input);
    if res.is_ok() {
        return res.map(|(input, i_value)| {
            (
                input,
                Value {
                    i_value: Some(i_value),
                    s_value: None,
                },
            )
        });
    }
    let (input, _) = tag("'")(input)?;
    let (input, s) = alphanumeric1(input)?;
    let (input, _) = tag("'")(input)?;
    Ok((
        input,
        Value {
            i_value: None,
            s_value: Some(s.to_string()),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_select() {
        let (left, output) = parse("select col1, col2 from table1;").unwrap();

        assert_eq!(left, "");
        let query = output.select.unwrap();
        assert_eq!(query.table, "table1");
        assert_eq!(query.columns, vec!["col1", "col2"]);
    }

    #[test]
    fn test_parse_insert() {
        let (left, output) = parse("insert into table1 (col1, col2) values (1, 'value');").unwrap();

        assert_eq!(left, "");
        let query = output.insert.unwrap();
        assert_eq!(query.table, "table1");
        assert_eq!(query.columns, vec!["col1", "col2"]);
        assert_eq!(
            query.values,
            vec![
                Value {
                    i_value: Some(1),
                    s_value: None
                },
                Value {
                    i_value: None,
                    s_value: Some("value".to_string())
                }
            ]
        );
    }

    #[test]
    fn select_multiple_columns() {
        let (remainder, select_query) =
            parse("select col1, col2, col3, col4, col5 from table1;").unwrap();
        assert_eq!(remainder, "");
        let query = select_query.select.unwrap();
        assert_eq!(query.table, "table1");
        assert_eq!(query.columns, vec!["col1", "col2", "col3", "col4", "col5"]);
    }

    #[test]
    fn insert_multiple_columns() {
        let (remainder, insert_query) = parse("insert into table1 (col1, col2, col3, col4, col5) values (1, 'value', 2, 'value2', 3);").unwrap();
        let query = insert_query.insert.unwrap();

        assert_eq!(remainder, "");
        assert_eq!(query.table, "table1");
        assert_eq!(query.columns, vec!["col1", "col2", "col3", "col4", "col5"]);
        assert_eq!(
            query.values,
            vec![
                Value {
                    i_value: Some(1),
                    s_value: None,
                },
                Value {
                    i_value: None,
                    s_value: Some("value".to_string()),
                },
                Value {
                    i_value: Some(2),
                    s_value: None,
                },
                Value {
                    i_value: None,
                    s_value: Some("value2".to_string()),
                },
                Value {
                    i_value: Some(3),
                    s_value: None,
                },
            ]
        );
    }

    #[test]
    fn select_single_column() {
        let (remainder, query) = parse("select col1 from table1;").unwrap();
        assert_eq!(remainder, "");
        assert_eq!(
            query,
            Query {
                select: Some(SelectQuery {
                    table: "table1".to_string(),
                    columns: vec!["col1".to_string()]
                }),
                insert: None
            }
        );
    }

    #[test]
    fn insert_single_column() {
        let (remainder, insert_query) = parse("insert into table1 (col1) values (1);").unwrap();
        assert_eq!(remainder, "");
        assert_eq!(
            insert_query,
            Query {
                select: None,
                insert: Some(InsertQuery {
                    table: "table1".to_string(),
                    columns: vec!["col1".to_string()],
                    values: vec![Value {
                        i_value: Some(1),
                        s_value: None,
                    }],
                }),
            }
        );
    }
}
