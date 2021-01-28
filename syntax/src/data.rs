#[derive(Debug, Clone)]
pub struct File<'a> {
    pub items: Vec<Item<'a>>,
}

#[derive(Debug, Clone)]
pub enum Item<'a> {
    Import(ImportItem<'a>),
    Func(FuncItem<'a>),
}

#[derive(Debug, Clone)]
pub struct Attribute<'a> {
    pub name: &'a str,
}

#[derive(Debug, Clone)]
pub struct ImportItem<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub items: Vec<Item<'a>>,
}

#[derive(Debug, Clone)]
pub struct FuncItem<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub name: &'a str,
    pub return_ty: &'a str,
    pub body: Option<Vec<Expression<'a>>>,
}

#[derive(Debug, Clone)]
pub struct Parameter<'a> {
    pub name: &'a str,
    pub ty: &'a str,
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Number(u8),
    Call {
        leading: Option<Box<Expression<'a>>>,
        name: &'a str,
        arguments: Vec<Expression<'a>>,
    },
}
