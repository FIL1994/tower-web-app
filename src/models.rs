use super::schema::expenses;

#[derive(Debug, Queryable, Response, Extract, Serialize)]
pub struct Expense {
    pub id: i32,
    pub name: String,
    pub amount: f32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "expenses"]
pub struct NewExpense<'a> {
    pub name: &'a str,
    pub amount: f32,
}
