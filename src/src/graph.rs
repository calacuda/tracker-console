use crate::{controls::MyGamepad, GraphSubState, PatternIndex, ScreenState, GRAPH_X, GRAPH_Y};
use bevy::prelude::*;
use std::{
    ops::{AddAssign, DerefMut, Index, IndexMut, SubAssign},
    time::{Duration, Instant},
};
use strum::{EnumIter, IntoEnumIterator};

pub trait Incrementable {
    fn get_big_n(&self) -> usize {
        16
    }

    fn small_inc(&mut self);
    fn big_inc(&mut self) {
        for _ in 0..self.get_big_n() {
            self.small_inc();
        }
    }

    fn small_dec(&mut self);
    fn big_dec(&mut self) {
        for _ in 0..self.get_big_n() {
            self.small_dec();
        }
    }
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum NodeVar {
    SeenSinceStart,
    // ContinuedFrom,
    NTrues,
    NFalses,
}

impl Incrementable for &mut NodeVar {
    fn get_big_n(&self) -> usize {
        1
    }

    fn small_inc(&mut self) {
        // **self = match *self {
        //     Operation::Mod => Operation::Lt,
        //     Operation::Lt => Operation::Gt,
        //     Operation::Gt => Operation::IntDiv,
        //     Operation::IntDiv => Operation::Mod,
        // }
        let all_ops: Vec<NodeVar> = NodeVar::iter().collect();
        let me = self.clone();

        for (i, op) in all_ops.iter().enumerate() {
            if *op == me {
                // info!("op and self are the same");
                **self = all_ops[(i + 1) % all_ops.len()];
            }
        }
    }

    fn small_dec(&mut self) {
        let all_ops: Vec<NodeVar> = NodeVar::iter().collect();
        let me = self.clone();

        for (i, op) in all_ops.iter().enumerate() {
            if *op == me {
                **self = all_ops[if i > 0 { i - 1 } else { all_ops.len() - 1 }];
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Operation {
    Mod,
    Lt,
    Gt,
    IntDiv,
}

impl Incrementable for &mut Operation {
    fn get_big_n(&self) -> usize {
        1
    }

    fn small_inc(&mut self) {
        // **self = match *self {
        //     Operation::Mod => Operation::Lt,
        //     Operation::Lt => Operation::Gt,
        //     Operation::Gt => Operation::IntDiv,
        //     Operation::IntDiv => Operation::Mod,
        // }
        let all_ops: Vec<Operation> = Operation::iter().collect();
        let me = self.clone();

        for (i, op) in all_ops.iter().enumerate() {
            if *op == me {
                // info!("op and self are the same");
                **self = all_ops[(i + 1) % all_ops.len()];
            }
        }
    }

    fn small_dec(&mut self) {
        let all_ops: Vec<Operation> = Operation::iter().collect();
        let me = self.clone();

        for (i, op) in all_ops.iter().enumerate() {
            if *op == me {
                **self = all_ops[if i > 0 { i - 1 } else { all_ops.len() - 1 }];
            }
        }
    }
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
                x: (x, true),
                y: (y, true),
            } => Self {
                node_dat: TrackerNodeData::Teleport(Point { x, y }),
                next: None,
            },
            PlacedPreNode::Conditional {
                op: (op, true),
                var: (var, true),
                goto: (goto, true),
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
        op: (Operation, bool),
        var: (NodeVar, bool),
        goto: (Point, bool),
    },
    Teleport {
        x: (usize, bool),
        y: (usize, bool),
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
            Self::Teleport { x, y } => x.1 && y.1,
            Self::Conditional { op, var, goto } => op.1 && var.1 && goto.1,
        }
    }
}

impl From<TrackerNodeType> for PlacedPreNode {
    fn from(value: TrackerNodeType) -> Self {
        match value {
            TrackerNodeType::Pattern => Self::Pattern(None),
            TrackerNodeType::Teleport => Self::Teleport {
                x: (0, false),
                y: (0, false),
            },
            TrackerNodeType::Conditional => Self::Conditional {
                op: (Operation::Mod, false),
                var: (NodeVar::SeenSinceStart, false),
                goto: (Point { x: 0, y: 0 }, false),
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
    pub display: Point,
    // pub node: MaybeTrackerNode,
    pub state: CursorState,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            pos: Point { x: 0, y: 0 },
            display: Point { x: 0, y: 0 },
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
            )
            .add_systems(
                Update,
                escape_tracker_node_args_set
                    .run_if(in_state(GraphSubState::EditNode))
                    .run_if(node_placed),
            )
            .add_systems(
                Update,
                set_conditional_goto
                    .run_if(in_state(GraphSubState::EditNode))
                    .run_if(node_placed),
            )
            .add_systems(OnEnter(ScreenState::Graph), default_cursor_state)
            .add_systems(OnEnter(GraphSubState::Neuteral), reset_display_cursor);
    }
}

fn node_placed(cursor: Res<Cursor>) -> bool {
    let CursorState::Holding(MaybeTrackerNode::PlacedPreNode(_)) = cursor.state else {
        return false;
    };

    true
}

fn reset_display_cursor(mut cursor: ResMut<Cursor>) {
    cursor.display.x = cursor.pos.x;
    cursor.display.y = cursor.pos.y;
}

fn set_conditional_goto(
    mut cursor: ResMut<Cursor>,
    mut display: ResMut<DisplayWindow>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut last_move: ResMut<LastMove>,
) {
    // TODO: write this by allowing the cursor to move.
    let CursorState::Holding(MaybeTrackerNode::PlacedPreNode(PlacedPreNode::Conditional {
        op: (_, true),
        var: (_, true),
        goto: (ref mut goto, false),
    })) = cursor.state
    else {
        return;
    };

    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    // move the cursor
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
            move_up(&mut display, &mut cursor, false)
        } else if button == 1 {
            // display.y += 1;
            move_down(&mut display, &mut cursor, false)
        } else if button == 2 {
            // display.x -= 1;
            move_left(&mut display, &mut cursor, false)
        } else if button == 3 {
            move_right(&mut display, &mut cursor, false)
            // display.x -= 1;
        } else {
            error!("uknown movement button pressed");
        }
    }
}

/// resets the cursor state
fn default_cursor_state(mut cursor: ResMut<Cursor>) {
    cursor.state = CursorState::None;
}

/// return to a neuteral GraphSubState state on the press of the b button
fn escape_tracker_node_args_set(
    // mut graph: ResMut<Graph>,
    cursor: Res<Cursor>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut next_state: ResMut<NextState<GraphSubState>>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let CursorState::Holding(MaybeTrackerNode::PlacedPreNode(_)) = cursor.state else {
        return;
    };

    let b = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::South,
    };

