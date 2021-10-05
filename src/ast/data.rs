#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Enum(EnumItem),
    Func(FuncItem),
}

impl From<EnumItem> for Item {
    fn from(it: EnumItem) -> Self {
        Self::Enum(it)
    }
}

impl From<FuncItem> for Item {
    fn from(it: FuncItem) -> Self {
        Self::Func(it)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumItem {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncItem {
    pub name: String,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Variable(String),
    EnumLiteral(EnumLiteral),
    LetExpr(Box<LetExpr>),
}

impl From<EnumLiteral> for Expr {
    fn from(it: EnumLiteral) -> Self {
        Self::EnumLiteral(it)
    }
}

impl From<LetExpr> for Expr {
    fn from(it: LetExpr) -> Self {
        Self::LetExpr(Box::new(it))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumLiteral {
    pub name: String,
    pub variant: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetExpr {
    pub name: String,
    pub expr: Expr,
    pub body: Expr,
}
