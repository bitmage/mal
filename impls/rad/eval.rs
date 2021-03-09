use std::io;

use types::{
    RadVal,
    RadNode,
    RadList,
    error_invalid_data,
    make_node,
    map_to_list,
    make_map_val,
};

use env::ReplEnv;

// eval all the items in the list
pub fn eval_all(items: &RadList, ns: &ReplEnv) -> io::Result<RadList> {
    let mut evaled_items: RadList = Vec::new();
    for i in 0..items.len() {
        evaled_items.push(eval_ast(&items[i], ns)?);
    };
    Ok(evaled_items)
}

pub fn eval_ast(tree: &RadNode, ns: &ReplEnv) -> io::Result<RadNode> {
    match &tree.rval {
        RadVal::Symbol => Ok(tree.clone()),
        RadVal::Map(map) => {
            // holy smokes this is crazy inefficient
            // TODO: find some way to iterate in place without
            // changing data structures
            let items = map_to_list(map);
            Ok(make_node(&tree.text, make_map_val(eval_all(&items, ns)?)))
        },
        RadVal::Array(items) => {
            Ok(make_node(&tree.text, RadVal::Array(eval_all(items, ns)?)))
        },
        RadVal::List(items) => {
            let evaled_items = eval_all(items, ns)?;
            // if we have a form at the beginning of the list
            // then run it as a function
            if let Some(form) = evaled_items.get(0) {
                // lookup the function in the current namespace
                match ns.get(form.text.as_str()) {
                    // run the function, passing the rest of the list items
                    Some(fun) => fun(&evaled_items.iter().skip(1).collect()),
                    None => {
                        let txt = format!("{} was not found in namespace!", form.text);
                        Err(error_invalid_data(txt))
                    }
                }
            // otherwise return a copy of the list
            } else {
                Ok(tree.clone())
            }
        },
        // or return whatever we have unmodified
        _ => Ok(tree.clone()),
    }
}
