
use std::str::Chars;


pub struct CharIter<'a>{
	previous: Option<char>,
	buffer: &'a str,
	iterator: Chars<'a>,
	position: usize
}

#[allow(dead_code)]
impl<'a> CharIter<'a> {
	//
	//Constructor
	//

	///Creates a new CharIter
	#[inline]
	pub fn new( s: &'a str ) -> CharIter<'a> {
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
	pub fn current( &self ) -> Option< char > {
		self.previous
	}
	///Returns the position in buffer
	#[inline]
	pub fn position( &self ) -> usize {
		self.position - 1
	}
	///Returns the length of the buffer
	#[inline]
	pub fn len( &self ) -> usize {
		self.buffer.len()
	}

	///Returns true if the buffer is empty, false otherwise
	#[inline]
	pub fn is_empty( &self ) -> bool {
		self.buffer.is_empty()
	}

	///Return buffer
	#[inline]
	pub fn get_buffer( &self ) -> &'a str {
		self.buffer
	}
	///Return a sub string
	#[inline]
	pub fn get_sub_string( &self, start: usize, end: usize ) -> &'a str {
		unsafe{ self.buffer.slice_unchecked( start, end ) }
	}

	//
	//Mutate State:
	//

	///Returns the next character in the iterator
	#[inline]
	pub fn next( &mut self ) -> Option< char > {
		self.position += 1;
		self.previous = self.iterator.next();
		self.previous
	}

	///Skips white space
	#[inline]
	#[allow(dead_code)]
	pub fn skip_whitespace( &mut self ) -> Option<char> {
		while let Some(x) = self.next() {
			if !x.is_whitespace() { return Some(x) }
		}
		None
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
