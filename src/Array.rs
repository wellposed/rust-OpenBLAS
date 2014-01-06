

/*
NB: only support for striding on major axis.
This is because these matrices are meant to model those
in BLAS and LAPACK. 

*/

#[deriving(Clone)]
pub struct Matrix<Axis,T> {
    dimRow : uint,
    dimCol : uint,
    leadingStride : uint,  // in row major
    elems : ~[T]

}

// #[deriving(Clone)]
pub struct MatrixSlice<'r,Axis,T> {
    dimRow : uint,
    dimCol : uint,
    leadingStride : uint,  // in row major
    elems : &'r[T]

}

// #[deriving(Clone)]
pub struct MatrixMutSlice<'r,Axis,T> {
    dimRow : uint,
    dimCol : uint,
    leadingStride : uint,  // in row major
    elems : &'r mut [T]

}



// can't clone slices! 
impl<'r, Axis, T:Clone>  MatrixMutSlice<'r, Axis, T> 
    { fn into_matrix(&self) -> Matrix<Axis, T>{ 
        Matrix { dimRow: self.dimRow, dimCol: self.dimCol,
         leadingStride: self.leadingStride , elems: self.elems.to_owned() } } }


impl<'r, Axis, T:Clone>  MatrixSlice<'r, Axis, T> 
    { fn into_matrix(&self) -> Matrix<Axis, T>{ 
        Matrix { dimRow: self.dimRow, dimCol: self.dimCol,
         leadingStride: self.leadingStride , elems: self.elems.to_owned() } } }

pub trait MatrixIx {

    
}

// 
pub struct RowMajor ;

pub struct ColMajor ;