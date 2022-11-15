use ncnn_rs::DataReader;
use ncnn_rs::Mat;
use ncnn_rs::Net;
use ncnn_rs::Option as ncnn_option;
use std::time;

fn param_path() -> std::path::PathBuf {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path
}

fn benchmark(name: &str, mut mat_in: Mat, opt: &ncnn_option, out: &str) -> anyhow::Result<()> {
    let mut mat_out = Mat::new();
    mat_in.fill(1.0 as f32);

    let mut net = Net::new();
    let path = param_path().join("../params").join(name);
    if !path.exists() {
        anyhow::bail!("param not found: {:?}", path)
    }

    net.set_option(opt);
    net.load_param(path.to_str().unwrap())?;
    let dr = DataReader::empty();
    net.load_model_datareader(&dr)?;

    // warmup
    let mut ex_warmup = net.create_extractor();
    ex_warmup.input("data", &mat_in)?;
    ex_warmup.extract(out, &mut mat_out)?;

    let loop_cnt = 10;
    let now = time::Instant::now();
    for _ in 0..loop_cnt {
        let mut ex = net.create_extractor();
        ex.input("data", &mat_in)?;
        ex.extract(out, &mut mat_out)?;
    }
    let duration = now.elapsed().as_millis() / loop_cnt;
    println!("{} \t\t {} ms", name, duration);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let opt = ncnn_option::new();
    // opt.set_num_threads(8);
    // opt.set_vulkan_compute(false);

    benchmark(
        "squeezenet.param",
        Mat::new_3d(227, 227, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "squeezenet_int8.param",
        Mat::new_3d(227, 227, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "mobilenet.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "mobilenet_int8.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "mobilenet_v2.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "mobilenet_v3.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "shufflenet.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "shufflenet_v2.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "mnasnet.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "proxylessnasnet.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "efficientnet_b0.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    // benchmark("efficientnetv2_b0.param", Mat::new_3d(224, 224, 3, None), &opt, "output")?;

    benchmark(
        "regnety_400m.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "blazeface.param",
        Mat::new_3d(128, 128, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "googlenet.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "googlenet_int8.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "resnet18.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "resnet18_int8.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "alexnet.param",
        Mat::new_3d(227, 227, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "vgg16.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "vgg16_int8.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "resnet50.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "resnet50_int8.param",
        Mat::new_3d(224, 224, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "squeezenet_ssd.param",
        Mat::new_3d(300, 300, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "squeezenet_ssd_int8.param",
        Mat::new_3d(300, 300, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "mobilenet_ssd.param",
        Mat::new_3d(300, 300, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "mobilenet_ssd_int8.param",
        Mat::new_3d(300, 300, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "mobilenet_yolo.param",
        Mat::new_3d(416, 416, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "mobilenetv2_yolov3.param",
        Mat::new_3d(352, 352, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "yolov4-tiny.param",
        Mat::new_3d(416, 416, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "nanodet-plus-m_416.param",
        Mat::new_3d(416, 416, 3, None),
        &opt,
        "output",
    )?;

    benchmark(
        "nanodet-plus-m_416-int8.param",
        Mat::new_3d(416, 416, 3, None),
        &opt,
        "output",
    )?;

    Ok(())
}
