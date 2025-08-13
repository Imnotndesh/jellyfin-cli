use inquire::Select;

pub fn choose_item<T, F>(items: &[T], display: F) -> Option<&T>
where
    F: Fn(&T) -> String,
{
    if items.is_empty() {
        println!("No items available.");
        return None;
    }

    let options: Vec<String> = items.iter().map(|item| display(item)).collect();

    let selection = Select::new("Choose an item:", options).prompt();

    match selection {
        Ok(choice) => {
            let index = items.iter().position(|item| display(item) == choice)?;
            Some(&items[index])
        }
        Err(_) => None,
    }
}

