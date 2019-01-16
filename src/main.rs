use libtorset::*;


use clap::{ App, Arg, ArgMatches, SubCommand, AppSettings, crate_version, crate_authors };


// TODO: Error handling
// TODO: Take microdescriptor file from command line
// TODO: Allow iptables, iptables with port, nftables
//
fn main()
{
	let _: ArgMatches = arguments();

	// print!("{:#?}", matches);



	let input  = read_descriptors( None ).unwrap();

	let parsed = parse_descriptors( &input ).unwrap();

	print!( "{}", &nft_var( &parsed ) );
}


fn arguments() -> ArgMatches< 'static >
{
	App::new( "torset" )

		.author ( crate_authors!() )
		.version( crate_version!() )
		.about  ( "Generate or update an ipset or an nftables set of tornodes from the cached microdescriptor file." )
		.setting( AppSettings::SubcommandRequiredElseHelp )
		.usage  ( "torset help [subcommand]\n             torset ipset <setname> [options] | ipset\n             torset nftables        [options] > /etc/tornodes.conf")
		.template
		(
"
{bin}     : {about}
version    : {version} (by {author})

usage      : {usage}

description: {bin} allows you to create firewall rules based on tor node ip addresses. Eg. If you want to transparently
             reroute traffic through tor, you would create a firewall rule to make sure no other outgoing connections
             get through, and you would not want to reroute traffic that already connects to tor, to avoid double torifying.

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
					Arg::with_name( "setname" )

					.help( "The name of the ipset to create or replace" )
					.index( 1 )
					.required( true )
					.takes_value( true )
					.display_order( 1 )
				)

				.usage( "torset ipset <setname> [OPTIONS]")

		)


		.subcommand
		(
			SubCommand::with_name( "nftables" )

				.about( "Generate an nftables set of tor nodes" )

				.arg
				(
					Arg::with_name( "setname" )

					.help( "The name of the ipset to create or replace" )
					.index( 1 )
					.required( true )
					.takes_value( true )
					.display_order( 1 )
				)

				.usage( "torset nftables <setname> [OPTIONS]")

		)


		.arg
		(
			Arg::with_name( "input"  )

				.help ( "Where to read the consensus file from. Note that the default value usually requires torset to be run as root. Special value accepted: 'stdin'. If you don't want torset to run as root, you can pipe to stdin." )
				.long ( "input" )
				.short( "i"     )
				.value_name( "filename" )
				.default_value( "/var/lib/tor/cached-microdesc-consensus" )
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

		)

	.get_matches()
}



