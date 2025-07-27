fn main() {
    let file = "./static/compressed.h5ad";
    let contents = hdf5_metno::File::open(file).unwrap();
    println!("{contents:?}");
    let obs = contents.group("obs").unwrap();
    println!("{:?}", obs);
    println!("{:?}", obs.member_names().unwrap());
    println!("{:?}", obs.group("batch").unwrap());
    let batch = obs.group("batch").unwrap();
    if let Ok(gns) = batch.member_names() {
        for gn in gns {
            println!("{gn:?}");
        }
    }
    println!("{:?}", batch.dataset("categories").unwrap());
    let cats = batch.dataset("categories").unwrap();
    let dtype = cats.dtype().unwrap();
    println!("{dtype:?}");
    println!("{:?}", dtype.size());
    println!("{:?}", dtype.to_descriptor().unwrap());
    println!("{:?}", cats.shape());
    let cats_deserialized = cats.read_raw::<hdf5_metno::types::VarLenUnicode>().unwrap();
    let cats_deserialized: Vec<&str> = cats_deserialized.iter().map(|x| x.as_str()).collect();
    println!("{:?}", cats_deserialized);
    let actual_codes = batch.dataset("codes").unwrap();
    let actual_codes: Vec<u32> = actual_codes.read_raw().unwrap();
    println!("{:?}", actual_codes);
    println!("============");
    let batch = obs.group("batch").unwrap();
    println!("{:?}", batch);
    let index = obs.dataset("_index").unwrap();
    let index: Vec<_> = index
        .read_raw::<hdf5_metno::types::VarLenUnicode>()
        .unwrap();
    let index: Vec<String> = index.iter().map(|x| x.to_string()).collect();
    println!("{:?}", index);
    println!("===============");
    println!("{:?}", contents.member_names());
    let counts = contents.group("X").unwrap();
    println!("{:?}", counts.member_names());
    let data = counts.dataset("data").unwrap();
    let data_de: Vec<f32> = data.read_raw().unwrap();
    println!("{:?}", data.dtype().unwrap().to_descriptor().unwrap());
    println!("{:?}", &data_de[0..10]);
    let indices = counts.dataset("indices").unwrap();
    let indices_de = indices.read_raw::<u32>().unwrap();
    println!("{:?}", indices.dtype().unwrap().to_descriptor().unwrap());
    println!("{:?}", &indices_de[0..10]);
    let indptr = counts.dataset("indptr").unwrap();
    let indptr_de = indptr.read_raw::<u32>().unwrap();
    println!("{:?}", indptr.dtype().unwrap().to_descriptor().unwrap());
    println!("{:?}", &indptr_de[0..10]);
}
