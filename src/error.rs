#[derive(Debug, Clone)]
pub enum ParseError {
    Hdf5Error(hdf5_metno::Error),
    Other(String),
}
