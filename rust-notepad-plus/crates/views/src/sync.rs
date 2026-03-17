//! View synchronization system

use crate::view::ViewId;
use std::collections::{HashMap, HashSet};

/// What to synchronize between views
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyncMode {
    /// Synchronize scroll position
    Scroll,

    /// Synchronize cursor position
    Cursor,

    /// Synchronize selection
    Selection,

    /// Synchronize all (scroll, cursor, selection)
    All,
}

/// Sync group identifier
pub type SyncGroupId = uuid::Uuid;

/// A synchronization group that links multiple views
#[derive(Debug, Clone)]
pub struct SyncGroup {
    /// Unique group identifier
    pub id: SyncGroupId,

    /// Name of the sync group
    pub name: String,

    /// Views in this group
    pub views: HashSet<ViewId>,

    /// Synchronization modes enabled
    pub sync_modes: HashSet<SyncMode>,
}

impl SyncGroup {
    /// Create a new sync group
    pub fn new(name: String) -> Self {
        Self {
            id: SyncGroupId::new_v4(),
            name,
            views: HashSet::new(),
            sync_modes: HashSet::new(),
        }
    }

    /// Add a view to the group
    pub fn add_view(&mut self, view_id: ViewId) {
        self.views.insert(view_id);
    }

    /// Remove a view from the group
    pub fn remove_view(&mut self, view_id: ViewId) -> bool {
        self.views.remove(&view_id)
    }

    /// Check if a view is in the group
    pub fn has_view(&self, view_id: ViewId) -> bool {
        self.views.contains(&view_id)
    }

    /// Get the number of views in the group
    pub fn view_count(&self) -> usize {
        self.views.len()
    }

    /// Enable a sync mode
    pub fn enable_sync(&mut self, mode: SyncMode) {
        if mode == SyncMode::All {
            self.sync_modes.insert(SyncMode::Scroll);
            self.sync_modes.insert(SyncMode::Cursor);
            self.sync_modes.insert(SyncMode::Selection);
        } else {
            self.sync_modes.insert(mode);
        }
    }

    /// Disable a sync mode
    pub fn disable_sync(&mut self, mode: SyncMode) {
        if mode == SyncMode::All {
            self.sync_modes.clear();
        } else {
            self.sync_modes.remove(&mode);
        }
    }

    /// Check if a sync mode is enabled
    pub fn is_sync_enabled(&self, mode: SyncMode) -> bool {
        match mode {
            SyncMode::All => {
                self.sync_modes.contains(&SyncMode::Scroll)
                    && self.sync_modes.contains(&SyncMode::Cursor)
                    && self.sync_modes.contains(&SyncMode::Selection)
            }
            _ => self.sync_modes.contains(&mode),
        }
    }

    /// Check if group is empty
    pub fn is_empty(&self) -> bool {
        self.views.is_empty()
    }
}

/// Manages view synchronization
pub struct ViewSynchronizer {
    /// All sync groups
    groups: HashMap<SyncGroupId, SyncGroup>,

    /// Index: view ID -> group IDs
    view_groups: HashMap<ViewId, HashSet<SyncGroupId>>,
}

