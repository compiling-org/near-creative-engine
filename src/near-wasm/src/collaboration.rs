//! On-chain collaboration features for creative sessions

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, Timestamp};
use std::collections::HashMap;

/// Live collaboration session
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CollaborationSession {
    pub session_id: String,
    pub creator: AccountId,
    pub participants: Vec<AccountId>,
    pub current_state: ToolState,
    pub patches: Vec<Patch>,
    pub permissions: PermissionMatrix,
    pub created_at: Timestamp,
    pub last_activity: Timestamp,
    pub is_active: bool,
}

/// Tool state for synchronization
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ToolState {
    pub tool_type: String,
    pub parameters: near_sdk::serde_json::Value,
    pub canvas_data: Vec<u8>,
    pub timeline_position: f32,
    pub version: u64,
}

/// Patch for collaborative editing
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Patch {
    pub id: String,
    pub author: AccountId,
    pub parent_patch: Option<String>,
    pub changes: Vec<StateChange>,
    pub timestamp: Timestamp,
    pub votes: i32,
    pub status: PatchStatus,
}

/// Individual state change
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StateChange {
    pub parameter_path: String,
    pub old_value: near_sdk::serde_json::Value,
    pub new_value: near_sdk::serde_json::Value,
    pub change_type: ChangeType,
}

/// Type of change
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ChangeType {
    ParameterUpdate,
    ToolSwitch,
    CanvasAction,
    TimelineSeek,
}

/// Patch approval status
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum PatchStatus {
    Draft,
    Proposed,
    Approved,
    Rejected,
    Merged,
}

/// Permission matrix for session access
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PermissionMatrix {
    pub can_edit: Vec<AccountId>,
    pub can_view: Vec<AccountId>,
    pub can_invite: Vec<AccountId>,
    pub can_merge: Vec<AccountId>,
}

/// Collaboration contract
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CollaborationContract {
    pub sessions: UnorderedMap<String, CollaborationSession>,
    pub user_sessions: LookupMap<AccountId, Vec<String>>,
    pub published_patches: UnorderedMap<String, Patch>,
    pub patch_votes: LookupMap<String, UnorderedMap<AccountId, bool>>, // patch_id -> (voter -> vote)
    pub owner_id: AccountId,
}

impl Default for CollaborationContract {
    fn default() -> Self {
        Self {
            sessions: UnorderedMap::new(b"s"),
            user_sessions: LookupMap::new(b"u"),
            published_patches: UnorderedMap::new(b"p"),
            patch_votes: LookupMap::new(b"v"),
            owner_id: env::predecessor_account_id(),
        }
    }
}

