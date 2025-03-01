#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::path::PathBuf;
use std::str::FromStr;

use clap::ArgEnum;
use color_eyre::Result;
use serde::Deserialize;
use serde::Serialize;
use strum::Display;
use strum::EnumString;

pub use arrangement::Arrangement;
pub use arrangement::Axis;
pub use custom_layout::CustomLayout;
pub use cycle_direction::CycleDirection;
pub use default_layout::DefaultLayout;
pub use direction::Direction;
pub use layout::Layout;
pub use operation_direction::OperationDirection;
pub use rect::Rect;

pub mod arrangement;
pub mod custom_layout;
pub mod cycle_direction;
pub mod default_layout;
pub mod direction;
pub mod layout;
pub mod operation_direction;
pub mod rect;

#[derive(Clone, Debug, Serialize, Deserialize, Display)]
#[serde(tag = "type", content = "content")]
pub enum SocketMessage {
    // Window / Container Commands
    FocusWindow(OperationDirection),
    MoveWindow(OperationDirection),
    CycleFocusWindow(CycleDirection),
    CycleMoveWindow(CycleDirection),
    StackWindow(OperationDirection),
    ResizeWindowEdge(OperationDirection, Sizing),
    ResizeWindowAxis(Axis, Sizing),
    UnstackWindow,
    CycleStack(CycleDirection),
    MoveContainerToMonitorNumber(usize),
    MoveContainerToWorkspaceNumber(usize),
    SendContainerToMonitorNumber(usize),
    SendContainerToWorkspaceNumber(usize),
    MoveWorkspaceToMonitorNumber(usize),
    Promote,
    ToggleFloat,
    ToggleMonocle,
    ToggleMaximize,
    ToggleWindowContainerBehaviour,
    WindowHidingBehaviour(HidingBehaviour),
    // Current Workspace Commands
    ManageFocusedWindow,
    UnmanageFocusedWindow,
    AdjustContainerPadding(Sizing, i32),
    AdjustWorkspacePadding(Sizing, i32),
    ChangeLayout(DefaultLayout),
    ChangeLayoutCustom(PathBuf),
    FlipLayout(Axis),
    // Monitor and Workspace Commands
    EnsureWorkspaces(usize, usize),
    NewWorkspace,
    ToggleTiling,
    Stop,
    TogglePause,
    Retile,
    QuickSave,
    QuickLoad,
    Save(PathBuf),
    Load(PathBuf),
    CycleFocusMonitor(CycleDirection),
    CycleFocusWorkspace(CycleDirection),
    FocusMonitorNumber(usize),
    FocusWorkspaceNumber(usize),
    FocusMonitorWorkspaceNumber(usize, usize),
    ContainerPadding(usize, usize, i32),
    WorkspacePadding(usize, usize, i32),
    WorkspaceTiling(usize, usize, bool),
    WorkspaceName(usize, usize, String),
    WorkspaceLayout(usize, usize, DefaultLayout),
    WorkspaceLayoutCustom(usize, usize, PathBuf),
    // Configuration
    ReloadConfiguration,
    WatchConfiguration(bool),
    InvisibleBorders(Rect),
    WorkAreaOffset(Rect),
    ResizeDelta(i32),
    WorkspaceRule(ApplicationIdentifier, String, usize, usize),
    FloatRule(ApplicationIdentifier, String),
    ManageRule(ApplicationIdentifier, String),
    IdentifyTrayApplication(ApplicationIdentifier, String),
    IdentifyBorderOverflow(ApplicationIdentifier, String),
    State,
    Query(StateQuery),
    FocusFollowsMouse(FocusFollowsMouseImplementation, bool),
    ToggleFocusFollowsMouse(FocusFollowsMouseImplementation),
    MouseFollowsFocus(bool),
    ToggleMouseFollowsFocus,
    AddSubscriber(String),
    RemoveSubscriber(String),
}

impl SocketMessage {
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_string(self)?.as_bytes().to_vec())
    }
}

impl FromStr for SocketMessage {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Display, EnumString, ArgEnum)]
#[strum(serialize_all = "snake_case")]
pub enum StateQuery {
    FocusedMonitorIndex,
    FocusedWorkspaceIndex,
    FocusedContainerIndex,
    FocusedWindowIndex,
}

#[derive(Clone, Debug, Serialize, Deserialize, Display, EnumString, ArgEnum)]
#[strum(serialize_all = "snake_case")]
pub enum ApplicationIdentifier {
    Exe,
    Class,
    Title,
}

#[derive(Clone, Debug, Serialize, Deserialize, Display, EnumString, ArgEnum)]
#[strum(serialize_all = "snake_case")]
pub enum FocusFollowsMouseImplementation {
    Komorebi,
    Windows,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Display, EnumString, ArgEnum)]
#[strum(serialize_all = "snake_case")]
pub enum WindowContainerBehaviour {
    Create,
    Append,
}

#[derive(Clone, Debug, Serialize, Deserialize, Display, EnumString, ArgEnum)]
#[strum(serialize_all = "snake_case")]
pub enum HidingBehaviour {
    Hide,
    Minimize,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Display, EnumString, ArgEnum)]
#[strum(serialize_all = "snake_case")]
pub enum Sizing {
    Increase,
    Decrease,
}

impl Sizing {
    #[must_use]
    pub const fn adjust_by(&self, value: i32, adjustment: i32) -> i32 {
        match self {
            Sizing::Increase => value + adjustment,
            Sizing::Decrease => {
                if value > 0 && value - adjustment >= 0 {
                    value - adjustment
                } else {
                    value
                }
            }
        }
    }
}
