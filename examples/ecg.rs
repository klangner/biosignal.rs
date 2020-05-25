use biosignal::io;


fn main() {
    let fname = "examples/data/ecg.mat";
    let signal = io::read_mat(fname, "ecg", 500).unwrap();
    println!("Length: {:?}", signal.len());
}
