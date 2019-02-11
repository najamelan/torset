use std::str::FromStr;
use std::net::Ipv4Addr;

use chrono::prelude::*;
use failure::ensure;



// MicroDescriptor - Currently only parse the 'r' line of the consensus file which holds
// the ip adress and ports for each relay.
//
// Note that we convert the strings to specific types like DateTime and Ipv4Address, even
// though we only need the as strings, but this guarantees validation.
//
// Description from dir-spec.txt (https://gitweb.torproject.org/torspec.git/tree/dir-spec.txt)
//
// 3.9.2. Microdescriptor consensus

//    The microdescriptor consensus is a consensus flavor that contains
//    microdescriptor hashes instead of descriptor hashes and that omits
//    exit-policy summaries which are contained in microdescriptors.  The
//    microdescriptor consensus was designed to contain elements that are
//    small and frequently changing.  Clients use the information in the
//    microdescriptor consensus to decide which servers to fetch information
//    about and which servers to fetch information from.

//    The microdescriptor consensus is based on the unflavored consensus with
//    the exceptions as follows:

//     "network-status-version" SP version SP "microdesc" NL

//         [At start, exactly once.]

//         The flavor name of a microdescriptor consensus is "microdesc".

//    Changes to router status entries are as follows:

//     "r" SP nickname SP identity SP publication SP IP SP ORPort SP DirPort NL

//         [At start, exactly once.]

//         Similar to "r" lines in section 3.4.1, but without the digest element.

//     "a" SP address ":" port NL

//         [Any number]

//         Identical to the "r" lines in section 3.4.1.

//         (Only included when the vote is generated with consensus-method 14
//         or later, and the consensus is generated with consensus-method 27 or
//         later.)

//     "p" ... NL

//         [At most once]

//         Not currently generated.

//         Exit policy summaries are contained in microdescriptors and
//         therefore omitted in the microdescriptor consensus.

//     "m" SP digest NL

//         [Exactly once.*]

//         "digest" is the base64 of the SHA256 hash of the router's
//         microdescriptor with trailing =s omitted.  For a given router
//         descriptor digest and consensus method there should only be a
//         single microdescriptor digest in the "m" lines of all votes.
//         If different votes have different microdescriptor digests for
//         the same descriptor digest and consensus method, at least one
//         of the authorities is broken.  If this happens, the microdesc
//         consensus should contain whichever microdescriptor digest is
//         most common.  If there is no winner, we break ties in the favor
//         of the lexically earliest.

//         [*Before consensus method 13, this field was sometimes erroneously
//         omitted.]

//    Additionally, a microdescriptor consensus SHOULD use the sha256 digest
//    algorithm for its signatures.
//
//
//
//  Relevant part from section 3.4.1
//  ===================================
//
// "r" SP nickname SP identity SP digest SP publication SP IP SP ORPort SP DirPort NL

// [At start, exactly once.]

// "Nickname" is the OR's nickname.  "Identity" is a hash of its
// identity key, encoded in base64, with trailing equals sign(s)
// removed.  "Digest" is a hash of its most recent descriptor as
// signed (that is, not including the signature), encoded in base64.

// "Publication" is the publication time of its most recent descriptor,
// in the form YYYY-MM-DD HH:MM:SS, in UTC.  Implementations MAY base
// decisions on publication times in the past, but MUST NOT reject
// publication times in the future.

// "IP" is its current IP address; ORPort is its current OR port,
// "DirPort" is its current directory port, or "0" for "none".

// Example line: r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 80
//
#[ derive( Debug ) ]
//
pub struct MicroDescriptor
{
	pub nickname   : String,
	pub identifier : String,
	pub publication: DateTime< Utc >,
	pub ip         : std::net::Ipv4Addr,
	pub orport     : u16,
	pub dirport    : u16,
}



impl MicroDescriptor
{
	pub fn new( input: &str ) -> Result< MicroDescriptor, failure::Error >
	{
		let split: Vec< &str > = input.split_whitespace().collect();

		// Validation
		// Will exit the function with an error. This is necessary to avoid the out of bounds indexing risks below.
		//
		ensure!( split.len() == 8, "Microdescriptor line does not contain 8 fields: {}", input );

		let publication  = Utc.datetime_from_str( &( split[ 3 ].to_string() + split[ 4 ] ), "%Y-%m-%d%H:%M:%S" )?;
		let ip           = Ipv4Addr::from_str   (    split[ 5 ]                                                )?;

		// https://en.wikipedia.org/wiki/Registered_port
		// Should fail if overflow occurs. I checked the std source.
		//
		let orport : u16 = split[ 6 ].parse()?;
		let dirport: u16 = split[ 7 ].parse()?;

		Ok
		(
			MicroDescriptor
			{
				nickname   : split[ 1 ].to_string(),
				identifier : split[ 2 ].to_string(),
				publication                        ,
				ip                                 ,
				orport                             ,
				dirport                            ,
			}
		)
	}
}


#[ cfg( test ) ]
//
mod tests
{
	use super::*;

	#[ test ]
	//
	fn valid()
	{
		let d = MicroDescriptor::new( "r CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 80" );

		assert!( d.is_ok() );

		let d = d.unwrap();

		assert_eq!( d.nickname    , "CalyxInstitute14"                         );
		assert_eq!( d.identifier  , "ABG9JIWtRdmE7EFZyI/AZuXjMA4"              );
		assert_eq!( d.publication , Utc.ymd( 2018, 1, 13 ).and_hms( 8, 19, 4 ) );
		assert_eq!( d.nickname    , "CalyxInstitute14"                         );
		assert_eq!( d.orport      , 443                                        );
		assert_eq!( d.dirport     , 80                                         );
	}


	#[ test ]
	//
	fn empty_string()
	{
		let d = MicroDescriptor::new( "" );

		assert!( d.is_err() );
	}


	#[ test ]
	//
	fn invalid()
	{
		let d = MicroDescriptor::new( "CalyxInstitute14 ABG9JIWtRdmE7EFZyI/AZuXjMA4 2018-01-13 08:19:04 162.247.72.201 443 80" );

		assert!( d.is_err() );
	}
}

