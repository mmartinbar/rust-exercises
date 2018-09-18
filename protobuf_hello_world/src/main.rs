extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
	protoc_rust::run(protoc_rust::Args {
      
	    out_dir: "src/messages",
	    input: &["src/protos/person.proto"],
      includes: &["src/protos"],
	    customize: Customize {
	      ..Default::default()
	    },
	}).expect("protoc");
}
