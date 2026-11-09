//! Tests unitaires pour l'application Rust avec base de données SQLite

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_initialization() {
        // Test d'initialisation de la base de données
        let db = Database::new(":memory:").unwrap();
        db.init().unwrap();
        
        assert!(true); // Si on arrive ici sans erreur, l'initialisation a réussi
    }

    #[test]
    fn test_create_item() {
        let db = Database::new(":memory:").unwrap();
        db.init().unwrap();

        let item_id = db.create_item("Test Item").unwrap();
        assert!(item_id > 0);
    }

    #[test]
    fn test_get_item() {
        let db = Database::new(":memory:").unwrap();
        db.init().unwrap();

        let item_id = db.create_item("Test Item").unwrap();
        let item = db.get_item(item_id).unwrap();
        
        assert!(item.is_some());
        assert_eq!(item.unwrap().0, item_id);
    }

    #[test]
    fn test_get_all_items() {
        let db = Database::new(":memory:").unwrap();
        db.init().unwrap();

        let _id1 = db.create_item("Item 1").unwrap();
        let _id2 = db.create_item("Item 2").unwrap();
        
        let items = db.get_all_items().unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_update_item() {
        let db = Database::new(":memory:").unwrap();
        db.init().unwrap();

        let item_id = db.create_item("Original Item").unwrap();
        let updated = db.update_item(item_id, "Updated Item").unwrap();
        
        assert!(updated);
    }

    #[test]
    fn test_delete_item() {
        let db = Database::new(":memory:").unwrap();
        db.init().unwrap();

        let item_id = db.create_item("Item to delete").unwrap();
        let deleted = db.delete_item(item_id).unwrap();
        
        assert!(deleted);
    }

    #[test]
    fn test_item_struct() {
        let item = Item::new(1, "Test Item".to_string());
        assert_eq!(item.id, Some(1));
        assert_eq!(item.name, "Test Item");
    }

    #[test]
    fn test_crud_result() {
        let result = CrudResult::success(42u32, "Success".to_string());
        assert!(result.success);
        assert_eq!(result.data, Some(42u32));
    }
}