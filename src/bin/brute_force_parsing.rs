///まずは一回、マクロを使わずに愚直にパーズを行う。

#[derive(Debug, Clone, Copy)]
pub enum LibraryInCompressed {
    #[allow(non_camel_case_types)]
    libA,
    #[allow(non_camel_case_types)]
    libB,
}

impl LibraryInCompressed {
    pub fn parse(group: &hdf5_metno::Group) -> Result<Vec<Self>, ParseError> {
        let categories = group.dataset("categories").map_err(ParseError::Hdf5Error)?;
        let cats_deserialized = categories
            .read_raw::<hdf5_metno::types::VarLenUnicode>()
            .map_err(ParseError::Hdf5Error)?;
        let cats_deserialized: Vec<&str> = cats_deserialized.iter().map(|x| x.as_str()).collect();
        let code_to_cat: std::collections::HashMap<usize, _> = cats_deserialized
            .into_iter()
            .enumerate()
            .map(|(index, cat)| match cat {
                "libA" => Ok((index, Self::libA)),
                "libB" => Ok((index, Self::libB)),
                _ => Err(ParseError::Other(format!("Unknown library: {cat}"))),
            })
            .collect::<Result<_, _>>()?;
        let codes = group
            .dataset("codes")
            .map_err(ParseError::Hdf5Error)?
            .read_raw::<usize>()
            .map_err(ParseError::Hdf5Error)?
            .into_iter()
            .map(|code| {
                code_to_cat
                    .get(&code)
                    .copied()
                    .ok_or_else(|| ParseError::Other(format!("Unknown code: {code}")))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(codes)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BatchInCompressed {
    #[allow(non_camel_case_types)]
    batch1,
    #[allow(non_camel_case_types)]
    batch2,
}

impl BatchInCompressed {
    pub fn parse(group: &hdf5_metno::Group) -> Result<Vec<Self>, ParseError> {
        let categories = group.dataset("categories").map_err(ParseError::Hdf5Error)?;
        let cats_deserialized = categories
            .read_raw::<hdf5_metno::types::VarLenUnicode>()
            .map_err(ParseError::Hdf5Error)?;
        let cats_deserialized: Vec<&str> = cats_deserialized.iter().map(|x| x.as_str()).collect();
        let batch_codes: std::collections::HashMap<usize, _> = cats_deserialized
            .into_iter()
            .enumerate()
            .map(|(index, cat)| match cat {
                "batch1" => Ok((index, Self::batch1)),
                "batch2" => Ok((index, Self::batch2)),
                _ => Err(ParseError::Other(format!("Unknown batch: {cat}"))),
            })
            .collect::<Result<_, _>>()?;
        let batches = group
            .dataset("codes")
            .map_err(ParseError::Hdf5Error)?
            .read_raw::<usize>()
            .map_err(ParseError::Hdf5Error)?
            .into_iter()
            .map(|code| {
                batch_codes
                    .get(&code)
                    .copied()
                    .ok_or_else(|| ParseError::Other(format!("Unknown code: {code}")))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(batches)
    }
}

#[derive(Debug, Clone)]
pub struct ObsInCompressed {
    pub library: Vec<LibraryInCompressed>,
    pub batch: Vec<BatchInCompressed>,
    pub index: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    Hdf5Error(hdf5_metno::Error),
    Other(String),
}

impl ObsInCompressed {
    pub fn len(&self) -> usize {
        self.index.len()
    }
    pub fn parse(group: &hdf5_metno::Group) -> Result<Self, ParseError> {
        let batches = group.group("batch").map_err(ParseError::Hdf5Error)?;
        let batch = BatchInCompressed::parse(&batches)?;
        let libraries = group.group("library").map_err(ParseError::Hdf5Error)?;
        let library = LibraryInCompressed::parse(&libraries)?;
        let index = group.dataset("_index").unwrap();
        let index: Vec<_> = index
            .read_raw::<hdf5_metno::types::VarLenUnicode>()
            .unwrap();
        let index: Vec<String> = index.iter().map(|x| x.to_string()).collect();
        let data_frame = Self {
            library,
            batch,
            index,
        };
        Ok(data_frame)
    }
}

#[derive(Debug, Clone)]
pub struct VarInCompressed {
    pub index: Vec<String>,
    pub genes: Vec<String>,
    pub is_hvg: Vec<bool>,
}

impl VarInCompressed {
    pub fn len(&self) -> usize {
        self.index.len()
    }
    pub fn parse(group: &hdf5_metno::Group) -> Result<Self, ParseError> {
        let index = group.dataset("_index").map_err(ParseError::Hdf5Error)?;
        let index: Vec<_> = index
            .read_raw::<hdf5_metno::types::VarLenUnicode>()
            .map_err(ParseError::Hdf5Error)?;
        let index: Vec<String> = index.iter().map(|x| x.to_string()).collect();
        let genes = group.dataset("genes").map_err(ParseError::Hdf5Error)?;
        let genes: Vec<_> = genes
            .read_raw::<hdf5_metno::types::VarLenUnicode>()
            .map_err(ParseError::Hdf5Error)?;
        let genes: Vec<String> = genes.iter().map(|x| x.to_string()).collect();
        let is_hvg = group.dataset("is_hvg").map_err(ParseError::Hdf5Error)?;
        let is_hvg: Vec<bool> = is_hvg.read_raw().map_err(ParseError::Hdf5Error)?;
        Ok(Self {
            index,
            genes,
            is_hvg,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct ObsmInCompressed {}

impl ObsmInCompressed {
    pub fn parse(_group: &hdf5_metno::Group) -> Result<Self, ParseError> {
        // Placeholder for actual parsing logic
        Ok(Self {})
    }
}

#[derive(Debug, Clone, Default)]
pub struct VarmInCompressed {}

impl VarmInCompressed {
    pub fn parse(_group: &hdf5_metno::Group) -> Result<Self, ParseError> {
        // Placeholder for actual parsing logic
        Ok(Self {})
    }
}

#[derive(Debug, Clone, Default)]
pub struct ObspInCompressed {}
impl ObspInCompressed {
    pub fn parse(_group: &hdf5_metno::Group) -> Result<Self, ParseError> {
        // Placeholder for actual parsing logic
        Ok(Self {})
    }
}

#[derive(Debug, Clone, Default)]
pub struct VarmpInCompressed {}
impl VarmpInCompressed {
    pub fn parse(_group: &hdf5_metno::Group) -> Result<Self, ParseError> {
        // Placeholder for actual parsing logic
        Ok(Self {})
    }
}

#[derive(Debug, Clone, Default)]
pub struct LayersInCompressed {}

impl LayersInCompressed {
    pub fn parse(_group: &hdf5_metno::Group) -> Result<Self, ParseError> {
        // Placeholder for actual parsing logic
        Ok(Self {})
    }
}

// The default implementation for Count is a dense array --- hmm?
// maybe it is up to users to define the count structure, but it is not so easy
// to create a "derive" macro when the compiler can not understand the representation
// it should be. -- dense or row/col sparse?
#[derive(Debug, Clone)]
pub struct Count(Vec<Vec<f32>>);

impl Count {
    pub fn parse(
        group: &hdf5_metno::Group,
        n_obs: usize,
        n_vars: usize,
    ) -> Result<Self, ParseError> {
        let mut counts = vec![vec![0f32; n_vars]; n_obs];
        let data = group
            .dataset("data")
            .and_then(|d| d.read_raw::<f32>())
            .map_err(ParseError::Hdf5Error)?;
        let indices = group
            .dataset("indices")
            .and_then(|d| d.read_raw::<u32>())
            .map_err(ParseError::Hdf5Error)?;
        let indptr = group
            .dataset("indptr")
            .and_then(|d| d.read_raw::<u32>())
            .map_err(ParseError::Hdf5Error)?;
        assert_eq!(data.len(), indices.len());
        for i in 0..indptr.len() - 1 {
            let start = indptr[i] as usize;
            let end = indptr[i + 1] as usize;
            for j in start..end {
                let col = indices[j] as usize;
                let value = data[j];
                counts[i][col] = value;
            }
        }
        Ok(Self(counts))
    }
}

#[derive(Debug, Clone)]
pub struct AnnData {
    pub obs: ObsInCompressed,
    pub var: VarInCompressed,
    pub obsm: ObsmInCompressed,
    pub varm: VarmInCompressed,
    pub obsp: ObspInCompressed,
    pub varmp: VarmpInCompressed,
    pub layers: LayersInCompressed,
    pub x: Count,
}

impl AnnData {
    pub fn parse(contents: &hdf5_metno::File) -> Result<Self, ParseError> {
        let obs = contents
            .group("obs")
            .map_err(ParseError::Hdf5Error)
            .and_then(|obs| ObsInCompressed::parse(&obs))?;
        let var = contents
            .group("var")
            .map_err(ParseError::Hdf5Error)
            .and_then(|var| VarInCompressed::parse(&var))?;
        let obsm = contents
            .group("obsm")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| ObsmInCompressed::parse(&x))
            .unwrap_or_default();
        let varm = contents
            .group("varm")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| VarmInCompressed::parse(&x))
            .unwrap_or_default();
        let obsp = contents
            .group("obsp")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| ObspInCompressed::parse(&x))
            .unwrap_or_default();
        let varmp = contents
            .group("varmp")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| VarmpInCompressed::parse(&x))
            .unwrap_or_default();
        let layers = contents
            .group("layers")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| LayersInCompressed::parse(&x))
            .unwrap_or_default();
        let n_obs = obs.len();
        let n_vars = var.len();
        let x = contents
            .group("X")
            .map_err(ParseError::Hdf5Error)
            .and_then(|x| Count::parse(&x, n_obs, n_vars))?;
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

fn main() -> std::io::Result<()> {
    let file = "./static/compressed.h5ad";
    let contents = hdf5_metno::File::open(file).unwrap();
    let adata = AnnData::parse(&contents).unwrap();
    println!("{:?}", adata);
    for count in adata.x.0.iter().take(10) {
        println!("{:?}", count.iter().sum::<f32>());
    }
    Ok(())
}
