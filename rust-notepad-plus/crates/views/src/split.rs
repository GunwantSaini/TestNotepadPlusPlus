//! Split view container and layout

use crate::view::{View, ViewId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Direction of a split
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    /// Horizontal split (top/bottom)
    Horizontal,

    /// Vertical split (left/right)
    Vertical,
}

/// Represents a split container that can hold views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SplitContainer {
    /// A single view
    View(ViewId),

    /// A split containing two containers
    Split {
        /// Direction of the split
        direction: SplitDirection,

        /// First container (top or left)
        first: Box<SplitContainer>,

        /// Second container (bottom or right)
        second: Box<SplitContainer>,

        /// Split ratio (0.0 to 1.0, representing first container's size)
        ratio: f64,
    },
}

impl SplitContainer {
    /// Create a new single-view container
    pub fn new_single(view_id: ViewId) -> Self {
        SplitContainer::View(view_id)
    }

    /// Create a new split container
    pub fn new_split(
        direction: SplitDirection,
        first: SplitContainer,
        second: SplitContainer,
        ratio: f64,
    ) -> Self {
        SplitContainer::Split {
            direction,
            first: Box::new(first),
            second: Box::new(second),
            ratio: ratio.clamp(0.1, 0.9),
        }
    }

    /// Check if this is a single view
    pub fn is_single_view(&self) -> bool {
        matches!(self, SplitContainer::View(_))
    }

    /// Check if this is a split
    pub fn is_split(&self) -> bool {
        matches!(self, SplitContainer::Split { .. })
    }

    /// Get all view IDs in this container
    pub fn view_ids(&self) -> Vec<ViewId> {
        match self {
            SplitContainer::View(id) => vec![*id],
            SplitContainer::Split { first, second, .. } => {
                let mut ids = first.view_ids();
                ids.extend(second.view_ids());
                ids
            }
        }
    }

    /// Count the number of views
    pub fn view_count(&self) -> usize {
        match self {
            SplitContainer::View(_) => 1,
            SplitContainer::Split { first, second, .. } => {
                first.view_count() + second.view_count()
            }
        }
    }

    /// Find the container that contains the given view ID
    pub fn find_view(&self, view_id: ViewId) -> bool {
        match self {
            SplitContainer::View(id) => *id == view_id,
            SplitContainer::Split { first, second, .. } => {
                first.find_view(view_id) || second.find_view(view_id)
            }
        }
    }

    /// Get the depth of the split tree
    pub fn depth(&self) -> usize {
        match self {
            SplitContainer::View(_) => 0,
            SplitContainer::Split { first, second, .. } => {
                1 + first.depth().max(second.depth())
            }
        }
    }
}

/// Manages split views
pub struct SplitViewManager {
    /// Root split container
    root: SplitContainer,

    /// All views indexed by ID
    views: HashMap<ViewId, View>,

    /// Currently focused view ID
    focused_view: Option<ViewId>,
}

impl SplitViewManager {
    /// Create a new split view manager with a single view
    pub fn new(initial_view: View) -> Self {
        let view_id = initial_view.id;
        let mut views = HashMap::new();
        views.insert(view_id, initial_view);

        Self {
            root: SplitContainer::new_single(view_id),
            views,
            focused_view: Some(view_id),
        }
    }

    /// Split a view
    pub fn split_view(
        &mut self,
        view_id: ViewId,
        direction: SplitDirection,
        new_view: View,
    ) -> Result<ViewId, String> {
        // Check if view exists
        if !self.views.contains_key(&view_id) {
            return Err(format!("View {} not found", view_id));
        }

        let new_view_id = new_view.id;
        self.views.insert(new_view_id, new_view);

        // Split the container
        self.root = self.split_container_recursive(
            self.root.clone(),
            view_id,
            direction,
            new_view_id,
        )?;

        Ok(new_view_id)
    }

    /// Recursive helper to split a container
    fn split_container_recursive(
        &self,
        container: SplitContainer,
        target_view_id: ViewId,
        direction: SplitDirection,
        new_view_id: ViewId,
    ) -> Result<SplitContainer, String> {
        match container {
            SplitContainer::View(id) if id == target_view_id => {
                Ok(SplitContainer::new_split(
                    direction,
                    SplitContainer::new_single(id),
                    SplitContainer::new_single(new_view_id),
                    0.5,
                ))
            }
            SplitContainer::View(_) => Ok(container),
            SplitContainer::Split {
                direction: dir,
                first,
                second,
                ratio,
            } => {
                if first.find_view(target_view_id) {
                    let new_first = self.split_container_recursive(
                        *first,
                        target_view_id,
                        direction,
                        new_view_id,
                    )?;
                    Ok(SplitContainer::new_split(dir, new_first, *second, ratio))
                } else if second.find_view(target_view_id) {
                    let new_second = self.split_container_recursive(
                        *second,
                        target_view_id,
                        direction,
                        new_view_id,
                    )?;
                    Ok(SplitContainer::new_split(dir, *first, new_second, ratio))
                } else {
                    Ok(SplitContainer::new_split(dir, *first, *second, ratio))
                }
            }
        }
    }

