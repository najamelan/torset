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


namespace torset
{

class IpsetRestore
{
	private:

		std::stringstream consensus ;
		std::string       setName   ;
		int               _errorCode;

		// this is the first line I found when trying ipset save. I don't know if it is important to keep hashsize and maxelem
		// have not found a format specification for ipset restore files
		//
		std::string _set;



	public:

		explicit IpsetRestore( const std::stringstream& consensusIn, const std::string& setName );
		virtual ~IpsetRestore();


		// methods
		//

		// returns a string which is a valid to feed to ipset restore
		//
		std::string set      () const { return _set      ; };
		int         errorCode() const { return _errorCode; };
};




// Definitions

IpsetRestore::IpsetRestore( const std::stringstream& consensusIn, const std::string& setName )

:   consensus    ( consensusIn.str() )
  , setName      ( setName           )
  , _errorCode   ( 0                 )

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
		// sample: "r Unnamed VLNV4cpI/C4jFPVQcWKBtO2EBV8 2013-11-04 22:38:31 76.100.70.54 9001 9030"
		//
		if( fields.size() != 8 || fields[ 0 ] != "r" )

			continue;


		// write add lines in the ipset format
		// add [setName] [ip],tcp:[port]
		//
		hold.clear();
		hold.append( "add ").append( setName ).append( " " ).append( fields[ 5 ] ).append( ",tcp:" ).append( fields[ 6 ] ).append( " -exist\n" );

		if( fields[ 6 ] != "0" )

			_set.append( hold );


		// second port
		//
		hold.clear();
		hold.append( "add ").append( setName ).append( " " ).append( fields[ 5 ] ).append( ",tcp:" ).append( fields[ 7 ] ).append( " -exist\n" );

		if( fields[ 7 ] != "0" )

			_set.append( hold );
	}


	if( _set.empty() )
	{
		std::cerr << "Something went wrong, _set is empty. Maybe you passed the wrong inputfile or it was not formatted correctly." << std::endl;

		++_errorCode;
	}
}


/// Destructor.

IpsetRestore::~IpsetRestore()
{
}



} 			// namespace torset
#endif 	// Guard_QDSF65D651S_65Z1Z8Z1_3SD021NY3
