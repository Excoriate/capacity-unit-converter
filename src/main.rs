use std::env;
use prettytable::{Table, row, cell};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please provide a number and unit (e.g. 5000 KB)");
        return;
    }

    let num = args[1].parse::<f64>().unwrap();
    let unit = &args[2];
    let mut table = Table::new();

    // Add a title row
    table.add_row(row!["Unit", "Value"]);

    match unit.to_lowercase().as_str() {
        "b" => {
            table.add_row(row!["B", num]);
            table.add_row(row!["KB", num / 1024.0]);
            table.add_row(row!["MB", num / 1024.0 / 1024.0]);
            table.add_row(row!["GB", num / 1024.0 / 1024.0 / 1024.0]);
            table.add_row(row!["TB", num / 1024.0 / 1024.0 / 1024.0 / 1024.0]);
            table.add_row(row!["PB", num / 1024.0 / 1024.0 / 1024.0 / 1024.0 / 1024.0]);
        }
        "kb" => {
            table.add_row(row!["B", num * 1024.0]);
            table.add_row(row!["KB", num]);
            table.add_row(row!["MB", num / 1024.0]);
            table.add_row(row!["GB", num / 1024.0 / 1024.0]);
            table.add_row(row!["TB", num / 1024.0 / 1024.0 / 1024.0]);
            table.add_row(row!["PB", num / 1024.0 / 1024.0 / 1024.0 / 1024.0]);
        }
        "mb" => {
            table.add_row(row!["B", num * 1024.0 * 1024.0]);
            table.add_row(row!["KB", num * 1024.0]);
            table.add_row(row!["MB", num]);
            table.add_row(row!["GB", num / 1024.0]);
            table.add_row(row!["TB", num / 1024.0 / 1024.0]);
            table.add_row(row!["PB", num / 1024.0 / 1024.0 / 1024.0]);
        }
        "gb" => {
            table.add_row(row!["B", num * 1024.0 * 1024.0 * 1024.0]);
            table.add_row(row!["KB", num * 1024.0 * 1024.0]);
            table.add_row(row!["MB", num * 1024.0]);
            table.add_row(row!["GB", num]);
            table.add_row(row!["TB", num / 1024.0]);
            table.add_row(row!["PB", num / 1024.0 / 1024.0]);
        }
        "tb" => {
            table.add_row(row!["B", num * 1024.0 * 1024.0 * 1024.0 * 1024.0]);
            table.add_row(row!["KB", num * 1024.0 * 1024.0 * 1024.0]);
            table.add_row(row!["MB", num * 1024.0 * 1024.0]);
            table.add_row(row!["GB", num * 1024.0]);
            table.add_row(row!["TB", num]);
            table.add_row(row!["PB", num / 1024.0]);
        }
        "pb" =>{
            table.add_row(row!["B", num * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0]);
            table.add_row(row!["KB", num * 1024.0 * 1024.0 * 1024.0 * 1024.0]);
            table.add_row(row!["MB", num * 1024.0 * 1024.0 * 1024.0]);
            table.add_row(row!["GB", num * 1024.0 * 1024.0]);
            table.add_row(row!["TB", num * 1024.0]);
            table.add_row(row!["PB", num]);
        }

        _ => {
            println!("Unrecognised unit. Please use B, KB, MB, GB, TB, or PB");
            return;
        }
    }

    // Print the table to stdout
    table.printstd();
}
