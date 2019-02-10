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
use super::MicroDescriptor;


// Create valid ipset to be piped to the ipset command
//
pub fn ipset( data: &[ MicroDescriptor ], set_name: &str, ports: bool ) -> String
{

	// We count on +/- 11000 relays/port combinations (beginning 2019) times max length of an entry ("add tornodes xxx.xxx.xxx.xxx,xxxxx -exists\n") +/- 50 bytes = 550000 bytes
	//
	let mut out  = String::with_capacity( 600_000 );
	let tmp_name = &format!( "tmp_{}", set_name );

	out += &create_set( set_name, ports );
	out += &create_set( tmp_name, ports );

	out += &format!( "flush {}\n", tmp_name ); // make sure the temporary set is empty

	add_ips( &mut out, tmp_name, data, ports );

	out += &format!( "swap {} {}\n", tmp_name, set_name ); // swap the two sets
	out += &format!( "destroy {}\n", tmp_name           ); // delete the temporary set

	out
}



#[inline]
//
fn create_set( set_name: &str, ports: bool ) -> String
{
	if   ports { format!( "create {} hash:ip,port -exist\n", set_name ) }
	else       { format!( "create {} hash:ip -exist\n"     , set_name ) }
}


#[inline]
//
fn add_ips( out: &mut String, set_name: &str, data: &[ MicroDescriptor ], ports: bool )
{
	if ports
	{
		for desc in data
		{
			if desc .orport != 0 { *out += &format!( "add {} {},{} -exist\n", set_name, desc.ip, desc.orport  ) }
			if desc.dirport != 0 { *out += &format!( "add {} {},{} -exist\n", set_name, desc.ip, desc.dirport ) }
		}
	}

	else
	{
		for desc in data
		{
			*out += &format!( "add {} {} -exist\n", set_name, desc.ip );
		}
	}
}





#[ cfg( test ) ]
//
mod tests
{
	use super::*;

	macro_rules! TEMPLATE
	{
	    () =>

("create tornodes hash:ip -exist
create tmp_tornodes hash:ip -exist
flush tmp_tornodes
{}swap tmp_tornodes tornodes
destroy tmp_tornodes
")
	}

	macro_rules! TEMPLATE_PORTS
	{
	    () =>

("create tornodes hash:ip,port -exist
create tmp_tornodes hash:ip,port -exist
flush tmp_tornodes
{}swap tmp_tornodes tornodes
destroy tmp_tornodes
")
	}

	#[ test ]
	//
	fn single()
	{
		let d = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 80" );
		let v = vec![ d.unwrap() ];

		assert_eq!( ipset( &v, "tornodes", false ), format!( TEMPLATE!(), "add tmp_tornodes 162.247.72.201 -exist\n" ) );

	}

	#[ test ]
	//
	fn single_port()
	{
		let d = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 80" );
		let v = vec![ d.unwrap() ];

		assert_eq!
		(
			  ipset( &v, "tornodes", true )
			, format!( TEMPLATE_PORTS!(), "add tmp_tornodes 162.247.72.201,443 -exist\nadd tmp_tornodes 162.247.72.201,80 -exist\n" )
		);

	}



	#[ test ]
	//
	fn empty()
	{
		let v = vec![];

		assert_eq!( ipset( &v, "tornodes", false ), format!( TEMPLATE!(), "" ) );
	}



	#[ test ]
	//
	fn empty_ports()
	{
		let v = vec![];

		assert_eq!( ipset( &v, "tornodes", true ), format!( TEMPLATE_PORTS!(), "" ) );
	}



	#[ test ]
	//
	fn double()
	{
		let d = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 80" );
		let e = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 1.1.1.1        4    6" );
		let v = vec![ d.unwrap(), e.unwrap() ];

		assert_eq!
		(
			  ipset( &v, "tornodes", false )
			, format!( TEMPLATE!(),
"add tmp_tornodes 162.247.72.201 -exist
add tmp_tornodes 1.1.1.1 -exist
")
		);
	}



	#[ test ]
	//
	fn double_ports()
	{
		let d = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 80" );
		let e = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 1.1.1.1        4    6" );
		let v = vec![ d.unwrap(), e.unwrap() ];

		assert_eq!
		(
			  ipset( &v, "tornodes", true )
			, format!( TEMPLATE_PORTS!(),
"add tmp_tornodes 162.247.72.201,443 -exist
add tmp_tornodes 162.247.72.201,80 -exist
add tmp_tornodes 1.1.1.1,4 -exist
add tmp_tornodes 1.1.1.1,6 -exist
")
		);
	}



	#[ test ]
	//
	fn without_dirport()
	{
		let d = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 0" );
		let e = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 1.1.1.1        4   6" );
		let v = vec![ d.unwrap(), e.unwrap() ];

		assert_eq!
		(
			  ipset( &v, "tornodes", true )
			, format!( TEMPLATE_PORTS!(),
"add tmp_tornodes 162.247.72.201,443 -exist
add tmp_tornodes 1.1.1.1,4 -exist
add tmp_tornodes 1.1.1.1,6 -exist
")
		);
	}
}
