//! Passive workspace graph — node and edge registration only.

use workspace_database::{Database, GraphRepository};
use workspace_domain::{GraphEdge, GraphRelationship, ResourceKind, ResourceRef};

use crate::error::{KernelError, Result};

/// Passive graph coordinator — registers resource nodes and edges (Sprint 12).
///
/// Services call this API; they never manipulate graph tables directly.
pub struct GraphService;

impl GraphService {
    pub fn register_node(db: &Database, resource: &ResourceRef) -> Result<()> {
        GraphRepository::new(db)
            .register_node(resource)
            .map_err(|error| map_graph_error(resource, error))?;
        Ok(())
    }

    pub fn register_child(
        db: &Database,
        parent: &ResourceRef,
        child: &ResourceRef,
    ) -> Result<()> {
        if parent.kind != ResourceKind::Workspace {
            return Err(KernelError::InvalidParentReference {
                expected_kind: ResourceKind::Workspace,
            });
        }

        if !Self::exists(db, parent)? {
            return Err(KernelError::ResourceNotFound {
                kind: parent.kind,
            });
        }

        Self::register_node(db, child)?;
        GraphRepository::new(db)
            .add_edge(parent, GraphRelationship::Contains, child)
            .map_err(|error| KernelError::Database(error))?;
        Ok(())
    }

    pub fn unregister_node(db: &Database, resource: &ResourceRef) -> Result<()> {
        GraphRepository::new(db)
            .remove_node(resource)
            .map_err(|error| map_graph_error(resource, error))?;
        Ok(())
    }

    pub fn exists(db: &Database, resource: &ResourceRef) -> Result<bool> {
        GraphRepository::new(db)
            .node_exists(resource)
            .map_err(|error| map_graph_error(resource, error))
    }

    pub fn lookup(db: &Database, resource: &ResourceRef) -> Result<Option<()>> {
        let exists = Self::exists(db, resource)?;
        Ok(if exists { Some(()) } else { None })
    }

    pub fn list_edges_from(db: &Database, source: &ResourceRef) -> Result<Vec<GraphEdge>> {
        let edges = GraphRepository::new(db)
            .list_edges_from_source(source)
            .map_err(|error| map_graph_error(source, error))?;

        Ok(edges
            .into_iter()
            .map(|(relationship, target)| GraphEdge {
                source: source.clone(),
                relationship,
                target,
            })
            .collect())
    }
}

fn map_graph_error(
    resource: &ResourceRef,
    error: workspace_database::DatabaseError,
) -> KernelError {
    match error {
        workspace_database::DatabaseError::DuplicateResource(_) => KernelError::DuplicateResource {
            kind: resource.kind,
        },
        workspace_database::DatabaseError::NotFound(_) => KernelError::ResourceNotFound {
            kind: resource.kind,
        },
        other => KernelError::Database(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_database::DatabaseService;
    use workspace_domain::ResourceId;
    use tempfile::tempdir;

    fn test_db() -> workspace_database::Database {
        let dir = tempdir().unwrap();
        DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database()
    }

    #[test]
    fn registers_workspace_and_child_zone() {
        let db = test_db();
        let workspace = ResourceRef::new(
            ResourceKind::Workspace,
            ResourceId::new("ws-g").unwrap(),
        );
        let zone = ResourceRef::new(ResourceKind::Zone, ResourceId::new("zone-g").unwrap());

        GraphService::register_node(&db, &workspace).unwrap();
        GraphService::register_child(&db, &workspace, &zone).unwrap();

        assert!(GraphService::exists(&db, &zone).unwrap());
    }

    #[test]
    fn rejects_non_workspace_parent() {
        let db = test_db();
        let zone = ResourceRef::new(ResourceKind::Zone, ResourceId::new("zone-p").unwrap());
        let app = ResourceRef::new(
            ResourceKind::Application,
            ResourceId::new("app-p").unwrap(),
        );

        GraphService::register_node(&db, &zone).unwrap();
        let error = GraphService::register_child(&db, &zone, &app).unwrap_err();
        assert!(matches!(
            error,
            KernelError::InvalidParentReference {
                expected_kind: ResourceKind::Workspace
            }
        ));
    }
}
