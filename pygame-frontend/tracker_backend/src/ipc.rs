use crate::pygame_coms::{InputCMD, State};
use anyhow::Result;
use bevy::{log::*, prelude::Resource};
use crossbeam::channel::{unbounded, Receiver, Sender};
use pyo3::prelude::*;

#[pyclass(module = "tracker_backend")]
#[derive(Debug, Clone)]
pub struct TrackerIPC {
    pub rx: Receiver<State>,
    pub tx: Sender<InputCMD>,
}

#[derive(Debug, Clone, Resource)]
pub struct RustIPC {
    pub rx: Receiver<InputCMD>,
    pub tx: Sender<State>,
}

#[pymethods]
impl TrackerIPC {
    fn recv(&self) -> Option<State> {
        match self.rx.try_recv() {
            Ok(cmd) => Some(cmd),
            Err(_e) => {
                // error!("{e}");
                None
            }
        }
    }

    fn recv_all(&self) -> Vec<State> {
        let mut v = Vec::with_capacity(self.rx.len());

        while let Some(cmd) = self.recv() {
            info!("{cmd:?}");
            v.push(cmd);
        }

        v
    }

    fn send(&self, cmd: InputCMD) {
        match self.tx.send(cmd) {
            Ok(_) => {}
            Err(e) => error!("{e}"),
        }
    }
}

impl RustIPC {
    pub fn len(&self) -> usize {
        self.rx.len()
    }

    pub fn recv_msg(&self) -> Option<InputCMD> {
        self.rx.try_recv().map_or(None, |inbox_elm| Some(inbox_elm))
    }

    pub fn send_msg(&self, state: State) -> Result<()> {
        Ok(self.tx.send(state)?)
    }
}

// impl<RX, TX> RTX<RX, TX> {
//     pub fn new(rx: Receiver<RX>, tx: Sender<TX>) -> Self {
//         Self { rx, tx }
//     }
// }
//
// pub fn gen_com_pairs<RX, TX>() -> (RTX<RX, TX>, RTX<TX, RX>) {
//     let (tx, rx) = unbounded::<RX>();
//     let (tx_2, rx_2) = unbounded::<TX>();
//
//     (RTX::new(rx, tx_2), RTX::new(rx_2, tx))
// }

pub fn gen_ipc() -> (RustIPC, TrackerIPC) {
    let (tx, rx) = unbounded();
    let (tx_2, rx_2) = unbounded();

    (RustIPC { rx, tx: tx_2 }, TrackerIPC { rx: rx_2, tx })
}
