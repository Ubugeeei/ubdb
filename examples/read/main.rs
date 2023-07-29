use ubdb::storage::read;

fn main() {
    let (table, data) = read();

    println!("{:#?}\n", table);

    println!("raw data: {:?}\n", data);

    for (i, d) in data.iter().enumerate() {
        if i == 0 {
            let key_name = table.pk.key.clone();
            println!("{}: {}", key_name, d.into_display(&table.pk));
        } else {
            let column = &table.columns[i - 1];
            println!("{}: {}", column.key, d.into_display(column));
        }
    }
}
