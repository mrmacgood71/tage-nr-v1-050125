use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{alphanumeric1, space1};
use nom::multi::separated_list1;
use nom::{bytes::complete::tag, IResult};
use nom::error::Error;

#[derive(Debug, PartialEq)]
struct Query{
    select: Option<SelectQuery>,
    insert_query: Option<InsertQuery>
}
#[derive(Debug, PartialEq)]
struct SelectQuery {
    table: String,
    columns: Vec<String>
}
#[derive(Debug, PartialEq)]
struct InsertQuery {
    table: String,
    columns: Vec<String>,
    values: Vec<String>
}

// cmu database group (YT, Andy Pavlo)- inspiration what can I to solve
fn main() {
    println!("Hello, world!");
}

fn parse(input: &str) -> IResult<&str, Query> {
    alt((
        parse_action_select,
        parse_action_insert
    ))(input)
}

fn parse_action_select(input: &str) -> IResult<&str, Query> {
    let (input, _) = tag("select")(input)?;
    let (input, _) = space1(input)?;
    let (input, columns) = parse_select_columns(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("from")(input)?;
    let (input, _) = space1(input)?;
    let (input, table) = alphanumeric1(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((input,
        Query {
            select: Some(SelectQuery {
                table: table.to_string(),
                columns: columns.iter().map(ToString::to_string).collect()
            }),
            insert_query: None
        }
    ))
}

fn parse_action_insert(input: &str) -> IResult<&str, Query> {
    let (input, _) = tag("insert")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("into")(input)?;
    let (input, _) = space1(input)?;
    let (input, table) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, columns) = parse_select_columns(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("values")(input)?;
    let (input, _) = space1(input)?;
    let (input, values) = parse_select_columns(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        Query {
            select: None,
            insert_query: Some(InsertQuery {
                table: table.to_string(),
                columns: columns.iter().map(ToString::to_string).collect(),
                values: values.iter().map(ToString::to_string).collect()
            })
        }
    ))
}

fn parse_select(input: &str) -> IResult<&str, SelectQuery> {
    let (remainder, parsed) = tag("select")(input)?;
    Ok((remainder, SelectQuery { table: "".to_string(), columns: vec![] }))
}



fn parse_select_columns(input: &str) -> IResult<&str, Vec<&str>, Error<&str>> {

    separated_list1(tag(", "), alphanumeric1)(input)
}


fn parse_action(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("select"),
        tag_no_case("insert"),
        tag_no_case("update"),
        tag_no_case("delete"),
        tag_no_case("create"),
        tag_no_case("drop")
    ))(input)
}

fn parse_nothing(input: &str) -> IResult<&str, &str> {
    tag("select")(input)
}

fn parse_cols_with_nums(input: &str) -> IResult<&str, &str> {
    tag("select")(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nothing() {
        let (left, output) = parse_nothing("select * from table").unwrap();
        assert_eq!(left, " * from table");
        assert_eq!(output, "select");
    }

    #[test]
    fn test_parse_select() {
        let (left, output) = parse("select col1, col2 from table1;").unwrap();

        assert_eq!(left, "");
        let query = output.select.unwrap();
        assert_eq!(query.table, "table1");
        assert_eq!(query.columns, vec!["col1", "col2"]);
    }


    // select * from t1;
    #[test]
    fn test_parse_action() {
        let (left, output) = parse_action("SELECT * from table").unwrap();
        assert_eq!(left, " * from table");
        // assert_eq!(output, "select");
    }

    #[test]
    fn test_parse_action_insert() {
        let (left, output) = parse_action("insert INTO table").unwrap();
        assert_eq!(left, " INTO table");
        assert_eq!(output, "insert");
    }
    #[test]
    fn test_parse_action_update() {
        let (left, output) = parse_action("update table").unwrap();
        assert_eq!(left, " table");
        assert_eq!(output, "update");
    }

    #[test]
    fn test_parse_action_delete() {
        let (left, output) = parse_action("delete from table").unwrap();
        assert_eq!(left, " from table");
        assert_eq!(output, "delete");
    }

    #[test]
    fn test_parse_action_create() {
        let (left, output) = parse_action("create table").unwrap();
        assert_eq!(left, " table");
        assert_eq!(output, "create");
    }

    #[test]
    fn test_parse_action_select() {
        let (left, output) = parse_action("select * from t1").unwrap();
        assert_eq!(left, " * from t1");
        assert_eq!(output, "select");
    }
}
