use uuid::{uuid, Uuid};
use chrono::{DateTime, Local};

#[derive(Debug)]
enum RuleCode {
    EAN,
    UPC,
    ITF,
    Code_39,
    Codebar,
    Code_128,
}

#[derive(Debug)]
struct Item {
    code: String,
    name: String,
    classification: Vec<String>,
    validity_date: DateTime<Local>,
    rule_code: RuleCode,
}


#[derive(Debug)]
struct InventaryStarted {
    id: Uuid,
    id_stock: Uuid,
    system_value: i32,
}

#[derive(Debug)]
struct InventaryCount {
    id: Uuid,
    id_stock: Uuid,
    count: i32,
    system_value: i32,
}

#[derive(Debug)]
struct Inventary {
    id: Uuid,
    id_stock: Uuid,
    physical_value: i32,
    system_value: i32,
}

impl Inventary {
    fn start_inventory(stock: Stock, system_value: i32) -> InventaryStarted {
        todo!()
    }

    fn increment_inventory(stock: Stock, count: i32) -> Result<InventaryCount, IssueInventary> {
        todo!()
    }

    fn decrement_inventory(stock: Stock, count: i32) -> Result<InventaryCount, IssueInventary> {
        todo!()
    }

    fn finish_inventory(stock: Stock) -> Inventary {
        todo!()
    }
}


#[derive(Debug)]
struct IssueInventary {
    id: Uuid,
    id_inventary: Uuid,
    description: String,
}

#[derive(Debug)]
struct Stock {
    id: Uuid,
    item: Item,
    value: i32,
}

impl Stock {
    fn started(item: Item, date: DateTime<Local>) -> Stock {
        Stock {
          id: Uuid::new_v4(),
          item,
          value: 0
        }
    }

    fn insert_item(item: Item, isert_value: i32) -> Stock {
        todo!()
    }

    fn remove_item(item: Item, remove_value: i32) -> Stock {
        todo!()
    }
}

#[cfg(test)]
mod stock_tests {
    use super::*;
    
    #[test]
    fn started_stock_item() {
        let item = Item {
            code: String::from("1234"),
            name: String::from("Papel Toalha"),
            classification: vec!["utencilios".to_string(), "cozinha".to_string()],
            validity_date: Local::now(),
            rule_code: RuleCode::EAN,
        };
        let stock = Stock::started(item, Local::now());
        assert_eq!(stock.value, 0)
    }
}

