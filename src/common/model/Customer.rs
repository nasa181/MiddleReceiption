struct Customer<'a>{
    id: i32,
    name: &'a str
}