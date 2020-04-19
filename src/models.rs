use super::schema::fonts;

#[derive(Queryable)]
pub struct Fonts {
    pub id: i32,
    pub name: String,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Insertable)]
#[table_name="fonts"]
pub struct NewFonts<'a> {
    pub name: &'a str,
    pub created_at: i32,
    pub updated_at: i32,
}