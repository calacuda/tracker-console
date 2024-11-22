use crate::{controls::MyGamepad, GraphSubState, PatternIndex, ScreenState, GRAPH_X, GRAPH_Y};
use bevy::prelude::*;
use std::{
    ops::{AddAssign, DerefMut, Index, IndexMut, SubAssign},
    time::{Duration, Instant},
};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Resource, Clone, Copy, Debug)]
pub struct LastMove(usize, Instant);

impl Default for LastMove {
    fn default() -> Self {
        Self(5, Instant::now())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeVar {
    SeenSinceStart,
    // ContinuedFrom,
    NTrues,
    NFalses,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    Gt,
    Lt,
    Mod,
    IntDiv,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CondStats {
    pub seen: usize,
    // pub continued_from: usize,
    pub trues: usize,
    pub falses: usize,
}

impl Index<NodeVar> for CondStats {
    type Output = usize;

    fn index(&self, index: NodeVar) -> &Self::Output {
        match index {
            NodeVar::SeenSinceStart => &self.seen,
            // NodeVar::ContinuedFrom => &self.continued_from,
            NodeVar::NTrues => &self.trues,
            NodeVar::NFalses => &self.falses,
        }
    }
}

impl IndexMut<NodeVar> for CondStats {
    fn index_mut(&mut self, index: NodeVar) -> &mut Self::Output {
        match index {
            NodeVar::SeenSinceStart => &mut self.seen,
            // NodeVar::ContinuedFrom => &mut self.continued_from,
            NodeVar::NTrues => &mut self.trues,
            NodeVar::NFalses => &mut self.falses,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Conditional {
    pub op: Operation,
    pub var: NodeVar,
    stats: CondStats,
    pub n: usize,
    pub goto: Point,
}

impl Conditional {
    pub fn is_true(&mut self) -> bool {
        let var = self.stats[self.var];

        let res = match self.op {
            Operation::Gt => var > self.n,
            Operation::Lt => var < self.n,
            Operation::Mod => (var % self.n) > 0,
            Operation::IntDiv => (var / self.n) > 0,
        };

        if res {
            self.stats.trues += 1;
        } else {
            self.stats.falses += 1;
        }

        self.stats.seen += 1;

        res
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrackerNodeData {
    Pattern(PatternIndex),
    Conditional(Conditional),
    Teleport(Point),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum TrackerNodeType {
    Pattern,
    Conditional,
    Teleport,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrackerNode {
    pub node_dat: TrackerNodeData,
    next: Option<Point>,
    // pub in_flux: bool,
}

impl From<PlacedPreNode> for TrackerNode {
    fn from(value: PlacedPreNode) -> Self {
        match value {
            PlacedPreNode::Pattern(Some(i)) => Self {
                node_dat: TrackerNodeData::Pattern(i),
                next: None,
            },
            PlacedPreNode::Teleport {
                x: Some(x),
                y: Some(y),
            } => Self {
                node_dat: TrackerNodeData::Teleport(Point { x, y }),
                next: None,
            },
            PlacedPreNode::Conditional {
                op: Some(op),
                var: Some(var),
                goto: Some(goto),
            } => Self {
                node_dat: TrackerNodeData::Conditional(Conditional {
                    op,
                    var,
                    stats: CondStats::default(),
                    n: 0,
                    goto,
                }),
                next: None,
            },
            _ => {
                error!("can't create Tracker Node from a \"PlacedPreNode\" with None values");

                Self {
                    node_dat: TrackerNodeData::Pattern(0),
                    next: None,
                }
            }
        }
    }
}

// impl Default for TrackerNode {
//     fn default() -> Self {
//         Self {
//             node_dat: TrackerNodeData::Pattern(),
//         }
//     }
// }

impl TrackerNode {
    pub fn get_next(&mut self) -> Option<Point> {
        if let TrackerNodeData::Conditional(ref mut cond) = self.node_dat
            && !cond.is_true()
        {
            Some(cond.goto)
        } else {
            self.next
        }
    }

    pub fn connect_to(&mut self, next: Option<Point>) {
        self.next = next;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlacedPreNode {
    Pattern(Option<PatternIndex>),
    Conditional {
        op: Option<Operation>,
        var: Option<NodeVar>,
        goto: Option<Point>,
    },
    Teleport {
        x: Option<usize>,
        y: Option<usize>,
    },
}

impl PlacedPreNode {
    // /// returns true if the node is fully set
    // fn can_set(&self) -> bool {
    //     match self {
    //         Self::Pattern(i) => i.is_some(),
    //         Self::Teleport { x, y } => x.is_some() && y.is_some(),
    //         Self::Conditional { op, var, goto: _ } => op.is_some() && var.is_some(),
    //     }
    // }

    fn should_set(&self) -> bool {
        match self {
            Self::Pattern(i) => i.is_some(),
            Self::Teleport { x, y } => x.is_some() && y.is_some(),
            Self::Conditional { op, var, goto } => op.is_some() && var.is_some() && goto.is_some(),
        }
    }
}

impl From<TrackerNodeType> for PlacedPreNode {
    fn from(value: TrackerNodeType) -> Self {
        match value {
            TrackerNodeType::Pattern => Self::Pattern(None),
            TrackerNodeType::Teleport => Self::Teleport { x: None, y: None },
            TrackerNodeType::Conditional => Self::Conditional {
                op: None,
                var: None,
                goto: None,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaybeTrackerNode {
    ConcreteNode(TrackerNode),
    PreNode(TrackerNodeType),
    PlacedPreNode(PlacedPreNode),
    // None,
}

impl MaybeTrackerNode {
    // fn is_none(&self) -> bool {
    //     *self == Self::None
    // }

    pub fn is_pre(&self) -> bool {
        let Self::PreNode(_) = self else {
            return false;
        };

        return true;
    }

    pub fn is_set(&self) -> bool {
        let Self::ConcreteNode(_) = self else {
            return false;
        };

        return true;
    }
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct Graph {
    pub graph: [[Option<TrackerNode>; GRAPH_Y]; GRAPH_X],
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            graph: [[None; GRAPH_Y]; GRAPH_X],
        }
    }
}

impl Index<Cursor> for Graph {
    type Output = Option<TrackerNode>;

    fn index(&self, index: Cursor) -> &Self::Output {
        &self.graph[index.pos.x][index.pos.y]
    }
}

impl IndexMut<Cursor> for Graph {
    // type Output = Option<TrackerNode>;

    fn index_mut(&mut self, index: Cursor) -> &mut Self::Output {
        &mut self.graph[index.pos.x][index.pos.y]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub min: isize,
    pub max: isize,
}

impl AddAssign<isize> for Range {
    // type Output = Self;

    fn add_assign(&mut self, rhs: isize) {
        // Self {
        //     min: self.min + rhs,
        //     max: self.max + rhs,
        // }
        self.min += rhs;
        self.max += rhs;
    }
}

impl SubAssign<isize> for Range {
    // type Output = Self;

    fn sub_assign(&mut self, rhs: isize) {
        // Self {
        //     min: self.min - rhs,
        //     max: self.max - rhs,
        // }
        self.min -= rhs;
        self.max -= rhs;
    }
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct DisplayWindow {
    pub x: Range,
    pub y: Range,
}

impl Default for DisplayWindow {
    fn default() -> Self {
        Self {
            x: Range { min: 0, max: 24 },
            y: Range { min: 0, max: 16 },
        }
    }
}

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CursorState {
    /// connecting the stored point to where ever it gets placed.
    Connecting(Point),
    /// moving arround a pre-tracker node or tracker node
    Holding(MaybeTrackerNode),
    None,
}

impl CursorState {
    fn is_none(&self) -> bool {
        *self == Self::None
    }

    // fn is_none(&self) -> bool {
    //     *self == Self::None
    // }
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct Cursor {
    pub pos: Point,
    // pub node: MaybeTrackerNode,
    pub state: CursorState,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            pos: Point { x: 0, y: 0 },
            // node: MaybeTrackerNode,
            state: CursorState::None,
        }
    }
}

pub struct GraphStatePlugin;

impl Plugin for GraphStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Graph>()
            .init_resource::<Cursor>()
            .init_resource::<DisplayWindow>()
            .init_resource::<LastMove>()
            .add_systems(
                Update,
                move_cursor
                    .run_if(in_state(ScreenState::Graph))
                    .run_if(not(node_placed)),
            )
            .add_systems(
                Update,
                add_tracker_node.run_if(in_state(ScreenState::Graph)),
            )
            .add_systems(
                Update,
                pick_tracker_node
                    .run_if(in_state(GraphSubState::EditNode))
                    .run_if(not(node_placed)),
            )
            .add_systems(
                Update,
                place_tracker_node
                    .run_if(in_state(GraphSubState::EditNode))
                    .run_if(not(node_placed)),
            )
            .add_systems(
                Update,
                set_tracker_node_args
                    .run_if(in_state(GraphSubState::EditNode))
                    .run_if(node_placed),
            );
    }
}

fn node_placed(cursor: Res<Cursor>) -> bool {
    let CursorState::Holding(MaybeTrackerNode::PlacedPreNode(_)) = cursor.state else {
        return false;
    };

    true
}

fn set_tracker_node_args(
    mut graph: ResMut<Graph>,
    mut cursor: ResMut<Cursor>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut next_state: ResMut<NextState<GraphSubState>>,
) {
    let CursorState::Holding(MaybeTrackerNode::PlacedPreNode(ref mut pre_node)) = cursor.state
    else {
        return;
    };

    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    // TODO: or together the function calls
    let should_set = match pre_node {
        PlacedPreNode::Pattern(ref mut pattern_i) => set_pattern_args(gamepad, &buttons, pattern_i),
        PlacedPreNode::Teleport { x: _, y: _ } => {
            todo!("teleport editing not written")
        } // set_teleport_args(gamepad, &buttons, pre_node),
        PlacedPreNode::Conditional {
            op: _,
            var: _,
            goto: _,
        } => {
            todo!("Conditional editing not yet written")
        } // set_teleport_args(gamepad, &buttons, pre_node),
    };

    if should_set {
        graph[*cursor] = Some((*pre_node).into());
        next_state.set(GraphSubState::Neuteral)
    }
}

fn set_teleport_args(
    gamepad: Gamepad,
    buttons: &Res<ButtonInput<GamepadButton>>,
    pre_node: &mut PlacedPreNode,
) -> bool {
}

fn set_pattern_args(
    gamepad: Gamepad,
    buttons: &Res<ButtonInput<GamepadButton>>,
    pattern_i: &mut Option<PatternIndex>,
) -> bool {
    if pattern_i.is_none() {
        *pattern_i = Some(0);
        return false;
    };

    let a = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };

    // up/down = +/- 1
    let up = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadUp,
    };
    let down = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadDown,
    };

    if buttons.just_released(up) {
        pattern_i.map(|ref mut i| *i += 1);
    }

    if buttons.just_released(down) {
        pattern_i.map(|ref mut i| {
            if *i > 0 {
                *i -= 1
            }
        });
    }

    // left/right = +/- 16
    let left = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadLeft,
    };
    let right = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadRight,
    };

    if buttons.just_released(left) {
        pattern_i.map(|ref mut i| {
            if *i > 15 {
                *i -= 16
            }
        });
    }

    if buttons.just_released(right) {
        pattern_i.map(|ref mut i| *i += 16);
    }

    buttons.just_released(a) && pattern_i.is_some()
}

/// places the tracker node held in the cursor and transitions to connection mode
fn place_tracker_node(
    mut cursor: ResMut<Cursor>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    // mut next_state: ResMut<NextState<GraphSubState>>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let a = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };

    if let CursorState::Holding(MaybeTrackerNode::PreNode(node_type)) = cursor.state.clone()
        && buttons.just_released(a)
    {
        cursor.state = CursorState::Holding(MaybeTrackerNode::PlacedPreNode(node_type.into()));
        // next_state.set(GraphSubState::EditArgs);
    }
}

fn pick_tracker_node(
    mut cursor: ResMut<Cursor>,
    // mut graph: ResMut<Graph>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    // mut next_state: ResMut<NextState<GraphSubState>>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let l_bumper = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::LeftTrigger,
    };
    let r_bumper = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::RightTrigger,
    };

    let pressed = [buttons.pressed(l_bumper), buttons.pressed(r_bumper)];

    if pressed.iter().filter(|b| **b).count() != 1 {
        // more then one dir button pressed
        return;
    }

    if let CursorState::Holding(MaybeTrackerNode::PreNode(node_type)) = cursor.state.clone()
        && buttons.just_released(l_bumper)
    {
        // shifts the tracker node type "down"
        let node_types: Vec<TrackerNodeType> = TrackerNodeType::iter().collect();
        let i = node_types.iter().position(|t| t == &node_type).unwrap();

        if i == 0 {
            cursor.state =
                CursorState::Holding(MaybeTrackerNode::PreNode(node_types[node_types.len() - 1]));
        } else {
            cursor.state = CursorState::Holding(MaybeTrackerNode::PreNode(node_types[i - 1]));
        }
    } else if let CursorState::Holding(MaybeTrackerNode::PreNode(node_type)) = cursor.state.clone()
        && buttons.just_released(r_bumper)
    {
        // shifts the tracker node type "up"
        let node_types: Vec<TrackerNodeType> = TrackerNodeType::iter().collect();
        let i = node_types.iter().position(|t| t == &node_type).unwrap();

        if i == node_types.len() - 1 {
            cursor.state = CursorState::Holding(MaybeTrackerNode::PreNode(node_types[0]));
        } else {
            cursor.state = CursorState::Holding(MaybeTrackerNode::PreNode(node_types[i + 1]));
        }
    }
}

/// adds a tracker node for
fn add_tracker_node(
    // display: Res<DisplayWindow>,
    mut cursor: ResMut<Cursor>,
    mut graph: ResMut<Graph>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    // mut last_move: ResMut<LastMove>,
    mut next_state: ResMut<NextState<GraphSubState>>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let a = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };

    if buttons.just_pressed(a) && graph.clone()[cursor.clone()].is_none() && cursor.state.is_none()
    {
        info!(
            "init new tracker node at location ({}, {})",
            cursor.pos.x, cursor.pos.y
        );
        // graph.deref_mut()[cursor.clone()] = Some(TrackerNode::default())
        cursor.state = CursorState::Holding(MaybeTrackerNode::PreNode(TrackerNodeType::Pattern));
        // enter "setting node" State
        next_state.set(GraphSubState::EditNode);
    } else if let CursorState::Holding(MaybeTrackerNode::ConcreteNode(node)) = cursor.state.clone()
        && buttons.just_pressed(a)
        && graph.clone()[cursor.clone()].is_none()
    {
        // place tracker node
        info!(
            "placing new tracker node at location ({}, {})",
            cursor.pos.x, cursor.pos.y
        );
        graph.deref_mut()[cursor.clone()] = Some(node);

        cursor.state = CursorState::None;
        next_state.set(GraphSubState::Neuteral);
    }
    // else if let CursorState::Holding(MaybeTrackerNode::PreNode(node_type)) = cursor.state.clone()
    //     && buttons.just_pressed(a)
    //     && graph.clone()[cursor.clone()].is_none()
    // {
}

fn move_cursor(
    mut display: ResMut<DisplayWindow>,
    mut cursor: ResMut<Cursor>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut last_move: ResMut<LastMove>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let up = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadUp,
    };
    let down = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadDown,
    };
    let left = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadLeft,
    };
    let right = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadRight,
    };

    let pressed = [
        buttons.pressed(up),
        buttons.pressed(down),
        buttons.pressed(left),
        buttons.pressed(right),
    ];

    if pressed.iter().filter(|b| **b).count() != 1 {
        // more then one dir button pressed
        return;
    }

    if let Some(button) = pressed.iter().position(|b| *b)
        && ((last_move.0 == button && last_move.1.elapsed() > Duration::from_secs_f64(0.2))
            || (last_move.0 != button))
    {
        last_move.1 = Instant::now();

        if button == 0 {
            // display.y -= 1;
            move_up(&mut display, &mut cursor)
        } else if button == 1 {
            // display.y += 1;
            move_down(&mut display, &mut cursor)
        } else if button == 2 {
            // display.x -= 1;
            move_left(&mut display, &mut cursor)
        } else if button == 3 {
            move_right(&mut display, &mut cursor)
            // display.x -= 1;
        } else {
            error!("uknown movement button pressed");
        }
    }
}

fn move_up(display: &mut ResMut<DisplayWindow>, cursor: &mut ResMut<Cursor>) {
    if cursor.pos.y > 0 {
        display.y -= 1;
        cursor.pos.y -= 1;
    }
}

fn move_down(display: &mut ResMut<DisplayWindow>, cursor: &mut ResMut<Cursor>) {
    if cursor.pos.y < GRAPH_Y {
        display.y += 1;
        cursor.pos.y += 1;
    }
}

fn move_left(display: &mut ResMut<DisplayWindow>, cursor: &mut ResMut<Cursor>) {
    if cursor.pos.x > 0 {
        display.x -= 1;
        cursor.pos.x -= 1;
    }
}

fn move_right(display: &mut ResMut<DisplayWindow>, cursor: &mut ResMut<Cursor>) {
    if cursor.pos.x < GRAPH_X {
        display.x += 1;
        cursor.pos.x += 1;
    }
}
