use crate::PatternIndex;
use bevy::prelude::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum NodeVar {
    SeenSinceStart,
    // ContinuedFrom,
    NTrues,
    NFalses,
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Gt,
    Lt,
    Mod,
    IntDiv,
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct Conditional {
    pub op: Operation,
    pub var: NodeVar,
    stats: CondStats,
    pub n: usize,
    pub goto: Point,
}

impl Conditional {
    fn is_true(&mut self) -> bool {
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

pub enum TrackerNodeData {
    Pattern(PatternIndex),
    Conditional(Conditional),
    Teleport(Point),
}

pub struct TrackerNode {
    pub node_dat: TrackerNodeData,
    next: Point,
}

impl TrackerNode {
    fn get_next(&mut self) -> Point {
        if let TrackerNodeData::Conditional(ref mut cond) = self.node_dat
            && cond.is_true()
        {
            cond.goto
        } else {
            self.next
        }
    }

    fn connect_to(&mut self, next: Point) {
        self.next = next;
    }
}

pub struct Graph {
    pub graph: [[Option<TrackerNode>; 256]; 256],
}
