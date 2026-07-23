use serde::{Deserialize, Serialize};

use crate::resource::ResourceRef;

/// Typed relationship between graph nodes (DEC-016 graph foundation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphRelationship {
    /// Parent workspace contains a child resource.
    Contains,
}

impl GraphRelationship {
    pub fn as_str(&self) -> &'static str {
        match self {
            GraphRelationship::Contains => "contains",
        }
    }
}

/// A directed edge between two resource nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: ResourceRef,
    pub relationship: GraphRelationship,
    pub target: ResourceRef,
}

impl GraphEdge {
    pub fn contains(parent: ResourceRef, child: ResourceRef) -> Self {
        Self {
            source: parent,
            relationship: GraphRelationship::Contains,
            target: child,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::{ResourceId, ResourceKind};

    #[test]
    fn contains_edge_links_workspace_to_zone() {
        let edge = GraphEdge::contains(
            ResourceRef::new(
                ResourceKind::Workspace,
                ResourceId::new("ws-1").unwrap(),
            ),
            ResourceRef::new(ResourceKind::Zone, ResourceId::new("zone-1").unwrap()),
        );
        assert_eq!(edge.relationship, GraphRelationship::Contains);
        assert_eq!(edge.source.canonical(), "workspace:ws-1");
        assert_eq!(edge.target.canonical(), "zone:zone-1");
    }
}
