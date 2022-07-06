use std::{
    borrow::Borrow,
    io::{Read, Write},
};
/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    index: usize,
}

// For composability, it is important that `munge` returns an iterator compatible with its input.
// Therefore, we define this output trait with generic implementations on all compatible types,
// and return that instead.

// This is to capture the lifetime 'a when returning an iterator
// from mungle. Without this, compilation fails with error E0700:
//   hidden type for `impl Trait` captures lifetime that does not appear in bounds
// https://doc.rust-lang.org/error-index.html#E0700
pub trait Captures<'a> {}
// This is to avoid error E0277
//   Captures<'a>` is not satisfied
// https://doc.rust-lang.org/error-index.html#E0277
impl<'a, T> Captures<'a> for T {}
struct Reader<'a, T: Read> {
    pub inner: T,
    pub xor: Xorcism<'a>,
}
impl<'a, T: Read> Read for Reader<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.inner.read(buf)?;
        self.xor.munge_in_place(buf);
        Ok(len)
    }
}
struct Writer<'a, T: Write> {
    pub inner: T,
    pub xor: Xorcism<'a>,
}
impl<'a, T: Write> Write for Writer<'a, T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let xored_buf: Vec<_> = self.xor.munge(buf).collect();
        let len = self.inner.write(&xored_buf)?;
        Ok(len)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        unimplemented!()
    }
}
/// A munger which XORs a key with some data
impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.

    // AsRef<[u8]> means it can be cheaply converted to a byte slice
    // ?Sized means the size is not known at compile time
    pub fn new<T: AsRef<[u8]> + ?Sized>(key: &'a T) -> Xorcism<'a> {
        Xorcism {
            key: key.as_ref(),
            index: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut().for_each(|byte| *byte ^= self.next_byte());
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.

    // In Rust, it is common to provide different representations of a type for different
    // use cases.
    // These types provide access to the underlying data through references to the type
    // of that data. They are said to be 'borrowed as' that type.
    // Types express that they can be borrowed as some type T by implementing Borrow<T>,
    // providing a reference to a T in the trait’s borrow method.
    pub fn munge<'b, T, U>(&'b mut self, data: T) -> impl Iterator<Item = u8> + 'b + Captures<'a>
    where
        T: IntoIterator<Item = U> + 'b,
        U: Borrow<u8>,
        // The colon is read "outlives", so a outlives b
        'a: 'b,
    {
        data.into_iter().map(move |x| x.borrow() ^ self.next_byte())
    }
    fn next_byte(&mut self) -> u8 {
        let index = self.index;
        self.index = (self.index + 1) % self.key.len();
        self.key[index]
    }
    pub fn reader(self, reader: impl Read + 'a) -> impl Read + 'a {
        Reader {
            inner: reader,
            xor: self,
        }
    }
    pub fn writer(self, writer: impl Write + 'a) -> impl Write + 'a {
        Writer {
            inner: writer,
            xor: self,
        }
    }
}
