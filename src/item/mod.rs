pub mod data;

use super::ast::data as ast;
use data as item;

fn prepare_items(input: ast::File) -> item::File {
    todo!()
}

/*
Go through all items
Record names and kinds
Go though all items again
Check signatures
Check bodies (not functions)
Check bodies (functions)
*/
