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
    classification: String,
    validity_date: DateTime<Local>,
    rule_code: RuleCode,
}


#[derive(Debug)]
struct Inventary {
    id: Uuid,
    id_stock: Uuid,
    count: i32,
    physical_value: i32,
    system_value: i32,
}


#[derive(Debug)]
struct IssueInventary {
    id: Uuid,
    id_inventary: Uuid,
    description: String,
}

#[derive(Debug, Deserialize)]
pub enum StockCommand {
    InitialStock { item: Item, value: i32},
    InsertItem { item: Item, value: i32 },
    RemoveItem { item: Item, value: i32 },
    InventoryCalculateStarted { item: Item, value: i32 },
    InventoryCalculateInscrement { item: Item, value: i32 }
    InventoryCalculateDecrement { item: Item, value: i32 }
    InventoryCalculateEnd { item: Item }
    InventoryItem { inventary: InventoryItem, issue: Some<IssueInventary> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StockEvent {
    StockInited {
        id: Uuid,
        item: Item,
        value: i32,
    },
    InventoryItem { inventary: InventoryItem, issue: Some<IssueInventary> },
}

impl DomainEvent for StockEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            StockEvent::StockInited { .. } => "StockInited",
            StockEvent::InventoryItem { .. } => "InventoryItem",
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug, PartialEq)]
pub struct StockError(String);

impl From<&str> for StockError {
    fn from(message: &str) -> Self {
        StockError(message.to_string())
    }
}

pub struct StockServices;

impl StockServices {
    async fn atm_withdrawal(&self, atm_id: &str, amount: f64) -> Result<(), AtmError> {
        Ok(())
    }

    async fn validate_check(&self, account: &str, check: &str) -> Result<(), CheckingError> {
        Ok(())
    }
}

pub struct AtmError;
pub struct CheckingError;

#[derive(Debug)]
struct Stock {
    id: Uuid,
    item: Item,
    value: i32,
}

#[async_trait]
impl Aggregate for Stock {
    type Command = StockCommand;
    type Event = StockEvent;
    type Service = StockServices;

    fn aggrate_type() -> String {
        "Stock".to_string()
    }

    async fn handle(
        &self,
        command: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
        
        }
    }
}

