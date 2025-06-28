use std::collections::HashMap;

#[derive(Debug)]
struct Inventory {
    items: HashMap<String, u32>,
}

impl Inventory {
    fn new() -> Self {
        todo!()
    }
    
    fn add_item(&mut self, name: String, quantity: u32) {
        todo!()
    }
    
    fn remove_item(&mut self, name: &str, quantity: u32) -> Result<(), &'static str> {
        todo!()
    }
    
    fn get_quantity(&self, name: &str) -> u32 {
        todo!()
    }
    
    fn total_items(&self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_inventory() {
        let mut inv = Inventory::new();
        inv.add_item("apples".to_string(), 10);
        inv.add_item("oranges".to_string(), 5);
        
        assert_eq!(inv.get_quantity("apples"), 10);
        assert_eq!(inv.total_items(), 15);
        
        assert!(inv.remove_item("apples", 3).is_ok());
        assert_eq!(inv.get_quantity("apples"), 7);
        
        assert!(inv.remove_item("apples", 10).is_err());
    }
}