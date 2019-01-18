#![feature(test)]

// The benchmarks here let us measure how long it takes to run torset

extern crate test;
use libtorset;
use std::fs::write;


use test::Bencher;

use libtorset::*;


fn setup() -> Vec< MicroDescriptor >
{
    let input  = read_descriptors( Some( "data/sample_consensus" ) ).unwrap();

    parse_descriptors( &input ).unwrap()
}


fn print_to_file( file: &str, out: &String )
{
	write( file, out ).unwrap();
}


#[bench] fn run_nft_plain( b: &mut Bencher ) { b.iter( || print_to_file( "/dev/null", &nft_plain( &setup()            , true )));}
#[bench] fn run_nft_var  ( b: &mut Bencher ) { b.iter( || print_to_file( "/dev/null", &nft_var  ( &setup(), "tornodes", true )));}
#[bench] fn run_ipset    ( b: &mut Bencher ) { b.iter( || print_to_file( "/dev/null", &ipset    ( &setup(), "tornodes", true )));}
