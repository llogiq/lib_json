mod buffer_walk;
use buffer_walk::CharIter;

use std::collections::BTreeMap;


///Json Enum that contains sub types
#[allow(dead_code)]
#[derive(Clone,Debug)]
pub enum Json<'a> {
	Var( &'a str ),
	Obj( BTreeMap<&'a str,Json<'a>> ),
	Vec( Vec<Json<'a>> )
}
impl<'a> Json<'a> {

	///Parse a Json File
	#[inline]
	#[allow(dead_code)]
	pub fn new<'b>( buffer: &'b str ) -> Option<Json<'b>> {
		let mut c = CharIter::new( buffer );
		let _ = c.skip_whitespace();
		read_field( &mut c )
	}

	///Type Checking
	#[inline]
	pub fn is_var( &self ) -> bool {
		match self {
			&Json::Var(_) => true,
			_ => false
		}
	}
	#[inline]
	pub fn is_obj( &self ) -> bool {
		match self {
			&Json::Obj(_) => true,
			_ => false
		}
	}
	#[inline]
	pub fn is_vec( &self ) -> bool {
		match self {
			&Json::Vec(_) => true,
			_ => false
		}
	}

	///Getting Pointer to Inner
	#[inline]
	pub fn get_var( &self ) -> Option<&'a str> {
		match self {
			&Json::Var(x) => Some( x ),
			_ => None
		}
	}
	#[inline]
	pub fn get_vec( &'a self ) -> Option<&'a Vec<Json<'a>>>{
		match self {
			&Json::Vec( ref x) => Some(x),
			_ => None
		}
	}
	#[inline]
	pub fn get_obj( &'a self ) -> Option<&'a BTreeMap<&'a str,Json<'a>>>{
		match self {
			&Json::Obj( ref x ) => Some(x),
			_ => None
		}
	}

	///Search an Object
	#[inline]
	pub fn search_obj( &'a self, key: &str ) -> Option<&'a Json<'a>>{
		match self {
			&Json::Obj( ref x ) => x.get( key ),
			_ => return None
		}
	}
}


#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::fs::File;
#[test]
fn test_external_json_0() {
	let mut f = match File::open( "bulbasaur.json" ) {
		Ok(x) => x,
		_ => panic!("Failed to open file")
	};
	let mut s = String::new();
	let _ = match f.read_to_string( &mut s ) {
		Ok(x) => x,
		_ => panic!("Failed to read file")
	};
	let o = match Json::new( &s ) {
		Option::Some(x) => x,
		Option::None => panic!("Failed to load")
	};
	assert!( o.is_obj() );
	assert_eq!( o.search_obj("id").unwrap().get_var().unwrap(), "1" );
	assert_eq!( o.search_obj("base_experience").unwrap().get_var().unwrap(), "64" );
	assert_eq!( o.search_obj("height").unwrap().get_var().unwrap(), "7" );
}


//
//Method for parsing Json Number
//
#[inline]
#[allow(dead_code)]
fn read_number<'a>( buffer: &mut CharIter<'a> ) -> Option<&'a str> {
	let start = buffer.position();
	let mut count = start;
	let mut decimal_flag = true;
	loop {
		match buffer.current() {
			Option::Some('0') |
			Option::Some('1') |
			Option::Some('2') |
			Option::Some('3') |
			Option::Some('4') |
			Option::Some('5') |
			Option::Some('6') |
			Option::Some('7') |
			Option::Some('8') |
			Option::Some('9') => { },
			Option::Some('.') => if decimal_flag{
					decimal_flag = false;
				} else {
					return None;
			},
			Option::Some(_) => return Some( buffer.get_sub_string(start,count) ),
			Option::None => return None
		};
		count += 1;
		let _ = buffer.next();
	}
}
#[test]
fn test_read_number() {
	let s = "12345.7,";
	let mut c = CharIter::new( s );
	let _ = c.skip_whitespace();
	let o = read_number( &mut c );
	assert!( o.is_some() );
	assert_eq!( o, Option::Some("12345.7") );

	let o = c.current();
	assert_eq!( o, Option::Some(',') );
}

