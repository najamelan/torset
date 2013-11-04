#ifndef Guard_QDSF65D651S_65Z1Z8Z1_3SD021NY3
#define Guard_QDSF65D651S_65Z1Z8Z1_3SD021NY3
#define torset_hpp

// local includes


// system includes
//
#include <cassert>
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>



namespace tidbits
{


class TorSet
{
	private:

		std::stringstream consensus;
		std::string       setName  ;

		// this is the first line I found when trying ipset save. I don't know if it is important to keep hashsize and maxelem
		// have not found a format specification for ipset restore files
		//
		std::string set;



	public:

		explicit TorSet( const std::stringstream& consensusIn, const std::string& setName );
		virtual ~TorSet();


		// methods
		//

		// returns a string which is a valid to feed to ipset restore
		//
		std::string getSet() const;
};




// Definitions

TorSet::TorSet( const std::stringstream& consensusIn, const std::string& setName )

:   consensus( consensusIn.str() )
  , setName  ( setName           )

{
	std::string              line      ;
	std::string              hold      ;
	std::vector<std::string> fields    ;
	std::string              field     ;
	std::stringstream        lineStream;

	// get each line separately
	//
	while( std::getline( consensus, line ) )
	{
		fields    .clear();
		lineStream.clear();
		lineStream.str  ( line );


		// get each field
		//
		while( std::getline( lineStream, field, ' ' ) )

			fields.push_back( std::string( field ) );


		// only work on lines that interest us
		//
		if( fields.size() != 8 || fields[ 0 ] != "r" )

			continue;


		// write add lines in the ipset format
		// add [setName] [ip],tcp:[port]
		//
		hold.clear();
		hold.append( "add ").append( setName ).append( " " ).append( fields[ 5 ] ).append( ",tcp:" ).append( fields[ 6 ] ).append( " -exist\n" );

		if( fields[ 6 ] != "0" )

			set.append( hold );


		hold.clear();
		hold.append( "add ").append( setName ).append( " " ).append( fields[ 5 ] ).append( ",tcp:" ).append( fields[ 7 ] ).append( " -exist\n" );

		if( fields[ 7 ] != "0" )

			set.append( hold );
	}


	if( set.empty() )

		std::cerr << "Something went wrong, set is empty. Maybe you passed the wrong inputfile or it was not formatted correctly." << std::endl;

}


/// Destructor.

TorSet::~TorSet()
{
}



// METHODS

std::string
TorSet::getSet() const
{
	return set;
}


} 			// namespace tidbits
#endif 	// Guard_QDSF65D651S_65Z1Z8Z1_3SD021NY3
