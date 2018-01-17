#![feature(test)]

// The benchmarks here let us measure how long it takes to run torset

extern crate test;
extern crate libtorset;


use test::Bencher;

use libtorset::*;


fn setup() -> Vec< MicroDescriptor >
{
    let input  = read_descriptors( Some( "resources/sample_consensus" ) ).unwrap();

    parse_descriptors( &input ).unwrap()
}


#[bench] fn run_nft_plain( b: &mut Bencher ) { b.iter( || nft_plain( &setup() ) ) }
#[bench] fn run_nft_var  ( b: &mut Bencher ) { b.iter( || nft_var  ( &setup() ) ) }
