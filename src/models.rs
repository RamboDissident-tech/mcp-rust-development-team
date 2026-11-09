use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Structure de données pour un élément
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    pub id: Option<u32>,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl Item {
    /// Crée un nouvel item avec l'ID et la date de création
    pub fn new(id: u32, name: String) -> Self {
        Item {
            id: Some(id),
            name,
            created_at: None,
        }
    }

    /// Crée un nouvel item sans ID (pour la création)
    pub fn with_name(name: String) -> Self {
        Item {
            id: None,
            name,
            created_at: None,
        }
    }

    /// Met à jour le nom de l'item
    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }
}

/// Structure pour les résultats des opérations CRUD
#[derive(Debug, Deserialize, Serialize)]
pub struct CrudResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> CrudResult<T> {
    /// Crée un résultat succès
    pub fn success(data: T, message: String) -> Self {
        CrudResult {
            success: true,
            data: Some(data),
            message,
        }
    }

    /// Crée un résultat d'échec
    pub fn error(message: String) -> Self {
        CrudResult {
            success: false,
            data: None,
            message,
        }
    }

    /// Crée un résultat avec données mais sans succès
    pub fn with_data(data: T, message: String) -> Self {
        CrudResult {
            success: true,
            data: Some(data),
            message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item::new(1, "Test Item".to_string());
        assert_eq!(item.id, Some(1));
        assert_eq!(item.name, "Test Item");
    }

    #[test]
    fn test_item_with_name() {
        let item = Item::with_name("Test Item".to_string());
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.id, None);
    }

    #[test]
    fn test_crud_result_success() {
        let result = CrudResult::success(42u32, "Success".to_string());
        assert!(result.success);
        assert_eq!(result.data, Some(42u32));
        assert_eq!(result.message, "Success");
    }

    #[test]
    fn test_crud_result_error() {
        let result = CrudResult::error("Error occurred".to_string());
        assert!(!result.success);
        assert_eq!(result.data, None);
        assert_eq!(result.message, "Error occurred");
    }
}