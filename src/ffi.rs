use std::marker::PhantomData;

pub use std::os::raw::{c_char, c_uint};

use super::types;

pub enum Source {}
pub enum Tempo {}
pub enum Sink {}
pub enum Onset {}

#[repr(C)]
pub struct FVecMut<'a> {
    len: c_uint,
    data: *mut types::Sample,
    _marker: PhantomData<&'a mut Vec<types::Sample>>,
}

pub fn fvec_mut<'a>(vec: &'a mut Vec<types::Sample>) -> FVecMut<'a> {
    FVecMut {
        len: uint(vec.len()),
        data: vec.as_mut_ptr(),
        _marker: PhantomData,
    }
}

#[repr(C)]
pub struct FVec<'a> {
    len: c_uint,
    data: *const types::Sample,
    _marker: PhantomData<&'a [types::Sample]>,
}


pub fn fvec<'a>(slice: &'a [types::Sample]) -> FVec<'a> {
    FVec {
        len: uint(slice.len()),
        data: slice.as_ptr(),
        _marker: PhantomData,
    }
}

pub fn uint(sz: usize) -> c_uint {
    if sz > c_uint::max_value() as usize {
        panic!("sz out of bounds!");
    }

    sz as c_uint
}

#[link(name="aubio")]
extern "C" {
    pub fn new_aubio_source(uri: *const c_char, sample_rate: c_uint, hop_size: c_uint) -> *mut Source;
    pub fn del_aubio_source(source: *mut Source);
    pub fn aubio_source_do(source: *mut Source, fvec: *mut FVecMut, read: *mut c_uint);
    pub fn aubio_source_get_samplerate(source: *const Source) -> c_uint;

    pub fn new_aubio_tempo(method: *const c_char, buf_size: c_uint, hop_size: c_uint, sample_rate: c_uint) -> *mut Tempo;
    pub fn del_aubio_tempo(tempo: *mut Tempo);
    pub fn aubio_tempo_do(tempo: *mut Tempo, imput: *const FVec, tempo: *mut FVecMut);
    pub fn aubio_tempo_get_bpm(tempo: *const Tempo) -> types::Sample;
    pub fn aubio_tempo_get_last_ms(tempo: *const Tempo) -> types::Sample;

    pub fn new_aubio_sink(uri: *const c_char, sample_rate: c_uint) -> *mut Sink;
    pub fn del_aubio_sink(sink: *mut Sink);
    pub fn aubio_sink_get_samplerate(sink: *const Sink) -> c_uint;
    pub fn aubio_sink_do(sink: *mut Sink, write_data: *const FVec, write: c_uint);

    pub fn new_aubio_onset(method: *const c_char, buf_size: c_uint, hop_size: c_uint, sample_rate: c_uint) -> *mut Onset;
    pub fn del_aubio_onset(onset: *mut Onset);
    pub fn aubio_onset_do(onset: *mut Onset, input: *const FVec, onset: *mut FVecMut);
    pub fn aubio_onset_get_last(onset: *mut Onset) -> c_uint;
}
