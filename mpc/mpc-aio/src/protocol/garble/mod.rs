pub mod backend;
pub mod exec;
mod label;

use std::sync::Arc;

use async_trait::async_trait;
use mpc_circuits::Circuit;
use mpc_core::{
    garble::{gc_state, ActiveInputLabelsSet, CircuitOpening, FullInputLabelsSet, GarbledCircuit},
    msgs::garble::GarbleMessage,
};
use utils_aio::Channel;

use super::ot::OTError;

pub type GarbleChannel = Box<dyn Channel<GarbleMessage, Error = std::io::Error>>;

#[derive(Debug, thiserror::Error)]
pub enum GCError {
    #[error("core error")]
    CoreError(#[from] mpc_core::garble::Error),
    #[error("circuit error")]
    CircuitError(#[from] mpc_circuits::CircuitError),
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[error("ot error")]
    OTError(#[from] OTError),
    #[error("Received unexpected message: {0:?}")]
    Unexpected(GarbleMessage),
    #[error("backend error")]
    BackendError(String),
}

#[async_trait]
pub trait Generator {
    /// Asynchronously generate a garbled circuit
    async fn generate(
        &mut self,
        circ: Arc<Circuit>,
        input_labels: FullInputLabelsSet,
    ) -> Result<GarbledCircuit<gc_state::Full>, GCError>;
}

#[async_trait]
pub trait Evaluator {
    /// Asynchronously evaluate a garbled circuit
    async fn evaluate(
        &mut self,
        circ: GarbledCircuit<gc_state::Partial>,
        input_labels: ActiveInputLabelsSet,
    ) -> Result<GarbledCircuit<gc_state::Evaluated>, GCError>;
}

#[async_trait]
pub trait Validator {
    /// Asynchronously validate an evaluated garbled circuit
    async fn validate_evaluated(
        &mut self,
        circ: GarbledCircuit<gc_state::Evaluated>,
        opening: CircuitOpening,
    ) -> Result<GarbledCircuit<gc_state::Evaluated>, GCError>;

    /// Asynchronously validate a compress garbled circuit
    async fn validate_compressed(
        &mut self,
        circ: GarbledCircuit<gc_state::Compressed>,
        opening: CircuitOpening,
    ) -> Result<GarbledCircuit<gc_state::Compressed>, GCError>;
}

#[async_trait]
pub trait Compressor {
    /// Asynchronously compress an evaluated garbled circuit
    async fn compress(
        &mut self,
        circ: GarbledCircuit<gc_state::Evaluated>,
    ) -> Result<GarbledCircuit<gc_state::Compressed>, GCError>;
}