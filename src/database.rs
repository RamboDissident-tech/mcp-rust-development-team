use rusqlite::{Connection, Result, Params};
use std::path::Path;

/// Gestion de la base de données SQLite pour l'application
pub struct Database {
    connection: Connection,
}

impl Database {
    /// Crée une nouvelle instance de base de données
    pub fn new(db_path: &str) -> Result<Database> {
        let connection = Connection::open(db_path)?;
        Ok(Database { connection })
    }

    /// Initialise la base de données avec les tables nécessaires
    pub fn init(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        Ok(())
    }

    /// Crée un nouvel élément dans la base de données
    pub fn create_item(&self, name: &str) -> Result<u32> {
        let mut stmt = self.connection.prepare(
            "INSERT INTO items (name) VALUES (?1)"
        )?;
        stmt.execute([name])?;
        
        Ok(self.connection.last_insert_rowid() as u32)
    }

    /// Récupère un élément par son ID
    pub fn get_item(&self, id: u32) -> Result<Option<(u32, String)>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, name FROM items WHERE id = ?1"
        )?;
        
        let mut rows = stmt.query([id])?;
        if let Some(row) = rows.next()? {
            let id: u32 = row.get(0)?;
            let name: String = row.get(1)?;
            Ok(Some((id, name)))
        } else {
            Ok(None)
        }
    }

    /// Récupère tous les éléments
    pub fn get_all_items(&self) -> Result<Vec<(u32, String)>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, name FROM items"
        )?;
        
        let mut rows = stmt.query([])?;
        let mut items = Vec::new();
        
        while let Some(row) = rows.next()? {
            let id: u32 = row.get(0)?;
            let name: String = row.get(1)?;
            items.push((id, name));
        }
        
        Ok(items)
    }

    /// Met à jour un élément
    pub fn update_item(&self, id: u32, name: &str) -> Result<bool> {
        let mut stmt = self.connection.prepare(
            "UPDATE items SET name = ?1 WHERE id = ?2"
        )?;
        
        let changes = stmt.execute([name, id.to_string()])?;
        Ok(changes > 0)
    }

    /// Supprime un élément
    pub fn delete_item(&self, id: u32) -> Result<bool> {
        let mut stmt = self.connection.prepare(
            "DELETE FROM items WHERE id = ?1"
        )?;
        
        let changes = stmt.execute([id.to_string()])?;
        Ok(changes > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = Database::new(":memory:").unwrap();
        db.init().unwrap();
        
        // Test de création d'élément
        let item_id = db.create_item("Test Item").unwrap();
        assert!(item_id > 0);
    }

    #[test]
    fn test_database_operations() {
        let db = Database::new(":memory:").unwrap();
        db.init().unwrap();

        // Création
        let item_id = db.create_item("Test Item").unwrap();
        
        // Récupération
        let item = db.get_item(item_id).unwrap();
        assert!(item.is_some());
        
        // Mise à jour
        let updated = db.update_item(item_id, "Updated Item").unwrap();
        assert!(updated);
        
        // Suppression
        let deleted = db.delete_item(item_id).unwrap();
        assert!(deleted);
    }
}