extern crate ethash;
extern crate block;
extern crate rlp;
extern crate hexutil;
extern crate bigint;
extern crate sha3;
extern crate blockchain;

use ethash::*;
use block::*;
use hexutil::*;
use bigint::*;
use blockchain::chain::HeaderHash;
use sha3::{Keccak256, Digest};

use std::str::FromStr;

#[test]
fn header1() {
    let header: Header = rlp::decode(&read_hex("f901f3a00000000000000000000000000000000000000000000000000000000000000000a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a09178d0f23c965d81f0834a4c72c6253ce6830f4022b1359aaebfc1ecba442d4ea056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008302000080830f4240808080a058f759ede17a706c93f13030328bcea40c1d1341fb26f2facd21ceb0dae57017884242424242424242").unwrap());
    let partial = PartialHeader::from_full(header.clone());

    assert_eq!(get_cache_size(header.number), 16776896);
    assert_eq!(get_full_size(header.number), 1073739904);
    assert_eq!(partial.header_hash(), H256::from_str("2a8de2adf89af77358250bf908bf04ba94a6e8c3ba87775564a41d269a05e4ce").unwrap());

    let cache_size = get_cache_size(header.number);
    let full_size = get_full_size(header.number);

    let mut cache: Vec<u8> = Vec::with_capacity(cache_size);
    cache.resize(cache_size, 0);

    let seed = get_seedhash(header.number);
    assert_eq!(seed, H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap());

    make_cache(&mut cache, seed);

    let hash = H256::from(Keccak256::digest(&cache).as_slice());
    assert_eq!(hash, H256::from_str("35ded12eecf2ce2e8da2e15c06d463aae9b84cb2530a00b932e4bbc484cde353").unwrap());

    let (mixhash, result) = hashimoto_light(&partial, H64::from("4242424242424242"),
                                            full_size, &cache);

    assert_eq!(mixhash, H256::from("58f759ede17a706c93f13030328bcea40c1d1341fb26f2facd21ceb0dae57017"));
    assert_eq!(result, H256::from("dd47fd2d98db51078356852d7c4014e6a5d6c387c35f40e2875b74a256ed7906"));
}

#[test]
fn header2() {
    let header: Header = rlp::decode(&read_hex("f901f7a01bef91439a3e070a6586851c11e6fd79bbbea074b2b836727b8e75c7d4a6b698a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d4934794ea3cb5f94fa2ddd52ec6dd6eb75cf824f4058ca1a00c6e51346be0670ce63ac5f05324e27d20b180146269c5aab844d09a2b108c64a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008302004002832fefd880845511ed2a80a0e55d02c555a7969361cf74a9ec6211d8c14e4517930a00442f171bdb1698d17588307692cf71b12f6d").unwrap());
    let partial = PartialHeader::from_full(header.clone());

    assert_eq!(get_cache_size(header.number), 16776896);
    assert_eq!(get_full_size(header.number), 1073739904);
    assert_eq!(partial.header_hash(), H256::from_str("100cbec5e5ef82991290d0d93d758f19082e71f234cf479192a8b94df6da6bfe").unwrap());

    let cache_size = get_cache_size(header.number);
    let full_size = get_full_size(header.number);

    let mut cache: Vec<u8> = Vec::with_capacity(cache_size);
    cache.resize(cache_size, 0);

    let seed = get_seedhash(header.number);
    assert_eq!(seed, H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap());

    make_cache(&mut cache, seed);

    let hash = H256::from(Keccak256::digest(&cache).as_slice());
    assert_eq!(hash, H256::from_str("35ded12eecf2ce2e8da2e15c06d463aae9b84cb2530a00b932e4bbc484cde353").unwrap());

    let (mixhash, result) = hashimoto_light(&partial, H64::from("307692cf71b12f6d"),
                                            full_size, &cache);

    assert_eq!(mixhash, H256::from("e55d02c555a7969361cf74a9ec6211d8c14e4517930a00442f171bdb1698d175"));
    assert_eq!(result, H256::from("ab9b13423cface72cbec8424221651bc2e384ef0f7a560e038fc68c8d8684829"));
}
