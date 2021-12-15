use ncnn_rs::mat::Mat;
use ncnn_rs::net::Net;
use ncnn_rs::net::Extractor;
use ncnn_rs::datareader::DataReader;
use ncnn_rs::allocator::Allocator;

fn param_path() -> std::path::PathBuf {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path
}

fn main() {
    let alloc = ncnn_rs::allocator::Allocator::new();
    let mat_in = Mat::create_3d(224, 224, 3, &alloc);
    let mat_out = Mat::create_1d(1000, &alloc);

    mat_in.fill(0.01 as f32);

    let net = Net::new();
    let path = param_path().join("../params/vgg16.param");
    if !path.exists() {
        println!("vgg16 param not exist: {:?}", path);
        return
    }

    net.load_param(path.to_str().unwrap());
    let dr = DataReader::new();
    dr.use_empty_config();
    net.load_model_datareader(&dr);

    let ex = net.create_extractor();
    ex.input("data", &mat_in);
    ex.extract("output", &mat_out);
}