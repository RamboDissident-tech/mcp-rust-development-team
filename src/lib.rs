//! Bibliothèque principale pour l'application Rust avec base de données SQLite

pub mod database;
pub mod models;

// Réexportation des types principaux pour une utilisation facile
pub use database::Database;
pub use models::{Item, CrudResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_import() {
        // Ce test vérifie que la bibliothèque peut être importée correctement
        assert!(true);
    }
}