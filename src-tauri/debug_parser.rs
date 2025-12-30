fn main() {
    let input = "\tOnly comment\n  Another comment";
    println!("Input: {:?}", input);
    
    let result = hyperreview_lib::commands::text_parser::parse_task_text(input);
    println!("Result: {:?}", result);
    
    if let Ok(items) = result {
        println!("Found {} items:", items.len());
        for (i, item) in items.iter().enumerate() {
            println!("  Item {}: file='{}' comment='{:?}'", i, item.file, item.preset_comment);
        }
    }
}
