pub mod lexer;

// use crate::parser::Query::{Insert, Select};
// use nom::branch::alt;
// use nom::character::complete::{alpha1, alphanumeric1, i32, space1};
// use nom::combinator::{map, opt};
// use nom::multi::separated_list1;
// use nom::sequence::terminated;
// use nom::{bytes::complete::tag, IResult};
//
// #[derive(Debug, PartialEq)]
// struct Queries {
//     queries: Vec<Query>,
// }
//
// #[derive(Debug, PartialEq)]
// enum Query {
//     Select(SelectQuery),
//     Insert(InsertQuery),
//     Update(UpdateQuery),
//     Delete(DeleteQuery),
//     CreateTable(CreateTableQuery),
//     DropTable(DropTableQuery),
//     AlterTable(AlterTableQuery),
// }
//
// /// *SelectQuery* is a struct that represents a select query
// /// select <select_statement>
// /// from <from_statement>
// /// where <where_statement>
// /// group by <group_by_statement>
// /// having <having_statement>
// /// order by <order_by_statement>
// /// limit <limit_statement>
// #[derive(Debug, PartialEq)]
// struct SelectQuery {
//     select_statement: SelectStatement,
//     from_statement: FromStatement,
//     where_statement: Option<WhereStatement>,
//     group_by_statement: Option<GroupByStatement>,
//     having_statement: Option<HavingStatement>,
//     order_by_statement: Option<OrderByStatement>,
//     limit_statement: Option<LimitStatement>,
// }
//
// /// *InsertQuery* is a struct that represents an insert query
// /// insert into <table_name>
// /// (<columns>)
// /// values (<values>)
// #[derive(Debug, PartialEq)]
// struct InsertQuery {
//     table_name: String,
//     columns: Vec<String>,
//     values: Vec<String>,
// }
//
// /// *UpdateQuery* is a struct that represents an update query
// /// update <table_name>
// /// set <set_statement>
// /// where <where_statement>
// #[derive(Debug, PartialEq)]
// struct UpdateQuery {
//     table_name: String,
//     set_statement: SetStatement,
//     where_statement: Option<WhereStatement>,
// }
//
// #[derive(Debug, PartialEq)]
// struct SetStatement {
//     columns: Vec<String>,
// }
//
// /// *CreateTableQuery* is a struct that represents a create table query
// /// create table <table_name>
// /// (<columns_definitions>)
// /// <constraints>
// #[derive(Debug, PartialEq)]
// struct CreateTableQuery {
//     table_name: String,
//     // todo: columns_definitions and constraints as struct
//     columns_definitions: Vec<String>,
//     constraints: Vec<String>,
// }
//
// /// *DeleteStatement* is a struct that represents a drop table query
// /// delete from <table_name>
// /// where <where_statement>
// #[derive(Debug, PartialEq)]
// struct DeleteQuery {
//     table_name: String,
//     where_statement: WhereStatement,
// }
//
// /// *DropTableStatement* is a struct that represents a drop table query
// /// drop table <table_name>
// #[derive(Debug, PartialEq)]
// struct DropTableQuery {
//     table_name: String,
// }
//
// /// *AlterTableStatement* is a struct that represents an alter table query
// /// alter table <table_name>
// /// <alter_table_statement>
// #[derive(Debug, PartialEq)]
// struct AlterTableQuery {
//     table_name: String,
//     // todo: alter_table_statement actions
//     alter_table_statement: String,
// }
//
// #[derive(Debug, PartialEq)]
// struct SelectStatement {
//     columns: Vec<ColumnStatement>,
//     distinct: bool,
// }
// #[derive(Debug, PartialEq)]
// struct FromStatement {
//     tables: Vec<TableStatement>,
//     joins: Vec<JoinStatement>,
// }
// #[derive(Debug, PartialEq)]
// struct WhereStatement {
//     conditions: Vec<Condition>,
// }
// #[derive(Debug, PartialEq)]
// struct OrderByStatement {
//     columns: Vec<ColumnStatement>,
//     order: Order,
// }
// #[derive(Debug, PartialEq)]
// enum Order {
//     Asc,
//     Desc,
// }
// #[derive(Debug, PartialEq)]
// struct LimitStatement {
//     limit: i32,
//     offset: i32,
// }
// #[derive(Debug, PartialEq)]
// struct HavingStatement {
//     conditions: Vec<Condition>,
// }
// #[derive(Debug, PartialEq)]
// struct GroupByStatement {
//     columns: Vec<ColumnStatement>,
// }
//
// #[derive(Debug, PartialEq)]
// enum ColumnStatement {
//     ColumnStatement(ColumnIdentifier),
//     ColumnStatementFunction(Function),
//     ColumnStatementLiteral(Literal),
// }
//
// #[derive(Debug, PartialEq)]
// struct ColumnIdentifier {
//     table_name: Option<String>,
//     column_name: String,
// }
//
// #[derive(Debug, PartialEq)]
// struct Function {
//     name: String,
//     arguments: Vec<ColumnStatement>,
// }
//
// #[derive(Debug, PartialEq)]
// enum Literal {
//     Integer(i32),
//     String(String),
//     Boolean(bool),
//     Float(f32),
// }
//
// #[derive(Debug, PartialEq)]
// struct ColumnFunction {
//     function: String,
//     column: ColumnIdentifier,
// }
//
// #[derive(Debug, PartialEq)]
// struct TableStatement {
//     table_name: String,
//     alias: Option<String>,
// }
//
// #[derive(Debug, PartialEq)]
// struct JoinStatement {
//     table: TableStatement,
//     // todo: join type as enum
//     join_type: String,
//     on: Vec<Condition>,
// }
//
// #[derive(Debug, PartialEq)]
// struct Condition {
//     left: ColumnStatement,
//     operator: Operator,
//     right: ColumnStatement,
// }
//
// #[derive(Debug, PartialEq)]
// enum Operator {
//     Equal,
//     NotEqual, // Todo: add more operators
//               // GreaterThan,
//               // LessThan,
//               // GreaterThanOrEqual,
//               // LessThanOrEqual,
//               // Like,
//               // In,
//               // NotIn,
//               // Between,
//               // NotBetween,
//               // IsNull,
//               // IsNotNull,
//               // And,
//               // Or
// }
//
// impl Queries {
//     fn parse(input: &str) -> IResult<&str, Queries> {
//         let (input, queries) = separated_list1(tag(";"), Query::parse)(input)?;
//
//         Ok((input, Queries { queries }))
//     }
// }
//
// impl Query {
//     fn parse(input: &str) -> IResult<&str, Query> {
//         alt((
//             map(SelectQuery::parse, Select),
//             map(InsertQuery::parse, Insert),
//         ))(input)
//     }
// }
//
// impl SelectQuery {
//     fn parse(input: &str) -> IResult<&str, SelectQuery> {
//         let (input, _) = tag("select")(input)?;
//         let (input, _) = space1(input)?;
//         let (input, select_statement) = SelectStatement::parse(input)?;
//         let (input, _) = space1(input)?;
//         let (input, _) = tag("from")(input)?;
//         let (input, _) = space1(input)?;
//         let (input, from_statement) = FromStatement::parse(input)?;
//         let (input, _) = tag(";")(input)?;
//
//         // TODO: Implement the rest of the parsing
//         // let (input, from_statement) = FromStatement::parse(input)?;
//         // let (input, where_statement) = WhereStatement::parse(input)?;
//         // let (input, group_by_statement) = GroupByStatement::parse(input)?;
//         // let (input, having_statement) = HavingStatement::parse(input)?;
//         // let (input, order_by_statement) = OrderByStatement::parse(input)?;
//         // let (input, limit_statement) = LimitStatement::parse(input)?;
//
//         Ok((
//             input,
//             SelectQuery {
//                 select_statement,
//                 from_statement,
//                 where_statement: None,
//                 group_by_statement: None,
//                 having_statement: None,
//                 order_by_statement: None,
//                 limit_statement: None,
//             },
//         ))
//     }
// }
//
// impl SelectStatement {
//     fn parse(input: &str) -> IResult<&str, SelectStatement> {
//         let (input, columns) = separated_list1(tag(", "), ColumnStatement::parse)(input)?;
//
//         Ok((
//             input,
//             SelectStatement {
//                 columns,
//                 distinct: false,
//             },
//         ))
//     }
// }
// impl Query {
//     fn parse(input: &str) -> IResult<&str, Query> {
//         alt((
//             map(SelectQuery::parse, Select),
//             map(InsertQuery::parse, Insert),
//         ))(input)
//     }
// }
//
// impl ColumnStatement {
//     fn parse(input: &str) -> IResult<&str, ColumnStatement> {
//         alt((
//             map(ColumnIdentifier::parse, ColumnStatement::ColumnStatement),
//             map(Function::parse, ColumnStatement::ColumnStatementFunction),
//             map(Literal::parse, ColumnStatement::ColumnStatementLiteral),
//         ))(input)
//     }
// }
//
// impl ColumnIdentifier {
//     fn parse(input: &str) -> IResult<&str, ColumnIdentifier> {
//         let (input, table_name) = opt(terminated(alphanumeric1, tag(".")))(input)?;
//         let (input, column_name) = alphanumeric1(input)?;
//
//         Ok((
//             input,
//             ColumnIdentifier {
//                 table_name: table_name.map(|s| s.to_string()),
//                 column_name: column_name.to_string(),
//             },
//         ))
//     }
// }
//
// impl Function {
//     fn parse(input: &str) -> IResult<&str, Function> {
//         let (input, name) = alphanumeric1(input)?;
//         let (input, _) = tag("(")(input)?;
//         let (input, arguments) = separated_list1(tag(", "), ColumnStatement::parse)(input)?;
//         let (input, _) = tag(")")(input)?;
//
//         Ok((
//             input,
//             Function {
//                 name: name.to_string(),
//                 arguments,
//             },
//         ))
//     }
// }
//
// impl Literal {
//     fn parse(input: &str) -> IResult<&str, Literal> {
//         let res = i32(input);
//         if res.is_ok() {
//             return res.map(|(input, i_value)| (input, Literal::Integer(i_value)));
//         }
//         let (input, _) = tag("'")(input)?;
//         let (input, s) = alpha1(input)?;
//         let (input, _) = tag("'")(input)?;
//         Ok((input, Literal::String(s.to_string())))
//     }
// }
//
// impl FromStatement {
//     fn parse(input: &str) -> IResult<&str, FromStatement> {
//         let (input, table) = TableStatement::parse(input)?;
//         // let (input, _) = space1(input)?;
//         // let (input, joins) = separated_list1(tag(" "), JoinStatement::parse)(input)?;
//         Ok((
//             input,
//             FromStatement {
//                 tables: vec![table],
//                 // TODO: Implement joins
//                 joins: vec![],
//             },
//         ))
//     }
// }
//
// impl TableStatement {
//     fn parse(input: &str) -> IResult<&str, TableStatement> {
//         let (input, table_name) = alphanumeric1(input)?;
//         Ok((
//             input,
//             TableStatement {
//                 table_name: table_name.to_string(),
//                 // TODO: Implement alias
//                 alias: None,
//             },
//         ))
//     }
// }
//
// impl InsertQuery {
//     fn parse(input: &str) -> IResult<&str, InsertQuery> {
//         let (input, _) = tag("insert")(input)?;
//         let (input, _) = space1(input)?;
//         let (input, _) = tag("into")(input)?;
//         let (input, _) = space1(input)?;
//         let (input, table_name) = alphanumeric1(input)?;
//         let (input, _) = space1(input)?;
//         let (input, _) = tag("(")(input)?;
//         let (input, columns) = Self::parse_column_list(input)?;
//         let (input, _) = tag(")")(input)?;
//         let (input, _) = space1(input)?;
//         let (input, _) = tag("values")(input)?;
//         let (input, _) = space1(input)?;
//         let (input, _) = tag("(")(input)?;
//         let (input, values) = Self::parse_column_list(input)?;
//         let (input, _) = tag(")")(input)?;
//         let (input, _) = tag(";")(input)?;
//
//         Ok((
//             input,
//             InsertQuery {
//                 table_name: table_name.to_string(),
//                 columns: columns.iter().map(ToString::to_string).collect(),
//                 values: values.iter().map(ToString::to_string).collect(),
//             },
//         ))
//     }
//     fn parse_column_list(input: &str) -> IResult<&str, Vec<&str>> {
//         separated_list1(tag(", "), alphanumeric1)(input)
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_parse_select_in_queries_with_table_and_column() {
//         let (input, queries) = Queries::parse("select t1.col1, t1.col2 from t1;").unwrap();
//
//         assert!(input.is_empty());
//         assert_eq!(
//             queries,
//             Queries {
//                 queries: vec![Select(SelectQuery {
//                     select_statement: SelectStatement {
//                         columns: vec![
//                             ColumnStatement::ColumnStatement(ColumnIdentifier {
//                                 table_name: Some("t1".to_string()),
//                                 column_name: "col1".to_string()
//                             }),
//                             ColumnStatement::ColumnStatement(ColumnIdentifier {
//                                 table_name: Some("t1".to_string()),
//                                 column_name: "col2".to_string()
//                             })
//                         ],
//                         distinct: false
//                     },
//                     from_statement: FromStatement {
//                         tables: vec![TableStatement {
//                             table_name: "t1".to_string(),
//                             alias: None
//                         }],
//                         joins: vec![]
//                     },
//                     where_statement: None,
//                     group_by_statement: None,
//                     having_statement: None,
//                     order_by_statement: None,
//                     limit_statement: None
//                 })]
//             }
//         );
//     }
//
//     #[test]
//     fn test_parse_select_in_queries() {
//         let (input, queries) = Queries::parse("select col1, col2 from t1;").unwrap();
//
//         assert_eq!(input, "");
//         assert_eq!(
//             queries,
//             Queries {
//                 queries: vec![Query::Select(SelectQuery {
//                     select_statement: SelectStatement {
//                         columns: vec![
//                             ColumnStatement::ColumnStatement(ColumnIdentifier {
//                                 table_name: None,
//                                 column_name: "col1".to_string()
//                             }),
//                             ColumnStatement::ColumnStatement(ColumnIdentifier {
//                                 table_name: None,
//                                 column_name: "col2".to_string()
//                             })
//                         ],
//                         distinct: false
//                     },
//                     from_statement: FromStatement {
//                         tables: vec![TableStatement {
//                             table_name: "t1".to_string(),
//                             alias: None
//                         }],
//                         joins: vec![]
//                     },
//                     where_statement: None,
//                     group_by_statement: None,
//                     having_statement: None,
//                     order_by_statement: None,
//                     limit_statement: None
//                 })]
//             }
//         );
//     }
//
//     #[test]
//     fn test_parse_select_query() {
//         let input = "select column1, column2 from table1;";
//         let result = SelectQuery::parse(input);
//         assert_eq!(
//             result,
//             Ok((
//                 "",
//                 SelectQuery {
//                     select_statement: SelectStatement {
//                         columns: vec![
//                             ColumnStatement::ColumnStatement(ColumnIdentifier {
//                                 table_name: None,
//                                 column_name: "column1".to_string()
//                             }),
//                             ColumnStatement::ColumnStatement(ColumnIdentifier {
//                                 table_name: None,
//                                 column_name: "column2".to_string()
//                             })
//                         ],
//                         distinct: false
//                     },
//                     from_statement: FromStatement {
//                         tables: vec![TableStatement {
//                             table_name: "table1".to_string(),
//                             alias: None
//                         }],
//                         joins: vec![]
//                     },
//                     where_statement: None,
//                     group_by_statement: None,
//                     having_statement: None,
//                     order_by_statement: None,
//                     limit_statement: None
//                 }
//             ))
//         );
//     }
//
//     #[test]
//     fn insert_single_column() {
//         let (remainder, insert_query) =
//             Queries::parse("insert into table1 (col1) values (1);").unwrap();
//         assert_eq!(remainder, "");
//         assert_eq!(
//             insert_query,
//             Queries {
//                 queries: vec![Query::Insert(InsertQuery {
//                     table_name: "table1".to_string(),
//                     columns: vec!["col1".to_string()],
//                     values: vec!["1".to_string()]
//                 })]
//             }
//         );
//     }
// }
