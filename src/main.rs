use hound::WavReader;

fn main() {
	let mut reader = WavReader::open("sample.wav").unwrap();
	for sample in reader.samples::<i16>() {
		println!("{}", sample.unwrap());
	}
}
