use crate::anndata::*;
//まずは一回、マクロを使わずに愚直にパーズを行う。
use crate::error::ParseError;
// これは具象構造体なので、ユーザーが定める。これから自動的にコードを生成するのが課題。
#[derive(Debug, Clone, Copy)]
pub enum LibraryInCompressed {
    #[allow(non_camel_case_types)]
    libA,
    #[allow(non_camel_case_types)]
    libB,
}

// 最終的にはマクロを使わない実装方法も提供しないとだめなので、トレイトからパーズを行うことにする。
impl EnumInAnnData for LibraryInCompressed {
    fn parse(group: &hdf5_metno::Group) -> Result<Vec<Self>, ParseError> {
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

impl EnumInAnnData for BatchInCompressed {
    fn parse(group: &hdf5_metno::Group) -> Result<Vec<Self>, ParseError> {
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

impl ObsInAnnData for ObsInCompressed {
    fn len(&self) -> usize {
        self.index.len()
    }

    fn parse(group: &hdf5_metno::Group) -> Result<Self, ParseError> {
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

impl VarInAnnData for VarInCompressed {
    fn parse(group: &hdf5_metno::Group) -> Result<Self, ParseError> {
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
    fn len(&self) -> usize {
        self.index.len()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ObsmInCompressed {}

impl ObsmInAnnData for ObsmInCompressed {
    fn parse(_group: &hdf5_metno::Group) -> Result<Self, ParseError> {
        // Placeholder for actual parsing logic
        Ok(Self {})
    }
}

#[derive(Debug, Clone, Default)]
pub struct VarmInCompressed {}

impl VarmInAnnData for VarmInCompressed {
    fn parse(_group: &hdf5_metno::Group) -> Result<Self, ParseError> {
        // Placeholder for actual parsing logic
        Ok(Self {})
    }
}

#[derive(Debug, Clone, Default)]
pub struct ObspInCompressed {}
impl ObspInAnnData for ObspInCompressed {
    fn parse(_group: &hdf5_metno::Group) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}

#[derive(Debug, Clone, Default)]
pub struct VarmpInCompressed {}

impl VarmpInAnnData for VarmpInCompressed {
    fn parse(_group: &hdf5_metno::Group) -> Result<Self, ParseError> {
        // Placeholder for actual parsing logic
        Ok(Self {})
    }
}

#[derive(Debug, Clone, Default)]
pub struct LayersInCompressed {}

impl LayersInAnnData for LayersInCompressed {
    fn parse(_group: &hdf5_metno::Group) -> Result<Self, ParseError> {
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

impl XInAnnData for Count {
    fn parse(group: &hdf5_metno::Group, n_obs: usize, n_vars: usize) -> Result<Self, ParseError> {
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

pub type AnnDataCompressed = AnnData<
    ObsInCompressed,
    VarInCompressed,
    ObsmInCompressed,
    VarmInCompressed,
    ObspInCompressed,
    VarmpInCompressed,
    LayersInCompressed,
    Count,
>;

#[cfg(test)]
mod tests {
    const TEST_FILE: &str = "./static/compressed.h5ad";
    fn open_test_file() -> hdf5_metno::File {
        hdf5_metno::File::open(TEST_FILE).unwrap()
    }
    use super::*;
    #[test]
    fn test_parse() {
        // 結局パーズできるか見る
        let contents = open_test_file();
        assert!(AnnDataCompressed::parse(&contents).is_ok());
    }
    #[test]
    fn test_parse_content() {
        let contents = open_test_file();
        let (n_obs, n_vars) = (100, 20);
        assert!(Count::parse(&contents.group("X").unwrap(), n_obs, n_vars).is_ok());
    }
    #[test]
    fn test_parse_enum() {
        let contents = open_test_file();
        let group = contents.group("obs/library").unwrap();
        assert!(LibraryInCompressed::parse(&group).is_ok());
        let group = contents.group("obs/batch").unwrap();
        assert!(BatchInCompressed::parse(&group).is_ok());
    }
    #[test]
    fn test_parse_obs() {
        let contents = open_test_file();
        let group = contents.group("obs").unwrap();
        assert!(ObsInCompressed::parse(&group).is_ok());
    }
    #[test]
    fn test_parse_var() {
        let contents = open_test_file();
        let group = contents.group("var").unwrap();
        assert!(VarInCompressed::parse(&group).is_ok());
    }
    #[test]
    fn test_parse_layer() {
        let contents = open_test_file();
        let group = contents.group("layers").unwrap();
        assert!(LayersInCompressed::parse(&group).is_ok());
    }
}
