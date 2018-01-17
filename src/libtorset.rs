#[macro_use] pub extern crate failure;
#[macro_use] extern crate lazy_static;

extern crate chrono ;
extern crate regex  ;

mod microdescriptor;
mod nftables;

pub use microdescriptor::*;
pub use nftables       ::*;


use regex::Regex;


// The interface:
//
//

pub fn parse_descriptors( input: &str ) -> Result< Vec< MicroDescriptor >, failure::Error >
{
	// At the time of writing (beginning 2018) there were 6125 relays in the consensus file.
	// The size of MicroDescriptor is 72 bytes, which gives 420KB plus the size of the string nickname and identifiers.
	// Those latter are usually under 50 bytes, which gives a total of +/- 800KB.
	//
	let mut out    : Vec< MicroDescriptor > = Vec::with_capacity( 6300 );
	let mut counter: usize = 0;

	lazy_static! { static ref RE: Regex = Regex::new( r"^r " ).unwrap(); }


	for line in input.lines().filter( |l| RE.is_match( l ) )
	{
		if let Ok( parsed ) = MicroDescriptor::new( line )
		{
			out.push( parsed );
		}

		else if cfg!( debug_assertions )
		{
			counter += 1;
		}
	}


	// We do return a result after all.after
	//
	ensure!( out.len() != 0, "Could not find any valid microdescriptor in the text passed in." );


	// Get some feedback about whether we get in a lot of lines that fail to parse.
	//
	if cfg!( debug_assertions ) && counter != 0
	{
		print!("Number of microdescriptor lines that failed to parse: {:?}\n", counter );
	}


	Ok( out )
}


#[ cfg( test ) ]
//
mod tests
{
	use super::*;

	#[ test ]
	//
	fn find2()
	{
		let input = "pr Cons=1-2 Desc=1-2 DirCache=1-2 HSDir=1-2 HSIntro=3-4 HSRend=1-2 Link=1-4 LinkAuth=1,3 Microdesc=1-2 Relay=1-2
w Bandwidth=344
r dominicTORetto U/ZsC3HFraxCy7dN4mw2YDEldso 2018-01-13 17:06:14 159.203.15.100 80 0
m UtOUktMsVBDmr0Y/rczRG8Ymek1XWZDRVClsiA8GY78
s Fast Guard Running Stable V2Dir Valid
v Tor 0.3.1.9
pr Cons=1-2 Desc=1-2 DirCache=1-2 HSDir=1-2 HSIntro=3-4 HSRend=1-2 Link=1-4 LinkAuth=1,3 Microdesc=1-2 Relay=1-2
w Bandwidth=10800
r UbuntuCore201 U/oDBUt/V5j9RE6nLLFw8ngffKw 2018-01-13 15:30:49 88.175.188.246 33841 0
m 2yd/vfmaGts8m3AWoKM5BNbvPTUVMiHzrVpq+jbfaOs
s Running V2Dir Valid
v Tor 0.3.1.9";

		let result = parse_descriptors( input );

		assert!   ( result.is_ok ()          );
		assert_eq!( result.unwrap().len(), 2 );
	}
}