    if buttons.just_released(b) {
        // cursor.state = CursorState((*pre_node).into());
        next_state.set(GraphSubState::Neuteral);
    }
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

    if let PlacedPreNode::Pattern(ref mut pattern_i) = pre_node {
        set_pattern_args(gamepad, &buttons, pattern_i);
    }
    set_teleport_args(gamepad, &buttons, pre_node);
    set_conditional_args(gamepad, &buttons, pre_node);

    if pre_node.should_set() {
        graph[*cursor] = Some((*pre_node).into());
        next_state.set(GraphSubState::Neuteral)
    }
}

/// handles setting the arguements for a teleport node
fn set_teleport_args(
    gamepad: Gamepad,
    buttons: &Res<ButtonInput<GamepadButton>>,
    pre_node: &mut PlacedPreNode,
) {
    let to_set: (&mut usize, &mut bool) = if let PlacedPreNode::Teleport {
        x: (ref mut x, ref mut set),
        y: _,
    } = pre_node
        && !*set
    {
        (x, set)
    } else if let PlacedPreNode::Teleport {
        x: (_, true),
        y: (ref mut y, ref mut set),
    } = pre_node
        && !*set
    {
        (y, set)
    } else {
        return;
    };

    let a = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };

    if buttons.just_released(a) {
        *to_set.1 = true;
    }

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
        *to_set.0 += 1;
    }

    if buttons.just_released(down) {
        if *to_set.0 > 0 {
            *to_set.0 -= 1
        }
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
        if *to_set.0 > 15 {
            *to_set.0 -= 16
        }
    }

    if buttons.just_released(right) {
        *to_set.0 += 16;
    }
}

