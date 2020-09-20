//! Tests
use super::*;
use std::collections::{
    HashMap,
};

#[test]
fn it_works()
{
    let mut map = Map::new();

    map.insert('>', false);
    map.insert('<', true);
    map.insert('ł', true);

    let clone = map.clone();

    for (k, v) in clone.into_iter()
    {
	assert!(map.get(&k) == Some(&v))
    }
}

macro_rules! test_insert_type {
    ($ty:ty, $data:expr) => {
	{
	    print!("Testing {}... ", std::any::type_name::<$ty>());
	    let mut small = Map::new();
	    let mut hash = HashMap::new();

	    for (i,x) in (0..).zip($data)
	    {
		small.insert(x, i);
		hash.insert(x, i);
	    }

	    assert_eq!(small.len(), hash.len());

	    for (k,v) in hash.iter()
	    {
		assert_eq!(v, small.get(k).unwrap());
	    }
	    println!("OK");
	}
    };
    ($ty:tt) => {
	test_insert_type!($ty, $ty::MIN..$ty::MAX);
    }
}

#[test]
fn type_char()
{
    test_insert_type!(char, TEST_STRING.chars());
}

#[test]
fn type_primitive()
{
    test_insert_type!(i8, TEST_STRING.chars());
    test_insert_type!(u8);
    test_insert_type!(i16);
    test_insert_type!(u16);
    test_insert_type!(i32, -100..1000);
    test_insert_type!(u32, 0..10000);
    test_insert_type!(u64, -100..1000);
    test_insert_type!(i64, 0..10000);
    test_insert_type!(u128, -100..1000);
    test_insert_type!(i128, 0..10000);
}

#[cfg(nightly)]
mod benchmarks
{
    use super::*;
    use std::collections::BTreeMap;
    use test::{Bencher, black_box};

    macro_rules! map_bench {
	($b:expr, $map:ident) => {
	    let mut map = $map::new();
	    $b.iter(|| {
		for chr in TEST_STRING.chars()
		{
		    if let Some(ent) = map.get_mut(&chr) {
			*ent += 1;
		    } else {
			black_box(map.insert(chr, 0));
		    }
		}
	    })
	};
    }

    
    macro_rules! ent_bench {
	($b:expr, $map:ident) => {
	    let mut map = $map::new();
	    $b.iter(|| {
		for chr in TEST_STRING.chars()
		{
		    black_box(*map.entry(chr).or_insert(0usize) += 1);
		}
	    })
	};
    }
    
    #[bench]
    fn es_char(b: &mut Bencher)
    {
	ent_bench!(b, Map);
    }
    
    #[bench]
    fn eh_char(b: &mut Bencher)
    {
	ent_bench!(b, HashMap);
    }
    
    #[bench]
    fn eb_char(b: &mut Bencher)
    {
	ent_bench!(b, BTreeMap);
    }
    #[bench]
    fn s_char(b: &mut Bencher)
    {
	map_bench!(b, Map);
    }
    
    #[bench]
    fn h_char(b: &mut Bencher)
    {
	map_bench!(b, HashMap);
    }
    
    #[bench]
    fn b_char(b: &mut Bencher)
    {
	map_bench!(b, BTreeMap);
    }
}

const TEST_STRING: &str = r#"
君のようなひとになりたいな
「僕らしいひと」になりたいな
望むならそうすりゃいいけどさ
でもそれってほんとにぼくなのかい

子供騙しな夢ひとつ
こんな僕なら死ねばいいのに

こんな僕が生きてるだけで
何万人のひとが悲しんで
誰も僕を望まない
そんな世界だったらいいのにな

こんな僕が消えちゃうだけで
何億人のひとが喜んで
誰も何も憎まないなら
そんなうれしいことはないな


明日も僕は夢うつつ
このまま僕は消えていいのに

こんな僕が生きたところで
何億人のひとは知らないし
誰も僕を望まない
そんな世界だったらいいのかな

こんな僕が消えたところで
何億人のひとは変わらない
誰も僕を憎まないなら
損した事に変わりないな

最期なんかみんな同じように倒れてゆきます
メイドイン 他人 の 「自分自身」崩れてゆきます
最期なんかみんな同じように離れてくのに

こんな僕が生きてるだけで
なんで君はそんなに笑うの
君がそんな笑顔じゃ
悲しくても消えたくても
さよならする理由なんてもう
無ければいいのに


こんな僕が消えたところで
何億人のひとは変わらない
だけど僕を止める何かが
そんな顔しちゃ笑えないや "#;