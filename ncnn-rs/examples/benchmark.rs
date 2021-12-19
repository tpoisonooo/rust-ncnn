use ncnn_rs::datareader::DataReader;
use ncnn_rs::mat::Mat;
use ncnn_rs::net::Net;
use ncnn_rs::option::Option as ncnn_option;
use std::time;

fn param_path() -> std::path::PathBuf {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path
}

fn benchmark(name: &str, mat_in: &Mat, opt: &ncnn_option, out: &str) {
    let mat_out = Mat::new();
    mat_in.fill(1.0 as f32);

    let net = Net::new();
    let path = param_path().join("../params").join(name);
    if !path.exists() {
        println!("vgg16 param not exist: {:?}", path);
        return;
    }

    net.set_option(opt);
    net.load_param(path.to_str().unwrap());
    let dr = DataReader::new();
    dr.use_empty_config();
    net.load_model_datareader(&dr);

    // warmup
    let ex_warmup = net.create_extractor();
    ex_warmup.input("data", &mat_in);
    ex_warmup.extract(out, &mat_out);

    let loop_cnt = 10;
    let now = time::Instant::now();
    for _ in 0..loop_cnt {
        let ex = net.create_extractor();
        ex.input("data", &mat_in);
        ex.extract(out, &mat_out);
    }
    let duration = now.elapsed().as_millis() / loop_cnt;
    println!("{} \t\t {} ms", name, duration);
}

fn main() {
    let opt = ncnn_option::new();
    opt.set_num_threads(1);

    let alloc = ncnn_rs::allocator::Allocator::new();

    benchmark(
        "squeezenet.param",
        &Mat::create_3d(227, 227, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "squeezenet_int8.param",
        &Mat::create_3d(227, 227, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "mobilenet.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "mobilenet_int8.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "mobilenet_v2.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "mobilenet_v3.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "shufflenet.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "shufflenet_v2.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "mnasnet.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "proxylessnasnet.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "efficientnet_b0.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    // benchmark("efficientnetv2_b0.param", &Mat::create_3d(224, 224, 3, &alloc), &opt, "output");

    benchmark(
        "regnety_400m.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "blazeface.param",
        &Mat::create_3d(128, 128, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "googlenet.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "googlenet_int8.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "resnet18.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "resnet18_int8.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "alexnet.param",
        &Mat::create_3d(227, 227, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "vgg16.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "vgg16_int8.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "resnet50.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "resnet50_int8.param",
        &Mat::create_3d(224, 224, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "squeezenet_ssd.param",
        &Mat::create_3d(300, 300, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "squeezenet_ssd_int8.param",
        &Mat::create_3d(300, 300, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "mobilenet_ssd.param",
        &Mat::create_3d(300, 300, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "mobilenet_ssd_int8.param",
        &Mat::create_3d(300, 300, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "mobilenet_yolo.param",
        &Mat::create_3d(416, 416, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "mobilenetv2_yolov3.param",
        &Mat::create_3d(352, 352, 3, &alloc),
        &opt,
        "output",
    );

    benchmark(
        "yolov4-tiny.param",
        &Mat::create_3d(416, 416, 3, &alloc),
        &opt,
        "output",
    );

    // benchmark("nanodet_m.param", &Mat::create_3d(320, 320, 3, &alloc), &opt, "output");
}
