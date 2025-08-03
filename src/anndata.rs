//! Traits defining each element of an AnnData file (e.g., X, obs, obsm, ...).
use crate::error::ParseError;

pub trait EnumInAnnData {
    fn parse(group: &hdf5_metno::Group) -> Result<Vec<Self>, ParseError>
    where
        Self: Sized;
}

pub trait ObsInAnnData {
    fn parse(group: &hdf5_metno::Group) -> Result<Self, ParseError>
    where
        Self: Sized;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait VarInAnnData {
    fn parse(group: &hdf5_metno::Group) -> Result<Self, ParseError>
    where
        Self: Sized;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait ObsmInAnnData {
    fn parse(group: &hdf5_metno::Group) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait VarmInAnnData {
    fn parse(group: &hdf5_metno::Group) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait ObspInAnnData {
    fn parse(group: &hdf5_metno::Group) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait VarmpInAnnData {
    fn parse(group: &hdf5_metno::Group) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait LayersInAnnData {
    fn parse(group: &hdf5_metno::Group) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait XInAnnData {
    fn parse(group: &hdf5_metno::Group, n_obs: usize, n_vars: usize) -> Result<Self, ParseError>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct AnnData<P, Q, R, S, T, U, V, W>
where
    P: ObsInAnnData + std::fmt::Debug + Clone,
    Q: VarInAnnData + std::fmt::Debug + Clone,
    R: ObsmInAnnData + std::fmt::Debug + Clone,
    S: VarmInAnnData + std::fmt::Debug + Clone,
    T: ObspInAnnData + std::fmt::Debug + Clone,
    U: VarmpInAnnData + std::fmt::Debug + Clone,
    V: LayersInAnnData + std::fmt::Debug + Clone,
    W: XInAnnData + std::fmt::Debug + Clone,
{
    pub obs: P,
    pub var: Q,
    pub obsm: Option<R>,
    pub varm: Option<S>,
    pub obsp: Option<T>,
    pub varmp: Option<U>,
    pub layers: Option<V>,
    pub x: W,
}

impl<P, Q, R, S, T, U, V, W> AnnData<P, Q, R, S, T, U, V, W>
where
    P: ObsInAnnData + std::fmt::Debug + Clone,
    Q: VarInAnnData + std::fmt::Debug + Clone,
    R: ObsmInAnnData + std::fmt::Debug + Clone,
    S: VarmInAnnData + std::fmt::Debug + Clone,
    T: ObspInAnnData + std::fmt::Debug + Clone,
    U: VarmpInAnnData + std::fmt::Debug + Clone,
    V: LayersInAnnData + std::fmt::Debug + Clone,
    W: XInAnnData + std::fmt::Debug + Clone,
{
    pub fn parse(contents: &hdf5_metno::File) -> Result<Self, ParseError> {
        let obs = contents
            .group("obs")
            .map_err(ParseError::Hdf5Error)
            .and_then(|obs| P::parse(&obs))?;
        let var = contents
            .group("var")
            .map_err(ParseError::Hdf5Error)
            .and_then(|var| Q::parse(&var))?;
        let obsm = contents
            .group("obsm")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| R::parse(&x))
            .ok();

        let varm = contents
            .group("varm")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| S::parse(&x))
            .ok();

        let obsp = contents
            .group("obsp")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| T::parse(&x))
            .ok();

        let varmp = contents
            .group("varmp")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| U::parse(&x))
            .ok();

        let layers = contents
            .group("layers")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| V::parse(&x))
            .ok();
        let n_obs = obs.len();
        let n_vars = var.len();
        let x = contents
            .group("X")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| W::parse(&x, n_obs, n_vars))?;
        Ok(Self {
            obs,
            var,
            obsm,
            varm,
            obsp,
            varmp,
            layers,
            x,
        })
    }
}