    /// Close a view
    pub fn close_view(&mut self, view_id: ViewId) -> Result<(), String> {
        // Don't allow closing the last view
        if self.views.len() == 1 {
            return Err("Cannot close the last view".to_string());
        }

        // Remove from views
        self.views
            .remove(&view_id)
            .ok_or_else(|| format!("View {} not found", view_id))?;

        // Remove from container
        self.root = self.remove_from_container(self.root.clone(), view_id)?;

        // Update focused view if needed
        if self.focused_view == Some(view_id) {
            self.focused_view = self.views.keys().next().copied();
        }

        Ok(())
    }

    /// Recursive helper to remove a view from container
    fn remove_from_container(
        &self,
        container: SplitContainer,
        view_id: ViewId,
    ) -> Result<SplitContainer, String> {
        match container {
            SplitContainer::View(id) if id == view_id => {
                Err(format!("Cannot remove view {}", view_id))
            }
            SplitContainer::View(_) => Ok(container),
            SplitContainer::Split {
                direction,
                first,
                second,
                ratio,
            } => {
                // Check if the target view is in first or second
                let first_has_view = first.find_view(view_id);
                let second_has_view = second.find_view(view_id);

                if first_has_view && first.view_count() == 1 {
                    // First container only has the view to remove, return second
                    Ok(*second)
                } else if second_has_view && second.view_count() == 1 {
                    // Second container only has the view to remove, return first
                    Ok(*first)
                } else if first_has_view {
                    // Recursively remove from first
                    let new_first = self.remove_from_container(*first, view_id)?;
                    Ok(SplitContainer::new_split(direction, new_first, *second, ratio))
                } else if second_has_view {
                    // Recursively remove from second
                    let new_second = self.remove_from_container(*second, view_id)?;
                    Ok(SplitContainer::new_split(direction, *first, new_second, ratio))
                } else {
                    Ok(SplitContainer::new_split(direction, *first, *second, ratio))
                }
            }
        }
    }

    /// Get a view by ID
    pub fn get_view(&self, view_id: ViewId) -> Option<&View> {
        self.views.get(&view_id)
    }

    /// Get a mutable view by ID
    pub fn get_view_mut(&mut self, view_id: ViewId) -> Option<&mut View> {
        self.views.get_mut(&view_id)
    }

    /// Get all views
    pub fn views(&self) -> Vec<&View> {
        self.views.values().collect()
    }

    /// Get all view IDs
    pub fn view_ids(&self) -> Vec<ViewId> {
        self.root.view_ids()
    }

    /// Get the number of views
    pub fn view_count(&self) -> usize {
        self.views.len()
    }

    /// Get the focused view ID
    pub fn focused_view_id(&self) -> Option<ViewId> {
        self.focused_view
    }

    /// Get the focused view
    pub fn focused_view(&self) -> Option<&View> {
        self.focused_view.and_then(|id| self.views.get(&id))
    }

    /// Get the focused view mutably
    pub fn focused_view_mut(&mut self) -> Option<&mut View> {
        self.focused_view.and_then(|id| self.views.get_mut(&id))
    }

    /// Set the focused view
    pub fn set_focused_view(&mut self, view_id: ViewId) -> Result<(), String> {
        if !self.views.contains_key(&view_id) {
            return Err(format!("View {} not found", view_id));
        }

        // Unfocus previous view
        if let Some(prev_id) = self.focused_view {
            if let Some(view) = self.views.get_mut(&prev_id) {
                view.set_focused(false);
            }
        }

        // Focus new view
        if let Some(view) = self.views.get_mut(&view_id) {
            view.set_focused(true);
        }

        self.focused_view = Some(view_id);
        Ok(())
    }

    /// Get the root container
    pub fn root_container(&self) -> &SplitContainer {
        &self.root
    }

    /// Get split depth
    pub fn split_depth(&self) -> usize {
        self.root.depth()
    }

