fn main() {
    let _query = r#"--sql
select * from book
"#;

let _query2 = r#"--sql
select * from book;
"#;

    let _another = r#"--sql; select * from book;"#;
}