//
//Method for parsing Json True/False
//
#[inline]
#[allow(dead_code)]
fn read_tf<'a>( buffer: &mut CharIter<'a> ) -> Option<&'a str> {
	let start = buffer.position();
	let mut count = 0usize;
	loop {
		match (buffer.current(), count){
			(Option::Some('f'),0) |
			(Option::Some('a'),1) |
			(Option::Some('l'),2) |
			(Option::Some('s'),3) |
			(Option::Some('e'),4) |
			(Option::Some('t'),0) |
			(Option::Some('r'),1) |
			(Option::Some('u'),2) |
			(Option::Some('e'),3) => {
				count +=1;
				let _ = buffer.next();
			},
			(Option::Some(','),4) |
			(Option::Some(']'),4) |
			(Option::Some('}'),4) |
			(Option::Some(','),5) |
			(Option::Some(']'),5) |
			(Option::Some('}'),5) => return Some( buffer.get_sub_string( start, start + count ) ),
			(Option::Some(x),5) |
			(Option::Some(x),4) => if x.is_whitespace() {
				return Some( buffer.get_sub_string( start, start+count) );
			} else {
				return None;
			},
			_ => {
				return None;
			}
		}
	}
}
#[test]
fn test_read_tf_0() {
	let s ="true,";
	let mut c = CharIter::new(s);
	let o = c.next();
	assert_eq!( o, Option::Some('t') );
	assert_eq!( c.current(), Option::Some('t') );
	let o = read_tf( &mut c );
	assert_eq!( o, Option::Some("true"));
}
#[test]
fn test_read_tf_1() {
	let s ="false,";
	let mut c = CharIter::new(s);
	let o = c.next();
	assert_eq!( o, Option::Some('f') );
	assert_eq!( c.current(), Option::Some('f') );
	let o = read_tf( &mut c );
	assert_eq!( o, Option::Some("false"));
}

//
//Method for parsing Json String
//
#[inline]
#[allow(dead_code)]
fn read_string<'a>( buffer: &mut CharIter<'a> ) -> Option<&'a str> {
	match buffer.current() {
		Option::Some('"') => { let _ = buffer.next(); },
		Option::None => return None,
		_ => { }
	};
	let mut backslash_flag = false;
	let start = buffer.position();
	let mut count = start;
	loop {
		match ( buffer.current(), backslash_flag ) {
			( Option::None, _ ) => return None,
			( Option::Some('"'), true ) => backslash_flag = false,
			( Option::Some('"'), false ) => {
				let rv = Some( buffer.get_sub_string( start, count) );
				let _ = buffer.skip_whitespace();
				return rv;
			},
			( Option::Some('\\'), false ) => backslash_flag = true,
			( Option::Some('\\'), true ) => backslash_flag = false,
			( Option::Some(_), true ) => return None,
			( Option::Some(_), false ) => { }
		};
		count += 1;
		let _ = buffer.next();
	}
}
#[test]
fn test_read_string() {
	let s = "\"HelloWorld\",";
	let mut c = CharIter::new( s );
	let _ = c.next();
	assert_eq!( c.current(), Option::Some('"') );
	let o = read_string( &mut c );
	assert_eq!( o, Option::Some("HelloWorld") );
	let o = c.current();
	assert_eq!( o, Option::Some(',') );
}

//
//Method for reading a String OR a Number
//
#[allow(dead_code)]
fn read_str<'a>( buffer: &mut CharIter<'a> ) -> Option<&'a str> {
	match buffer.current() {
		Option::Some('0') |
		Option::Some('1') |
		Option::Some('2') |
		Option::Some('3') |
		Option::Some('4') |
		Option::Some('5') |
		Option::Some('6') |
		Option::Some('7') |
		Option::Some('8') |
		Option::Some('9') => read_number( buffer ),
		Option::Some('"') => read_string( buffer ),
		Option::Some('t') |
		Option::Some('f') => read_tf( buffer ),
		_ => None
	}
}

//
//Method for reading Json Var
//
#[allow(dead_code)]
fn read_var<'a>( buffer: &mut CharIter<'a> ) -> Option< Json<'a> > {
	match read_str( buffer ) {
		Option::Some( x ) => Some( Json::Var( x ) ),
		_ => None
	}
}

