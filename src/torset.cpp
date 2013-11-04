// This application creates or updates an ipset with ip's read from a tor microdescriptor consensus file
//
//
// run torentryset to get usage information



#include "config.hpp"
#include "torset.hpp"

#include "stdio.h"

#include <string>
#include <iostream>
#include <fstream>
#include <cstdlib>


const std::string DEFAULT_CONSENSUS = "/var/lib/tor/cached-microdesc-consensus";


void printUsage()
{
	std::cout

		<< std::endl
		<< "torentryset: create or update an ipset from a tor microdescriptor consensus file." << std::endl
	   << "version    : ALPHA - you are warned"                                               << std::endl
	   << "Usage      : torentryset setname [ consensusFile ]"                                << std::endl << std::endl
	   << "consensusFile defaults to '/var/lib/tor/cached-microdesc-consensus' in which case" << std::endl
	   << "you will need root privileges."                                                    << std::endl << std::endl
	;
}


int main( int argc, char* argv[] )
{
	// check the arguments
	//
	if( argc < 2 || argc > 3 )
	{
		printUsage(   );
		exit      ( 1 );
	}



	// create the set names
	//
	std::string setName   ( argv[ 1 ] );
	std::string tmpSetName( "new_"    );
	tmpSetName.append( setName );



	// Determine the input file
	//
	std::string consensus;

	if( argc == 3 )

		consensus = argv[ 2 ];

	else

		consensus = DEFAULT_CONSENSUS;



	// Create the set of ip's
	//
	std::ifstream inputFile( consensus.c_str() );


	if( !inputFile )

		std::cerr << "Error: Can't open consensus file. Do you have the right permissions?" << std::endl;


	std::stringstream inputStream;

	inputStream << inputFile.rdbuf();

	tidbits::TorSet torset( inputStream, tmpSetName );



	// Create the restore file for ipset
	//
	std::string toRestore;

	toRestore

		.append( "create "       ).append( setName    ).append( " hash:ip,port -exist\n" )               // create set
		.append( "create "       ).append( tmpSetName ).append( " hash:ip,port -exist\n" )               // create temporary set
		.append( torset.getSet() )                                                                        // add the ip's to the temporary set
		.append( "swap "         ).append( setName    ).append( " " ).append( tmpSetName ).append( "\n" ) // swap the two sets over
		.append( "destroy "      ).append( tmpSetName ).append( "\n" )                                    // delete the temporary set
	;


	// feed it all to ipset
	//
	FILE* ipsetSTDIN = popen( "ipset restore", "w" );

	fputs( toRestore.c_str(), ipsetSTDIN );

	pclose( ipsetSTDIN );
}