    /// Check if manager has a view
    pub fn has_view(&self, view_id: ViewId) -> bool {
        self.views.contains_key(&view_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view::View;

    #[test]
    fn test_split_container_single() {
        let view_id = ViewId::new_v4();
        let container = SplitContainer::new_single(view_id);

        assert!(container.is_single_view());
        assert!(!container.is_split());
        assert_eq!(container.view_count(), 1);
        assert_eq!(container.view_ids(), vec![view_id]);
        assert_eq!(container.depth(), 0);
    }

    #[test]
    fn test_split_container_split() {
        let view1 = ViewId::new_v4();
        let view2 = ViewId::new_v4();

        let container = SplitContainer::new_split(
            SplitDirection::Horizontal,
            SplitContainer::new_single(view1),
            SplitContainer::new_single(view2),
            0.5,
        );

        assert!(!container.is_single_view());
        assert!(container.is_split());
        assert_eq!(container.view_count(), 2);
        assert_eq!(container.view_ids(), vec![view1, view2]);
        assert_eq!(container.depth(), 1);
    }

    #[test]
    fn test_split_container_find_view() {
        let view1 = ViewId::new_v4();
        let view2 = ViewId::new_v4();
        let view3 = ViewId::new_v4();

        let container = SplitContainer::new_split(
            SplitDirection::Vertical,
            SplitContainer::new_single(view1),
            SplitContainer::new_single(view2),
            0.5,
        );

        assert!(container.find_view(view1));
        assert!(container.find_view(view2));
        assert!(!container.find_view(view3));
    }

    #[test]
    fn test_split_view_manager_creation() {
        let view = View::new();
        let view_id = view.id;

        let manager = SplitViewManager::new(view);

        assert_eq!(manager.view_count(), 1);
        assert_eq!(manager.focused_view_id(), Some(view_id));
        assert!(manager.has_view(view_id));
    }

    #[test]
    fn test_split_view_horizontal() {
        let view1 = View::new();
        let view1_id = view1.id;

        let mut manager = SplitViewManager::new(view1);

        let view2 = View::new();
        let view2_id = view2.id;

        let result = manager.split_view(view1_id, SplitDirection::Horizontal, view2);
        assert!(result.is_ok());

        assert_eq!(manager.view_count(), 2);
        assert!(manager.has_view(view1_id));
        assert!(manager.has_view(view2_id));
        assert_eq!(manager.split_depth(), 1);
    }

    #[test]
    fn test_split_view_vertical() {
        let view1 = View::new();
        let view1_id = view1.id;

        let mut manager = SplitViewManager::new(view1);

        let view2 = View::new();

        manager
            .split_view(view1_id, SplitDirection::Vertical, view2)
            .unwrap();

        assert_eq!(manager.view_count(), 2);
        assert_eq!(manager.split_depth(), 1);
    }

    #[test]
    fn test_multiple_splits() {
        let view1 = View::new();
        let view1_id = view1.id;

        let mut manager = SplitViewManager::new(view1);

        // Split horizontally
        let view2 = View::new();
        let view2_id = view2.id;
        manager
            .split_view(view1_id, SplitDirection::Horizontal, view2)
            .unwrap();

        // Split vertically
        let view3 = View::new();
        manager
            .split_view(view2_id, SplitDirection::Vertical, view3)
            .unwrap();

        assert_eq!(manager.view_count(), 3);
        assert_eq!(manager.split_depth(), 2);
    }

    #[test]
    fn test_close_view() {
        let view1 = View::new();
        let view1_id = view1.id;

        let mut manager = SplitViewManager::new(view1);

        let view2 = View::new();
        let view2_id = view2.id;
        manager
            .split_view(view1_id, SplitDirection::Horizontal, view2)
            .unwrap();

        // Close view2
        manager.close_view(view2_id).unwrap();

        assert_eq!(manager.view_count(), 1);
        assert!(!manager.has_view(view2_id));
        assert!(manager.has_view(view1_id));
    }

    #[test]
    fn test_cannot_close_last_view() {
        let view = View::new();
        let view_id = view.id;

        let mut manager = SplitViewManager::new(view);

        let result = manager.close_view(view_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_focused_view() {
        let view1 = View::new();
        let view1_id = view1.id;

        let mut manager = SplitViewManager::new(view1);

        let view2 = View::new();
        let view2_id = view2.id;
        manager
            .split_view(view1_id, SplitDirection::Horizontal, view2)
            .unwrap();

        // Initially view1 is focused
        assert_eq!(manager.focused_view_id(), Some(view1_id));

        // Focus view2
        manager.set_focused_view(view2_id).unwrap();
        assert_eq!(manager.focused_view_id(), Some(view2_id));

        // Check focus state
        assert!(manager.get_view(view2_id).unwrap().is_focused());
        assert!(!manager.get_view(view1_id).unwrap().is_focused());
    }

    #[test]
    fn test_get_views() {
        let view1 = View::new();
        let view1_id = view1.id;

        let mut manager = SplitViewManager::new(view1);

        let view2 = View::new();
        manager
            .split_view(view1_id, SplitDirection::Horizontal, view2)
            .unwrap();

        let views = manager.views();
        assert_eq!(views.len(), 2);
    }

    #[test]
    fn test_get_view_mut() {
        let view = View::new();
        let view_id = view.id;

        let mut manager = SplitViewManager::new(view);

        let view_mut = manager.get_view_mut(view_id).unwrap();
        view_mut.set_cursor(10, 5);

        assert_eq!(manager.get_view(view_id).unwrap().cursor(), (10, 5));
    }

    #[test]
    fn test_focused_view_mut() {
        let view = View::new();
        let view_id = view.id;

        let mut manager = SplitViewManager::new(view);

        let focused = manager.focused_view_mut().unwrap();
        focused.set_cursor(20, 10);

        assert_eq!(manager.get_view(view_id).unwrap().cursor(), (20, 10));
    }
}