//
//Method for reading a Json Vec
//
#[allow(dead_code)]
fn read_vec<'a>( buffer: &mut CharIter<'a> ) -> Option< Json<'a>> {
	match buffer.current() {
		Option::Some('[') => { let _ = buffer.skip_whitespace(); },
		Option::None => return None,
		_ => { }
	};
	let mut v = Vec::<Json<'a>>::with_capacity(10);
	loop {
		match buffer.current() {
			Option::Some('0') |
			Option::Some('1') |
			Option::Some('2') |
			Option::Some('3') |
			Option::Some('4') |
			Option::Some('5') |
			Option::Some('6') |
			Option::Some('7') |
			Option::Some('8') |
			Option::Some('9') |
			Option::Some('"') |
			Option::Some('[') |
			Option::Some('{') => match read_field( buffer ) {
				Option::Some(x) => v.push(x),
				_ => return None
			},
			Option::Some(',') => { let _ = buffer.skip_whitespace(); },
			Option::Some(']') => {
				let _ = buffer.skip_whitespace();
				return Some( Json::Vec(v) );
			},
			_ => return None
		};
	}
}
#[test]
fn test_read_vec() {
	let s = "[0,\"HelloWorld\",2,[0,\"Hi\"]]";
	let mut c = CharIter::new(s);
	let o = c.next();
	assert_eq!( o, Option::Some('[') );
	let o = read_vec( &mut c );
	let o = o.unwrap();
	let o = match o {
		Json::Vec(x) =>x,
		_ => panic!("Type Mismatch")
	};
	match o[0] {
		Json::Var(x) => assert_eq!( x, "0" ),
		_ => panic!("Type Mismatch")
	};
	match o[1] {
		Json::Var(x) => assert_eq!( x, "HelloWorld" ),
		_ => panic!("Type Mismatch")
	};
	match o[2] {
		Json::Var(x) => assert_eq!( x, "2" ),
		_ => panic!("Type Mismatch")
	};
	match &o[3] {
		&Json::Vec( ref x) => {
			match x[0] {
				Json::Var(y) => assert_eq!(y, "0"),
				_ => panic!("Type Mismatch")
			};
			match x[1] {
				Json::Var(y) => assert_eq!(y, "Hi"),
				_ => panic!("Type Mismatch")
			};
		},
		_ => panic!("Type Mismatch")
	};
}

//
//method for reading a Json Object
//
#[allow(dead_code)]
fn read_obj<'a>( buffer: &mut CharIter<'a> ) -> Option< Json<'a>> {
	match buffer.current() {
		Option::Some('{') => { let _ = buffer.skip_whitespace(); },
		Option::None => return None,
		_ => { }
	};
	let mut b = BTreeMap::<&'a str, Json<'a>>::new();
	loop {
		match buffer.current() {
			Option::Some('0') |
			Option::Some('1') |
			Option::Some('2') |
			Option::Some('3') |
			Option::Some('4') |
			Option::Some('5') |
			Option::Some('6') |
			Option::Some('7') |
			Option::Some('8') |
			Option::Some('9') |
			Option::Some('"') => {
				let key = match read_str( buffer ) {
					Option::Some(x) => x,
					_ => return None
				};
				let _ = match buffer.current() {
					Option::Some(':') => 1usize,
					Option::Some(x) => if x.is_whitespace() {
						match buffer.skip_whitespace() {
							Option::Some(':') => 1usize,
							_ => return None
						}
					} else {
						return None;
					},
					_ => return None
				};
				let _ = buffer.skip_whitespace();
				let value = match read_field( buffer ) {
					Option::Some(x) => x,
					_ => return None
				};
				let _ = b.insert( key, value );
			},
			Option::Some(',') => {
				let _ = buffer.skip_whitespace();
			},
			Option::Some('}') => {
				let _ = buffer.skip_whitespace();
				return Some( Json::Obj(b) );
			},
			_ => return None
		}
	}
}
#[test]
fn test_read_object() {
	let s = "{1: [0,\"HelloWorld\",2,[0,\"Hi\"]] }";
	let mut c = CharIter::new( s );
	let o = c.next();
	assert_eq!( o, Option::Some('{') );
	let o = read_obj( &mut c );
	assert!( o.is_some() );
	let o = o.unwrap();
	assert!( o.is_obj() );
	let o = o.search_obj( "1" ).unwrap();
	assert!( o.is_vec() );
}

#[allow(dead_code)]
fn read_field<'a>( buffer: &mut CharIter<'a> ) -> Option<Json<'a>>{
	match buffer.current() {
		Option::Some('t') |
		Option::Some('f') |
		Option::Some('0') |
		Option::Some('1') |
		Option::Some('2') |
		Option::Some('3') |
		Option::Some('4') |
		Option::Some('5') |
		Option::Some('6') |
		Option::Some('7') |
		Option::Some('8') |
		Option::Some('9') |
		Option::Some('"') => read_var( buffer ),
		Option::Some('{') => read_obj( buffer ),
		Option::Some('[') => read_vec( buffer ),
		_ => None
	}
}
