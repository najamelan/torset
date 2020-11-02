#![allow(clippy::suspicious_else_formatting)]

use libtorset::*;

use std::process;
use std::fs::write;
use clap::{ App, Arg, ArgMatches, SubCommand, AppSettings, crate_version, crate_authors };
use anyhow::Context;

/// Our type alias for handling errors throughout torset.
///
type TorsetResult<T> = anyhow::Result<T>;


const DEFAULT_SETNAME: &str = "tornodes";
const DEFAULT_INPUT  : &str = "/var/lib/tor/cached-microdesc-consensus";


fn main()
{
	env_logger::init();

	if let Err( err ) = try_main()
	{
		// Print the error, including all of its underlying causes.
		//
		eprintln!( "{}", pretty_error( &err ) );

		process::exit(1);
	}
}


//
//
fn try_main() -> TorsetResult<()>
{
	let args = arguments();

	// print!("{:#?}", args);

	let data   = read_descriptors ( args.value_of( "input" ) )?;
	let parsed = parse_descriptors( &data                    )?;

	let out: String;

	match args.subcommand_name()
	{
		Some( "ipset" ) =>
		{
			out =

				ipset
				(
					  &parsed

					, args.subcommand_matches( "ipset" )
					      .expect( "CLI: No subcommand matches? Error in the clap library?" )
					      .value_of( "set_name" )
					      .expect( "CLI: No set_name value, even though we have a default? Error in the clap library?" )

					, args.is_present( "ports" )
				)
		},

		Some( "nftables" ) =>
		{
			out =

				nft_var
				(
					  &parsed

					, args.subcommand_matches( "nftables" )
					      .expect( "CLI: No subcommand matches? Error in the clap library?" )
					      .value_of( "var_name" )
					      .expect( "CLI: No var_name value, even though we have a default? Error in the clap library?" )

					, args.is_present( "ports" )
				)
		},

		_ => { out = "".to_string() /*TODO: handle error*/ }
	}


	match args.value_of( "output" )
	{
		Some( "stdout" ) => { print!       ( "{}", out  )  },
		Some( "stderr" ) => { eprint!      ( "{}", out  )  },
		Some( file     ) => { print_to_file( file, &out )? },

		None             => panic!( "No output specified, but we have a default value of stdout, so this shouln't happen." )
	}

	Ok(())
}


fn print_to_file( file: &str, out: &str ) -> TorsetResult<()>
{
	write( file, out ).context( file.to_string() )?;

	Ok(())
}


/// Return a prettily formatted error, including its entire causal chain.
///
fn pretty_error( err: &anyhow::Error ) -> String
{
    let mut pretty = err.to_string();

    for cause in err.chain()
    {
    	pretty.push_str( ": "               );
    	pretty.push_str( &cause.to_string() );
    }

    pretty
}


fn arguments() -> ArgMatches< 'static >
{
	App::new( "torset" )

		.author ( crate_authors!() )
		.version( crate_version!() )
		.about  ( "Generate or update an ipset or an nftables set of tornodes from the cached microdescriptor file." )
		.setting( AppSettings::SubcommandRequiredElseHelp )
		.usage  ( "torset help     [subcommand]\n             torset ipset    [set_name]   [OPTIONS] | ipset restore\n             torset nftables [var_name]   [OPTIONS] > /etc/tornodes.conf")
		.template
		(
"
{bin}     : {about}
version    : {version} (by {author})

usage      : {usage}

description: {bin} creates an ipset or an nftables variable containing all tor nodes, for use in firewalls.you
             If you want to transparently reroute traffic through tor, you would create a firewall rule to make
             sure no other outgoing connections get through, and you would not want to reroute traffic that already
             connects to tor, to avoid double torifying.

             With the help of the set generated from torset you can do this in the most common firewalls on linux. Firehol does
             not support ipsets with ports, so if you don't want to bother modifying the iptables rules, you can still generate
             an ipset without the --ports option to get ip addresses only.

             {bin} does not generate ip6 sets.

{subcommands}

{unified}
"
		)

		.subcommand
		(
			SubCommand::with_name( "ipset" )

				.about( "Generate an ipset of tor nodes" )

				.arg
				(
					Arg::with_name( "set_name" )

					.help( "The name of the ipset to create or replace" )
					.index( 1 )
					.default_value( DEFAULT_SETNAME )
					.takes_value( true )
					.display_order( 1 )
				)

				.usage( "torset ipset [set_name] [OPTIONS] | ipset restore")

		)


		.subcommand
		(
			SubCommand::with_name( "nftables" )

				.about( "Generate an nftables set of tor nodes" )

				.arg
				(
					Arg::with_name( "var_name" )

					.help( "The name of the nftables variable containing the set" )
					.index( 1 )
					.default_value( DEFAULT_SETNAME )
					.takes_value( true )
					.display_order( 1 )
				)

				.usage( "torset nftables [var_name] [OPTIONS] > /etc/nftables/tornodes.conf" )

		)


		.arg
		(
			Arg::with_name( "input"  )

				.help ( "Where to read the consensus file from. Note that the default value usually requires torset to be run as root. If you don't want torset to run as root, you can pipe to stdin. Special value accepted: 'stdin'. " )
				.long ( "input" )
				.short( "i"     )
				.value_name( "filename" )
				.default_value( DEFAULT_INPUT )
				.global( true )
		)


		.arg
		(
			Arg::with_name( "output"  )

				.help ( "Where to send the results. Special values accepted: 'stdout' and 'stderr'" )
				.long ( "output" )
				.short( "o"     )
				.value_name( "filename" )
				.default_value( "stdout" )
				.global( true )
		)


		.arg
		(
			Arg::with_name( "ports" )

				.help ( "Whether to make a set with both ip addresses and ports" )
				.long ( "ports" )
				.short( "p"     )
				.global( true   )

		)

	.get_matches()
}



