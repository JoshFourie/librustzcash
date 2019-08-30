use std::sync::Arc;
use pairing::Engine;

<<<<<<< HEAD
use super::{Future, ParameterSource, AssignmentField, Result, source};
=======
use super::{Future, ParameterSource, ArcAssignment, Result, source};
>>>>>>> cc85ab246c5a9ca813760717d9600ccdf6bb4603

use crate::multiexp::{multiexp, FullDensity};
use crate::multicore::Worker;

pub struct Answer<E: Engine> {
    pub(super) a: E::G1,
    pub(super) b1: E::G1,
    pub(super) b2: E::G2
}

impl<E> Answer<E>
where
    E: Engine
{
    pub fn try_new<P: ParameterSource<E>>(
        worker: &Worker, 
        src: source::Answer<P,E>,
<<<<<<< HEAD
        input: AssignmentField<E>
=======
        input: ArcAssignment<E>
>>>>>>> cc85ab246c5a9ca813760717d9600ccdf6bb4603
    ) -> Result<Self> {
        let a: E::G1 = multiexp(
            &worker,
            src.a_input_src,
            FullDensity,
            input.clone(),
        ).wait()?;

        let b1: E::G1 = multiexp(
            &worker,
            src.b1_input_src,
            src.b_input_density.clone(),
            input.clone(),
        ).wait()?;

        let b2: E::G2 = multiexp(
            &worker,
            src.b2_input_src,
            src.b_input_density,
            input
        ).wait()?;

        Ok(Answer { a, b1, b2 })
    }
}

pub struct Auxiliary<E: Engine> {
    pub(super) a: E::G1,
    pub(super) b1: E::G1,
    pub(super) b2: E::G2,  
}

impl<E> Auxiliary<E> 
where
    E: Engine
{
    pub fn try_new<P>(
        worker: &Worker,
        src: source::Auxiliary<P,E>,
<<<<<<< HEAD
        assignment: AssignmentField<E>
=======
        assignment: ArcAssignment<E>
>>>>>>> cc85ab246c5a9ca813760717d9600ccdf6bb4603
    ) -> Result<Self> 
    where
        P: ParameterSource<E>
    {
        let a: _ = multiexp(
            &worker,
            src.a_aux_src,
            Arc::new(src.a_aux_density),
            assignment.clone(),
        ).wait()?;

        let b1: _ = multiexp(
            &worker,
            src.b1_aux_src,
            src.b_aux_density.clone(),
            assignment.clone(),
        ).wait()?;

        let b2 = multiexp(
            &worker, 
            src.b2_aux_src, 
            src.b_aux_density, 
            assignment
        ).wait()?;

        Ok(Auxiliary{ a, b1, b2 })
    }
}
