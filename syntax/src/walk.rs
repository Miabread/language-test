use {
    super::{data::*, ParseError::*, Result},
    std::mem,
    tree_sitter::{Node, TreeCursor},
};

pub fn file<'a>(cursor: TreeCursor, source: &'a [u8]) -> Result<'a, File<'a>> {
    Ok(File {
        items: items(cursor, source)?,
    })
}

pub fn items<'a>(mut cursor: TreeCursor, source: &'a [u8]) -> Result<'a, Vec<Item<'a>>> {
    let mut attributes = Vec::new();
    let mut items = Vec::new();

    cursor.goto_first_child();

    loop {
        let node = cursor.node();

        if node.kind() == "attribute" {
            let name = node
                .child_by_field_name("name")
                .expect("attribute name")
                .utf8_text(source)?;
            attributes.push(Attribute { name });
        } else {
            let item = item(node.walk(), source, mem::take(&mut attributes))?;
            items.push(item);
        }

        if cursor.goto_next_sibling() {
            break;
        }
    }

    Ok(items)
}

pub fn item<'a>(
    mut cursor: TreeCursor,
    source: &'a [u8],
    attributes: Vec<Attribute<'a>>,
) -> Result<'a, Item<'a>> {
    let node = cursor.node();
    Ok(match node.kind() {
        "import_item" => unimplemented!(),
        "func_item" => Item::Func(func_item(node, source, attributes)?),
        found => return Err(UnknownItem { found }),
    })
}

pub fn func_item<'a>(
    node: Node,
    source: &'a [u8],
    attributes: Vec<Attribute<'a>>,
) -> Result<'a, FuncItem<'a>> {
    let name = node
        .child_by_field_name("name")
        .expect("func name")
        .utf8_text(source)?;

    let return_ty = node
        .child_by_field_name("name")
        .expect("func return ty")
        .utf8_text(source)?;

    Ok(FuncItem {
        attributes,
        name,
        return_ty,
        body: None,
    })
}