impl ViewSynchronizer {
    /// Create a new view synchronizer
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            view_groups: HashMap::new(),
        }
    }

    /// Create a new sync group
    pub fn create_group(&mut self, name: String, sync_modes: Vec<SyncMode>) -> SyncGroupId {
        let mut group = SyncGroup::new(name);

        for mode in sync_modes {
            group.enable_sync(mode);
        }

        let group_id = group.id;
        self.groups.insert(group_id, group);

        log::info!("Created sync group: {}", group_id);
        group_id
    }

    /// Add a view to a sync group
    pub fn add_to_group(&mut self, view_id: ViewId, group_id: SyncGroupId) -> Result<(), String> {
        let group = self
            .groups
            .get_mut(&group_id)
            .ok_or_else(|| format!("Sync group {} not found", group_id))?;

        group.add_view(view_id);

        self.view_groups
            .entry(view_id)
            .or_insert_with(HashSet::new)
            .insert(group_id);

        log::debug!("Added view {} to sync group {}", view_id, group_id);
        Ok(())
    }

    /// Remove a view from a sync group
    pub fn remove_from_group(
        &mut self,
        view_id: ViewId,
        group_id: SyncGroupId,
    ) -> Result<(), String> {
        let group = self
            .groups
            .get_mut(&group_id)
            .ok_or_else(|| format!("Sync group {} not found", group_id))?;

        if !group.remove_view(view_id) {
            return Err(format!(
                "View {} not in sync group {}",
                view_id, group_id
            ));
        }

        if let Some(groups) = self.view_groups.get_mut(&view_id) {
            groups.remove(&group_id);
        }

        // Remove empty group
        if group.is_empty() {
            self.groups.remove(&group_id);
            log::info!("Removed empty sync group: {}", group_id);
        }

        Ok(())
    }

    /// Remove a view from all sync groups
    pub fn remove_view(&mut self, view_id: ViewId) {
        if let Some(group_ids) = self.view_groups.remove(&view_id) {
            for group_id in group_ids {
                if let Some(group) = self.groups.get_mut(&group_id) {
                    group.remove_view(view_id);

                    // Remove empty group
                    if group.is_empty() {
                        self.groups.remove(&group_id);
                    }
                }
            }
        }
    }

    /// Delete a sync group
    pub fn delete_group(&mut self, group_id: SyncGroupId) -> Result<(), String> {
        let group = self
            .groups
            .remove(&group_id)
            .ok_or_else(|| format!("Sync group {} not found", group_id))?;

        // Remove from view_groups index
        for view_id in group.views {
            if let Some(groups) = self.view_groups.get_mut(&view_id) {
                groups.remove(&group_id);
            }
        }

        log::info!("Deleted sync group: {}", group_id);
        Ok(())
    }

    /// Get all groups a view belongs to
    pub fn get_view_groups(&self, view_id: ViewId) -> Vec<&SyncGroup> {
        if let Some(group_ids) = self.view_groups.get(&view_id) {
            group_ids
                .iter()
                .filter_map(|id| self.groups.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get a sync group by ID
    pub fn get_group(&self, group_id: SyncGroupId) -> Option<&SyncGroup> {
        self.groups.get(&group_id)
    }

    /// Get a mutable sync group by ID
    pub fn get_group_mut(&mut self, group_id: SyncGroupId) -> Option<&mut SyncGroup> {
        self.groups.get_mut(&group_id)
    }

    /// Get all sync groups
    pub fn groups(&self) -> Vec<&SyncGroup> {
        self.groups.values().collect()
    }

    /// Get the number of sync groups
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    /// Get all views that should be synchronized with the given view for a specific mode
    pub fn get_synced_views(&self, view_id: ViewId, mode: SyncMode) -> HashSet<ViewId> {
        let mut synced_views = HashSet::new();

        if let Some(group_ids) = self.view_groups.get(&view_id) {
            for group_id in group_ids {
                if let Some(group) = self.groups.get(group_id) {
                    if group.is_sync_enabled(mode) {
                        for &other_view_id in &group.views {
                            if other_view_id != view_id {
                                synced_views.insert(other_view_id);
                            }
                        }
                    }
                }
            }
        }

        synced_views
    }

    /// Check if two views are synchronized for a specific mode
    pub fn are_views_synced(&self, view1: ViewId, view2: ViewId, mode: SyncMode) -> bool {
        if let Some(group_ids) = self.view_groups.get(&view1) {
            for group_id in group_ids {
                if let Some(group) = self.groups.get(group_id) {
                    if group.has_view(view2) && group.is_sync_enabled(mode) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Enable a sync mode for a group
    pub fn enable_sync_mode(
        &mut self,
        group_id: SyncGroupId,
        mode: SyncMode,
    ) -> Result<(), String> {
        let group = self
            .groups
            .get_mut(&group_id)
            .ok_or_else(|| format!("Sync group {} not found", group_id))?;

        group.enable_sync(mode);
        Ok(())
    }

    /// Disable a sync mode for a group
    pub fn disable_sync_mode(
        &mut self,
        group_id: SyncGroupId,
        mode: SyncMode,
    ) -> Result<(), String> {
        let group = self
            .groups
            .get_mut(&group_id)
            .ok_or_else(|| format!("Sync group {} not found", group_id))?;

        group.disable_sync(mode);
        Ok(())
    }
}

impl Default for ViewSynchronizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_group_creation() {
        let group = SyncGroup::new("Test Group".to_string());
        assert_eq!(group.name, "Test Group");
        assert!(group.is_empty());
        assert_eq!(group.view_count(), 0);
    }

    #[test]
    fn test_sync_group_add_remove_view() {
        let mut group = SyncGroup::new("Test".to_string());
        let view_id = ViewId::new_v4();

        group.add_view(view_id);
        assert!(group.has_view(view_id));
        assert_eq!(group.view_count(), 1);

        assert!(group.remove_view(view_id));
        assert!(!group.has_view(view_id));
        assert!(group.is_empty());
    }

    #[test]
    fn test_sync_modes() {
        let mut group = SyncGroup::new("Test".to_string());

        assert!(!group.is_sync_enabled(SyncMode::Scroll));

        group.enable_sync(SyncMode::Scroll);
        assert!(group.is_sync_enabled(SyncMode::Scroll));

        group.disable_sync(SyncMode::Scroll);
        assert!(!group.is_sync_enabled(SyncMode::Scroll));
    }

    #[test]
    fn test_sync_mode_all() {
        let mut group = SyncGroup::new("Test".to_string());

        group.enable_sync(SyncMode::All);
        assert!(group.is_sync_enabled(SyncMode::Scroll));
        assert!(group.is_sync_enabled(SyncMode::Cursor));
        assert!(group.is_sync_enabled(SyncMode::Selection));
        assert!(group.is_sync_enabled(SyncMode::All));

        group.disable_sync(SyncMode::All);
        assert!(!group.is_sync_enabled(SyncMode::Scroll));
        assert!(!group.is_sync_enabled(SyncMode::All));
    }

    #[test]
    fn test_synchronizer_creation() {
        let sync = ViewSynchronizer::new();
        assert_eq!(sync.group_count(), 0);
    }

    #[test]
    fn test_create_group() {
        let mut sync = ViewSynchronizer::new();

        let group_id = sync.create_group("Test".to_string(), vec![SyncMode::Scroll]);

        assert_eq!(sync.group_count(), 1);
        assert!(sync.get_group(group_id).is_some());
    }

    #[test]
    fn test_add_to_group() {
        let mut sync = ViewSynchronizer::new();
        let group_id = sync.create_group("Test".to_string(), vec![]);

        let view_id = ViewId::new_v4();
        sync.add_to_group(view_id, group_id).unwrap();

        let group = sync.get_group(group_id).unwrap();
        assert!(group.has_view(view_id));
    }

    #[test]
    fn test_remove_from_group() {
        let mut sync = ViewSynchronizer::new();
        let group_id = sync.create_group("Test".to_string(), vec![]);

        let view_id = ViewId::new_v4();
        sync.add_to_group(view_id, group_id).unwrap();
        sync.remove_from_group(view_id, group_id).unwrap();

        // Group should be automatically deleted when empty
        assert!(sync.get_group(group_id).is_none());
    }

    #[test]
    fn test_remove_view() {
        let mut sync = ViewSynchronizer::new();
        let group1 = sync.create_group("Group1".to_string(), vec![]);
        let group2 = sync.create_group("Group2".to_string(), vec![]);

        let view_id = ViewId::new_v4();
        sync.add_to_group(view_id, group1).unwrap();
        sync.add_to_group(view_id, group2).unwrap();

        sync.remove_view(view_id);

        // Both groups should be deleted
        assert_eq!(sync.group_count(), 0);
    }

    #[test]
    fn test_get_view_groups() {
        let mut sync = ViewSynchronizer::new();
        let group1 = sync.create_group("Group1".to_string(), vec![]);
        let group2 = sync.create_group("Group2".to_string(), vec![]);

        let view_id = ViewId::new_v4();
        sync.add_to_group(view_id, group1).unwrap();
        sync.add_to_group(view_id, group2).unwrap();

        let groups = sync.get_view_groups(view_id);
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn test_get_synced_views() {
        let mut sync = ViewSynchronizer::new();
        let group_id = sync.create_group("Test".to_string(), vec![SyncMode::Scroll]);

        let view1 = ViewId::new_v4();
        let view2 = ViewId::new_v4();
        let view3 = ViewId::new_v4();

        sync.add_to_group(view1, group_id).unwrap();
        sync.add_to_group(view2, group_id).unwrap();
        sync.add_to_group(view3, group_id).unwrap();

        let synced = sync.get_synced_views(view1, SyncMode::Scroll);
        assert_eq!(synced.len(), 2);
        assert!(synced.contains(&view2));
        assert!(synced.contains(&view3));
        assert!(!synced.contains(&view1)); // Should not include self
    }

    #[test]
    fn test_are_views_synced() {
        let mut sync = ViewSynchronizer::new();
        let group_id = sync.create_group("Test".to_string(), vec![SyncMode::Scroll]);

        let view1 = ViewId::new_v4();
        let view2 = ViewId::new_v4();
        let view3 = ViewId::new_v4();

        sync.add_to_group(view1, group_id).unwrap();
        sync.add_to_group(view2, group_id).unwrap();

        assert!(sync.are_views_synced(view1, view2, SyncMode::Scroll));
        assert!(!sync.are_views_synced(view1, view3, SyncMode::Scroll));
        assert!(!sync.are_views_synced(view1, view2, SyncMode::Cursor)); // Not enabled
    }

    #[test]
    fn test_enable_disable_sync_mode() {
        let mut sync = ViewSynchronizer::new();
        let group_id = sync.create_group("Test".to_string(), vec![]);

        sync.enable_sync_mode(group_id, SyncMode::Scroll).unwrap();
        assert!(sync
            .get_group(group_id)
            .unwrap()
            .is_sync_enabled(SyncMode::Scroll));

        sync.disable_sync_mode(group_id, SyncMode::Scroll).unwrap();
        assert!(!sync
            .get_group(group_id)
            .unwrap()
            .is_sync_enabled(SyncMode::Scroll));
    }

    #[test]
    fn test_delete_group() {
        let mut sync = ViewSynchronizer::new();
        let group_id = sync.create_group("Test".to_string(), vec![]);

        let view_id = ViewId::new_v4();
        sync.add_to_group(view_id, group_id).unwrap();

        sync.delete_group(group_id).unwrap();

        assert_eq!(sync.group_count(), 0);
        assert_eq!(sync.get_view_groups(view_id).len(), 0);
    }

    #[test]
    fn test_multiple_groups_per_view() {
        let mut sync = ViewSynchronizer::new();
        let group1 = sync.create_group("Group1".to_string(), vec![SyncMode::Scroll]);
        let group2 = sync.create_group("Group2".to_string(), vec![SyncMode::Cursor]);

        let view_id = ViewId::new_v4();
        sync.add_to_group(view_id, group1).unwrap();
        sync.add_to_group(view_id, group2).unwrap();

        let groups = sync.get_view_groups(view_id);
        assert_eq!(groups.len(), 2);
    }
}
