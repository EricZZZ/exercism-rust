use std::collections::HashMap;

fn main() {
    let mut spreadsheet: Spreadsheet = Spreadsheet::new(3);
    spreadsheet.get_value("=5+7".to_string()); // 返回 12 (5+7)
    spreadsheet.set_cell("A1".to_string(), 10); // 设置 A1 为 10
}

struct Spreadsheet {
    map: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Spreadsheet {
            map: HashMap::new(),
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.map.insert(cell, value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.map.insert(cell, 0);
    }

    fn get_value(&self, mut formula: String) -> i32 {
        let str_v: Vec<&str> = formula[1..].split('+').collect();
        let num1 = if let Ok(num1) = str_v[0].parse::<i32>() {
            num1
        } else {
            *self.map.get(str_v[0]).unwrap_or(&0)
        };
        let num2 = if let Ok(num2) = str_v[1].parse::<i32>() {
            num2
        } else {
            *self.map.get(str_v[1]).unwrap_or(&0)
        };
        num1 + num2
    }
}