#[near_bindgen]
impl CollaborationContract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            sessions: UnorderedMap::new(b"s"),
            user_sessions: LookupMap::new(b"u"),
            published_patches: UnorderedMap::new(b"p"),
            patch_votes: LookupMap::new(b"v"),
            owner_id,
        }
    }

    /// Create a new collaboration session
    #[payable]
    pub fn create_session(
        &mut self,
        session_id: String,
        tool_type: String,
        initial_params: near_sdk::serde_json::Value,
    ) -> CollaborationSession {
        let creator = env::predecessor_account_id();

        // Check if session ID already exists
        assert!(self.sessions.get(&session_id).is_none(), "Session ID already exists");

        let session = CollaborationSession {
            session_id: session_id.clone(),
            creator: creator.clone(),
            participants: vec![creator.clone()],
            current_state: ToolState {
                tool_type,
                parameters: initial_params,
                canvas_data: Vec::new(),
                timeline_position: 0.0,
                version: 1,
            },
            patches: Vec::new(),
            permissions: PermissionMatrix {
                can_edit: vec![creator.clone()],
                can_view: vec![creator.clone()],
                can_invite: vec![creator.clone()],
                can_merge: vec![creator.clone()],
            },
            created_at: env::block_timestamp(),
            last_activity: env::block_timestamp(),
            is_active: true,
        };

        // Store session
        self.sessions.insert(&session_id, &session);

        // Update user's sessions
        let mut user_sessions = self.user_sessions.get(&creator).unwrap_or_default();
        user_sessions.push(session_id.clone());
        self.user_sessions.insert(&creator, &user_sessions);

        session
    }

    /// Join an existing collaboration session
    pub fn join_session(&mut self, session_id: String) -> bool {
        let user = env::predecessor_account_id();

        if let Some(mut session) = self.sessions.get(&session_id) {
            // Check if user can view the session
            if !session.permissions.can_view.contains(&user) {
                // Auto-add to viewers if session allows
                session.permissions.can_view.push(user.clone());
            }

            // Update last activity
            session.last_activity = env::block_timestamp();
            self.sessions.insert(&session_id, &session);

            // Update user's sessions
            let mut user_sessions = self.user_sessions.get(&user).unwrap_or_default();
            if !user_sessions.contains(&session_id) {
                user_sessions.push(session_id.clone());
                self.user_sessions.insert(&user, &user_sessions);
            }

            true
        } else {
            false
        }
    }

    /// Update session state (real-time sync)
    pub fn update_session_state(
        &mut self,
        session_id: String,
        new_state: ToolState,
        changes: Vec<StateChange>,
    ) {
        let user = env::predecessor_account_id();

        if let Some(mut session) = self.sessions.get(&session_id) {
            // Check edit permissions
            assert!(session.permissions.can_edit.contains(&user), "No edit permission");

            // Create patch from changes
            let patch = Patch {
                id: format!("{}_{}", session_id, env::block_timestamp()),
                author: user,
                parent_patch: session.patches.last().map(|p| p.id.clone()),
                changes,
                timestamp: env::block_timestamp(),
                votes: 0,
                status: PatchStatus::Draft,
            };

            // Update session state
            session.current_state = new_state;
            session.patches.push(patch);
            session.last_activity = env::block_timestamp();

            self.sessions.insert(&session_id, &session);
        } else {
            env::panic_str("Session not found");
        }
    }

    /// Propose a patch for community approval
    pub fn propose_patch(&mut self, session_id: String, patch_id: String) {
        let user = env::predecessor_account_id();

        if let Some(mut session) = self.sessions.get(&session_id) {
            // Find and update patch status
            if let Some(patch) = session.patches.iter_mut().find(|p| p.id == patch_id) {
                assert_eq!(patch.author, user, "Only patch author can propose");
                patch.status = PatchStatus::Proposed;
                self.sessions.insert(&session_id, &session);
            }
        }
    }

    /// Vote on a proposed patch
    pub fn vote_on_patch(&mut self, session_id: String, patch_id: String, approve: bool) {
        let voter = env::predecessor_account_id();

        if let Some(mut session) = self.sessions.get(&session_id) {
            if let Some(patch) = session.patches.iter_mut().find(|p| p.id == patch_id) {
                // Check if user already voted
                let mut votes = self.patch_votes.get(&patch_id).unwrap_or_else(|| UnorderedMap::new(b"pv"));
                if votes.get(&voter).is_some() {
                    env::panic_str("Already voted on this patch");
                }

                // Record vote
                votes.insert(&voter, &approve);
                self.patch_votes.insert(&patch_id, &votes);

                // Update vote count
                patch.votes += if approve { 1 } else { -1 };

                // Auto-merge if enough positive votes (simple majority)
                let total_participants = session.participants.len() as i32;
                if patch.votes > total_participants / 2 {
                    patch.status = PatchStatus::Approved;
                } else if patch.votes < -total_participants / 2 {
                    patch.status = PatchStatus::Rejected;
                }

                self.sessions.insert(&session_id, &session);
            }
        }
    }

    /// Merge an approved patch
    pub fn merge_patch(&mut self, session_id: String, patch_id: String) {
        let user = env::predecessor_account_id();

        if let Some(mut session) = self.sessions.get(&session_id) {
            // Check merge permissions
            assert!(session.permissions.can_merge.contains(&user), "No merge permission");

            if let Some(patch) = session.patches.iter_mut().find(|p| p.id == patch_id) {
                assert!(matches!(patch.status, PatchStatus::Approved), "Patch not approved");

                // Apply changes to current state
                for change in &patch.changes {
                    // In practice, this would apply the changes to the session state
                    // For now, just mark as merged
                }

                patch.status = PatchStatus::Merged;
                session.current_state.version += 1;

                self.sessions.insert(&session_id, &session);
            }
        }
    }

    /// Publish a patch to the global patch repository
    pub fn publish_patch(&mut self, session_id: String, patch_id: String) {
        let user = env::predecessor_account_id();

        if let Some(session) = self.sessions.get(&session_id) {
            if let Some(patch) = session.patches.iter().find(|p| p.id == patch_id) {
                assert_eq!(patch.author, user, "Only patch author can publish");
                assert!(matches!(patch.status, PatchStatus::Merged | PatchStatus::Approved),
                       "Patch must be approved or merged to publish");

                // Publish to global repository
                self.published_patches.insert(&patch_id, patch);
            }
        }
    }

    /// Get session information
    pub fn get_session(&self, session_id: String) -> Option<CollaborationSession> {
        self.sessions.get(&session_id)
    }

    /// Get user's sessions
    pub fn get_user_sessions(&self, account_id: AccountId) -> Vec<String> {
        self.user_sessions.get(&account_id).unwrap_or_default()
    }

    /// Get published patches
    pub fn get_published_patches(&self, limit: Option<u32>) -> Vec<Patch> {
        let limit = limit.unwrap_or(50) as usize;
        self.published_patches.values().take(limit).collect()
    }

    /// Invite user to session
    pub fn invite_to_session(&mut self, session_id: String, invitee: AccountId, can_edit: bool) {
        let inviter = env::predecessor_account_id();

        if let Some(mut session) = self.sessions.get(&session_id) {
            // Check invite permissions
            assert!(session.permissions.can_invite.contains(&inviter), "No invite permission");

            // Add to appropriate permission lists
            if can_edit && !session.permissions.can_edit.contains(&invitee) {
                session.permissions.can_edit.push(invitee.clone());
            }
            if !session.permissions.can_view.contains(&invitee) {
                session.permissions.can_view.push(invitee.clone());
            }

            // Add to participants if not already
            if !session.participants.contains(&invitee) {
                session.participants.push(invitee.clone());
            }

            self.sessions.insert(&session_id, &session);
        }
    }

    /// Leave session
    pub fn leave_session(&mut self, session_id: String) {
        let user = env::predecessor_account_id();

        if let Some(mut session) = self.sessions.get(&session_id) {
            // Remove from all permission lists
            session.permissions.can_edit.retain(|u| u != &user);
            session.permissions.can_view.retain(|u| u != &user);
            session.permissions.can_invite.retain(|u| u != &user);
            session.permissions.can_merge.retain(|u| u != &user);
            session.participants.retain(|u| u != &user);

            // If no participants left, mark as inactive
            if session.participants.is_empty() {
                session.is_active = false;
            }

            self.sessions.insert(&session_id, &session);

            // Update user's sessions
            let mut user_sessions = self.user_sessions.get(&user).unwrap_or_default();
            user_sessions.retain(|s| s != &session_id);
            self.user_sessions.insert(&user, &user_sessions);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id("contract.testnet".parse().unwrap());
        builder.signer_account_id("alice.testnet".parse().unwrap());
        builder.predecessor_account_id("alice.testnet".parse().unwrap());
        builder
    }

    #[test]
    fn test_create_session() {
        let context = get_context().build();
        testing_env!(context);

        let mut contract = CollaborationContract::default();

        let params = near_sdk::serde_json::json!({"iterations": 50, "zoom": 1.0});
        let session = contract.create_session(
            "test_session".to_string(),
            "fractal_shader".to_string(),
            params,
        );

        assert_eq!(session.session_id, "test_session");
        assert_eq!(session.creator, "alice.testnet".parse().unwrap());
        assert!(session.is_active);
    }

    #[test]
    fn test_join_session() {
        let context = get_context().build();
        testing_env!(context);

        let mut contract = CollaborationContract::default();

        // Create session
        let params = near_sdk::serde_json::json!({"test": true});
        contract.create_session(
            "test_session".to_string(),
            "test_tool".to_string(),
            params,
        );

        // Switch to different user
        let context2 = VMContextBuilder::new()
            .current_account_id("contract.testnet".parse().unwrap())
            .signer_account_id("bob.testnet".parse().unwrap())
            .predecessor_account_id("bob.testnet".parse().unwrap())
            .build();
        testing_env!(context2);

        // Join session
        let joined = contract.join_session("test_session".to_string());
        assert!(joined);
    }
}