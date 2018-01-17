extern crate libtorset;

use libtorset::*;


fn main()
{
	let input  = read_descriptors( None ).unwrap();

	let parsed = parse_descriptors( &input ).unwrap();

	print!( "{}", &nft_var( &parsed ) );
}




