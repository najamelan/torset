// The desired output is a set that can be assigned to a variable, like so:
//
// define torset = { 1.1.1.1 . 443, 2.2.2.2 . 9050,
// 	...
// }
//
// We probably only want to have a method that only outputs the actual address lines, so it can also be used slightly differently.
// However we can add additional methods that wrap it up into a complete nftables file, and even write it to disk if you like.
//
//
use crate::microdescriptor::MicroDescriptor;


// Create a comma separated list of ip . port entries to put in a nftables set
//
pub fn nft_plain( data: &Vec< MicroDescriptor >, ports: bool ) -> String
{
	// We count on +/- 6300 relays (beginning 2018) times max length of an entry (xxx.xxx.xxx.xxx . xxxxx,) 24 bytes = 151200 bytes
	//
	let mut out = String::with_capacity( 160000 );


	if ports
	{
		for desc in data
		{
			if desc.orport  != 0 { out += &format!( "{} . {},", desc.ip, desc.orport  ) }
			if desc.dirport != 0 { out += &format!( "{} . {},", desc.ip, desc.dirport ) }
		}
	}

	else
	{
		for desc in data { out += &( format!( "{},", desc.ip ) ) }
	}

	out
}



// Create valid nftables variable statement that can be send to a file for inclusion or to nft on the command line.
//
pub fn nft_var( input: &Vec< MicroDescriptor >, var_name: &str, ports: bool ) -> String
{
	// We count on +/- 6300 relays (beginning 2018) times max length of an entry (xxx.xxx.xxx.xxx . xxxxx,) 24 bytes = 151200 bytes
	//
	let mut out = String::with_capacity( 180000 );

	out += &format!( "define {} = {{", var_name );
	out += &nft_plain( input, ports )            ;
	out += "};\n"                                ;

	out
}





#[ cfg( test ) ]
//
mod tests
{
	use super::*;

	#[ test ]
	//
	fn single()
	{
		let d = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 80" );
		let v = vec![ d.unwrap() ];

		assert_eq!( nft_plain( &v, false ), "162.247.72.201," );

		assert_eq!( nft_var  ( &v, "tornodes", false ), "define tornodes = {162.247.72.201,};\n" );
	}

	#[ test ]
	//
	fn single_ports()
	{
		let d = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 80" );
		let v = vec![ d.unwrap() ];

		assert_eq!( nft_plain( &v, true ), "162.247.72.201 . 443,162.247.72.201 . 80," );

		assert_eq!( nft_var  ( &v, "tornodes", true ), "define tornodes = {162.247.72.201 . 443,162.247.72.201 . 80,};\n" );
	}



	#[ test ]
	//
	fn empty()
	{
		let v = vec![];

		assert_eq!( nft_plain( &v, true ), "" );

		assert_eq!( nft_var  ( &v, "tornodes", true ), "define tornodes = {};\n" );
	}



	#[ test ]
	//
	fn double()
	{
		let d = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 80" );
		let e = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 1.1.1.1        4    6" );
		let v = vec![ d.unwrap(), e.unwrap() ];

		assert_eq!( nft_plain( &v, false ), "162.247.72.201,1.1.1.1," );

		assert_eq!( nft_var  ( &v, "tornodes", false ), "define tornodes = {162.247.72.201,1.1.1.1,};\n" );
	}



	#[ test ]
	//
	fn double_ports()
	{
		let d = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 80" );
		let e = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 1.1.1.1        4    6" );
		let v = vec![ d.unwrap(), e.unwrap() ];

		assert_eq!( nft_plain( &v, true ), "162.247.72.201 . 443,162.247.72.201 . 80,1.1.1.1 . 4,1.1.1.1 . 6," );

		assert_eq!( nft_var  ( &v, "tornodes", true ), "define tornodes = {162.247.72.201 . 443,162.247.72.201 . 80,1.1.1.1 . 4,1.1.1.1 . 6,};\n" );
	}



	#[ test ]
	//
	fn without_dirport()
	{
		let d = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 0" );
		let e = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 1.1.1.1        4   6" );
		let v = vec![ d.unwrap(), e.unwrap() ];

		assert_eq!( nft_plain( &v, true ), "162.247.72.201 . 443,1.1.1.1 . 4,1.1.1.1 . 6," );

		assert_eq!( nft_var  ( &v, "tornodes", true ), "define tornodes = {162.247.72.201 . 443,1.1.1.1 . 4,1.1.1.1 . 6,};\n" );
	}
}
