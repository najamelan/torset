extern crate libtorset;

use std::io::Read;

use libtorset::*;

fn main()
{
	let input  = read_default_descriptors().unwrap();

	let parsed = parse_descriptors( &input ).unwrap();

	print!( "{}", &nft_var( &parsed ) );
}


fn read_default_descriptors() -> Result< String, failure::Error >
{
	let path = "/var/lib/tor/cached-microdesc-consensus";

	// let path = "resources/sample_consensus";

	// Open the path in read-only mode, returns `io::Result<File>`
	//
	let mut file = std::fs::File::open( path )?;


	// Read the file contents into a string, returns `io::Result<usize>`
	// We set the starting size to 2MB here, so we avoid reallocation while reading from the file.
	//
	let mut buffer = String::with_capacity( 2000000 );
	file.read_to_string( &mut buffer )?;

	Ok( buffer )
}

