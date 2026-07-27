use uuid::Uuid;

use crate::connection::Database;
use crate::error::{DatabaseError, Result};
use workspace_domain::{GraphRelationship, ResourceId, ResourceKind, ResourceRef};

/// Persistence for the passive workspace graph (nodes + edges).
pub struct GraphRepository<'a> {
    db: &'a Database,
}

impl<'a> GraphRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn register_node(&self, resource: &ResourceRef) -> Result<()> {
        if self.node_exists(resource)? {
            return Err(DatabaseError::DuplicateResource(resource.canonical()));
        }

        self.db.connection().execute(
            "INSERT INTO graph_nodes (kind, id) VALUES (?1, ?2)",
            (kind_to_str(resource.kind), resource.id.as_str()),
        )?;
        Ok(())
    }

    pub fn remove_node(&self, resource: &ResourceRef) -> Result<bool> {
        let changed = self.db.connection().execute(
            "DELETE FROM graph_nodes WHERE kind = ?1 AND id = ?2",
            (kind_to_str(resource.kind), resource.id.as_str()),
        )?;
        Ok(changed > 0)
    }

    pub fn node_exists(&self, resource: &ResourceRef) -> Result<bool> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM graph_nodes WHERE kind = ?1 AND id = ?2",
            (kind_to_str(resource.kind), resource.id.as_str()),
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn add_edge(
        &self,
        source: &ResourceRef,
        relationship: GraphRelationship,
        target: &ResourceRef,
    ) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO graph_edges (id, source_kind, source_id, relationship, target_kind, target_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                Uuid::new_v4().to_string(),
                kind_to_str(source.kind),
                source.id.as_str(),
                relationship.as_str(),
                kind_to_str(target.kind),
                target.id.as_str(),
            ),
        )?;
        Ok(())
    }

    pub fn edge_count_for_source(&self, source: &ResourceRef) -> Result<i64> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM graph_edges WHERE source_kind = ?1 AND source_id = ?2",
            (kind_to_str(source.kind), source.id.as_str()),
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub fn list_edges_from_source(&self, source: &ResourceRef) -> Result<Vec<(GraphRelationship, ResourceRef)>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT relationship, target_kind, target_id FROM graph_edges
             WHERE source_kind = ?1 AND source_id = ?2
             ORDER BY target_kind, target_id",
        )?;

        let rows = stmt.query_map(
            (kind_to_str(source.kind), source.id.as_str()),
            |row| {
                let relationship = parse_relationship(row.get::<_, String>(0)?.as_str())?;
                let target_kind = parse_kind(row.get::<_, String>(1)?.as_str())?;
                let target_id = ResourceId::new(row.get::<_, String>(2)?).map_err(|_| {
                    rusqlite::Error::InvalidColumnType(
                        2,
                        "target_id".into(),
                        rusqlite::types::Type::Text,
                    )
                })?;
                Ok((
                    relationship,
                    ResourceRef::new(target_kind, target_id),
                ))
            },
        )?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn parse_kind(value: &str) -> rusqlite::Result<ResourceKind> {
    match value {
        "workspace" => Ok(ResourceKind::Workspace),
        "zone" => Ok(ResourceKind::Zone),
        "application" => Ok(ResourceKind::Application),
        "widget" => Ok(ResourceKind::Widget),
        _ => Err(rusqlite::Error::InvalidColumnType(
            0,
            "resource_kind".into(),
            rusqlite::types::Type::Text,
        )),
    }
}

fn parse_relationship(value: &str) -> rusqlite::Result<GraphRelationship> {
    match value {
        "contains" => Ok(GraphRelationship::Contains),
        _ => Err(rusqlite::Error::InvalidColumnType(
            0,
            "relationship".into(),
            rusqlite::types::Type::Text,
        )),
    }
}

fn kind_to_str(kind: ResourceKind) -> &'static str {
    match kind {
        ResourceKind::Workspace => "workspace",
        ResourceKind::Zone => "zone",
        ResourceKind::Application => "application",
        ResourceKind::Widget => "widget",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::DatabaseService;
    use workspace_domain::ResourceId;
    use tempfile::tempdir;

    fn initialized_db() -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();
        (dir, db)
    }

    #[test]
    fn registers_and_removes_nodes() {
        let (_dir, db) = initialized_db();
        let repo = GraphRepository::new(&db);
        let node = ResourceRef::new(
            ResourceKind::Workspace,
            ResourceId::new("ws-graph").unwrap(),
        );

        repo.register_node(&node).unwrap();
        assert!(repo.node_exists(&node).unwrap());
        assert!(repo.remove_node(&node).unwrap());
        assert!(!repo.node_exists(&node).unwrap());
    }

    #[test]
    fn rejects_duplicate_nodes() {
        let (_dir, db) = initialized_db();
        let repo = GraphRepository::new(&db);
        let node = ResourceRef::new(
            ResourceKind::Zone,
            ResourceId::new("zone-dup").unwrap(),
        );

        repo.register_node(&node).unwrap();
        assert!(matches!(
            repo.register_node(&node),
            Err(DatabaseError::DuplicateResource(_))
        ));
    }

    #[test]
    fn adds_contains_edge_between_nodes() {
        let (_dir, db) = initialized_db();
        let repo = GraphRepository::new(&db);
        let workspace = ResourceRef::new(
            ResourceKind::Workspace,
            ResourceId::new("ws-edge").unwrap(),
        );
        let zone = ResourceRef::new(ResourceKind::Zone, ResourceId::new("zone-edge").unwrap());

        repo.register_node(&workspace).unwrap();
        repo.register_node(&zone).unwrap();
        repo.add_edge(&workspace, GraphRelationship::Contains, &zone)
            .unwrap();

        assert_eq!(repo.edge_count_for_source(&workspace).unwrap(), 1);
    }

    #[test]
    fn lists_edges_from_source() {
        let (_dir, db) = initialized_db();
        let repo = GraphRepository::new(&db);
        let workspace = ResourceRef::new(
            ResourceKind::Workspace,
            ResourceId::new("ws-list").unwrap(),
        );
        let zone = ResourceRef::new(ResourceKind::Zone, ResourceId::new("zone-list").unwrap());

        repo.register_node(&workspace).unwrap();
        repo.register_node(&zone).unwrap();
        repo.add_edge(&workspace, GraphRelationship::Contains, &zone)
            .unwrap();

        let edges = repo.list_edges_from_source(&workspace).unwrap();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].0, GraphRelationship::Contains);
        assert_eq!(edges[0].1, zone);
    }
}
