
use std::str::Chars;


pub struct CharIter<'a>{
	previous: Option<char>,
	buffer: &'a str,
	iterator: Chars<'a>,
	position: usize
}
impl<'a> CharIter<'a> {

	//
	//Constructor
	//

	///Creates a new CharIter
	#[inline]
	#[allow(dead_code)]
	pub fn new<'b>( s: &'b str ) -> CharIter<'b>{
		CharIter{
			previous: None,
			buffer: s,
			iterator: s.chars(),
			position: 0
		}
	}

	//
	//Read Only Properties
	//

	///Returns the previous character in the buffer
	#[inline]
	#[allow(dead_code)]
	pub fn current( &self ) -> Option< char > {
		self.previous
	}
	///Returns the position in buffer
	#[inline]
	#[allow(dead_code)]
	pub fn position( &self ) -> usize {
		self.position - 1
	}
	///Returns the length of the buffer
	#[inline]
	#[allow(dead_code)]
	pub fn len( &self ) -> usize {
		self.buffer.len()
	}
	///Return buffer
	#[inline]
	#[allow(dead_code)]
	pub fn get_buffer( &self ) -> &'a str {
		self.buffer
	}
	///Return a sub string
	#[inline]
	#[allow(dead_code)]
	pub fn get_sub_string( &self, start: usize, end: usize ) -> &'a str {
		unsafe{ self.buffer.slice_unchecked( start, end ) }
	}

	//
	//Mutate State:
	//

	///Returns the next character in the iterator
	#[inline]
	#[allow(dead_code)]
	pub fn next( &mut self ) -> Option< char > {
		self.position += 1;
		self.previous = self.iterator.next();
		self.previous
	}
	///Skips white space
	#[inline]
	#[allow(dead_code)]
	pub fn skip_whitespace( &mut self ) -> Option<char> {
		loop {
			match self.next() {
				Option::None => return None,
				Option::Some(x) => if x.is_whitespace() { continue } else { return Some(x) }
			}
		}
	}
}

#[test]
fn test_char_iter_skip_whitespace() {
	let s = "     5";
	let mut c = CharIter::new( s );
	let o = c.skip_whitespace();
	assert_eq!( o, Option::Some('5') );
}

#[test]
fn test_char_iter_next() {
	let s = "563";
	let mut c = CharIter::new( s );
	let o = c.current();
	assert_eq!( o, Option::None );
	let o = c.next();
	assert_eq!( o, Option::Some('5') );
	let o = c.current();
	assert_eq!( o, Option::Some('5') );
	let o = c.next();
	assert_eq!( o, Option::Some('6') );
}