/// handles setting the arguements for a conditional node
fn set_conditional_args(
    gamepad: Gamepad,
    buttons: &Res<ButtonInput<GamepadButton>>,
    pre_node: &mut PlacedPreNode,
) {
    // TODO: write setting of conditional nodes
    // todo!("write set_conditional_args");
    let (mut to_set, complete): (Box<dyn Incrementable>, &mut bool) =
        if let PlacedPreNode::Conditional {
            op: (ref mut op, ref mut done),
            var: _,
            goto: _,
        } = pre_node
            && !*done
        {
            (Box::new(op), done)
        } else if let PlacedPreNode::Conditional {
            op: _,
            var: (ref mut var, ref mut done),
            goto: _,
        } = pre_node
            && !*done
        {
            (Box::new(var), done)
        } else if let PlacedPreNode::Conditional {
            op: _,
            var: _,
            goto: (ref mut goto, ref mut done),
        } = pre_node
            && !*done
        {
            // Box::new(goto.into())
            // set_conditional_goto(gamepad, buttons, goto, done);
            return;
        } else {
            return;
        };

    let a = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };

    if buttons.just_released(a) {
        *complete = true;
    }

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
        to_set.small_inc();
    }

    if buttons.just_released(down) {
        to_set.small_dec();
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
        // if *to_set.0 > 15 {
        to_set.big_dec()
        // }
    }

    if buttons.just_released(right) {
        to_set.big_inc();
    }
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
            move_up(&mut display, &mut cursor, true)
        } else if button == 1 {
            // display.y += 1;
            move_down(&mut display, &mut cursor, true)
        } else if button == 2 {
            // display.x -= 1;
            move_left(&mut display, &mut cursor, true)
        } else if button == 3 {
            move_right(&mut display, &mut cursor, true)
            // display.x -= 1;
        } else {
            error!("uknown movement button pressed");
        }
    }
}

fn move_up(display: &mut ResMut<DisplayWindow>, cursor: &mut ResMut<Cursor>, move_cursor: bool) {
    if cursor.pos.y > 0 {
        display.y -= 1;
        if move_cursor {
            cursor.pos.y -= 1;
        }
        cursor.display.y -= 1;
    }
}

fn move_down(display: &mut ResMut<DisplayWindow>, cursor: &mut ResMut<Cursor>, move_cursor: bool) {
    if cursor.pos.y < GRAPH_Y {
        display.y += 1;
        if move_cursor {
            cursor.pos.y += 1;
        }
        cursor.display.y += 1;
    }
}

fn move_left(display: &mut ResMut<DisplayWindow>, cursor: &mut ResMut<Cursor>, move_cursor: bool) {
    if cursor.pos.x > 0 {
        display.x -= 1;
        if move_cursor {
            cursor.pos.x -= 1;
        }
        cursor.display.x -= 1;
    }
}

fn move_right(display: &mut ResMut<DisplayWindow>, cursor: &mut ResMut<Cursor>, move_cursor: bool) {
    if cursor.pos.x < GRAPH_X {
        display.x += 1;
        if move_cursor {
            cursor.pos.x += 1;
        }
        cursor.display.x += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_operation_incrementable() {
        let mut op = Operation::Mod;
        (&mut op).small_inc();

        assert_eq!(
            op,
            Operation::Lt,
            "operation started as '{:?}' and ended as {op:?}. ({:?} was expected)",
            Operation::Mod,
            Operation::Lt
        );
        (&mut op).small_dec();
        assert_eq!(
            op,
            Operation::Mod,
            "operation started as '{:?}' and ended as {op:?}. ({:?} was expected)",
            Operation::Lt,
            Operation::Mod
        );
    }

    #[test]
    fn operation_incrementable() {
        let operations: Vec<Operation> = Operation::iter().collect();

        for i in 0..operations.len() {
            let op_bak = operations[i].clone();
            let mut op = operations[i].clone();
            (&mut op).small_inc();
            let should_be = operations[(i + 1) % operations.len()].clone();

            assert_eq!(op, should_be, "[INCREMENT FAILED] operation started as '{op_bak:?}' and ended as {op:?}. ({should_be:?} was expected)");
            let op_sav = op.clone();
            (&mut op).small_dec();
            assert_eq!(op, op_bak, "[RETURN DECREMENT FAILED] operation started as '{op_sav:?}' and ended as {op:?}. ({:?} was expected)", operations[i].clone());

            // let op_bak = operations[i].clone();
            let mut op = operations[i].clone();
            (&mut op).small_dec();
            let should_be = operations[if i > 0 { i - 1 } else { operations.len() - 1 }].clone();

            assert_eq!(op, should_be, "[DECREMENT FAILED] operation started as '{op_bak:?}' and ended as {op:?}. ({should_be:?} was expected)");
            let op_sav = op.clone();
            (&mut op).small_inc();
            assert_eq!(op, op_bak, "[RETURN INCREMENT FAILED] operation started as '{op_sav:?}' and ended as {op:?}. ({:?} was expected)", operations[i].clone());
        }
    }
}
