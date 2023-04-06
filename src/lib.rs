

#[allow(dead_code)]

#[macro_export] macro_rules! matrix {
    ($dimensions:tt; $type:ty, $default:tt) => {
        NdMatrix::<$type>::new(vec!$dimensions, $default);
    }
}





        //vec_a is user input to compare against
fn oob(vec_a:&Vec<usize>, vec_b:&Vec<usize>) -> bool {
    for i in 0..vec_a.len() {
        if vec_a[i] >= vec_b[i] {return true}
    }

    return false
}

#[derive(Clone, Debug)]
pub struct NdMatrix<T> {
    data:Vec<T>,//data of the matrix

    dimensions:usize,//how many layers
    size:Vec<usize>,//size of each layer

    length:usize,//length of the vector
}

impl<T: Clone> NdMatrix<T> {
    pub fn new(dim:Vec<usize>, default: T) -> Self {

        let dimensions = dim.len();
        let size = dim;

        let mut length:usize = 1;

        for i in 0..size.len() {
            length *= size[i];
        }
        
        let data = vec![default; length];

        NdMatrix {data, dimensions, size, length}
    }


    pub fn pos(&self, index:Vec<usize>) -> Result<T, Error> {
        if index.len() != self.size.len() {return Err(Error::InvalidDimensions)}
        if oob(&index, &self.size) {return Err(Error::OOBIndex)}

        //fuckin, math n shit
        let mut total:usize = 0;
        for i in 0..self.dimensions {
            total += index[i] * (self.dimensions - (i+1))
        }


        Ok(self.data[total].clone())
    }

    pub fn nth(&self, index:usize) -> Result<T, Error> {
        if index >= self.length {return Err(Error::OOBIndex)}

        Ok(self.data[index].clone())
    }


    pub fn set_pos(&mut self, index:Vec<usize>, value:T) -> Option<Error> {
        if index.len() != self.size.len() {return Some(Error::InvalidDimensions)}
        if oob(&index, &self.size) {return Some(Error::OOBIndex)}
        
        let total = self.pos_to_nth(index);

        self.data[total] = value;

        None //returning None is actually the good path as no errors were returned
    }

    pub fn set_nth(&mut self, index:usize, value:T) -> Option<Error> {
        if index >= self.length {return Some(Error::OOBIndex)}

        self.data[index] = value;

        None
    }

    //thank you chatgpt for writing these 2 methods because i wanted to pull my hair out
    //no idea wtfs goin on in the for loop tho
    pub fn pos_to_nth(&self, pos: Vec<usize>) -> usize {
        let mut result = 0;
        let mut stride = 1;
        for (p, s) in pos.iter().rev().zip(self.size.iter().rev()) {
            result += p * stride;
            stride *= s;
        }
        result
    }

    pub fn nth_to_pos(&self, nth: usize) -> Vec<usize> {
        let mut result = Vec::with_capacity(self.size.len());
        let mut rem = nth;
        for s in self.size.iter().rev() {
            let p = rem % s;
            rem /= s;
            result.push(p);
        }
        result.reverse();
        result
    }

    //properties
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn dim(&self) -> usize {
        self.dimensions
    }

    pub fn size(&self) -> Vec<usize> {
        self.size.clone()
    }

}

#[derive(Clone, Debug)]
pub enum Error {
    InvalidDimensions,
    OOBIndex,
}