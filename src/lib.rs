

#[allow(dead_code)]

#[macro_export] macro_rules! matrix {
    ($dimensions:tt; $type:ty, $default:tt) => {
        NdMatrix::<$type>::new(vec!$dimensions, $default, 1);
    };

    ($dimensions:tt; $type:ty, $default:tt, $threads:tt) => {
        NdMatrix::<$type>::new(vec!$dimensions, $default, $threads);
    }

}





        //vec_a is user input to compare against
fn oob(vec_a:&Vec<usize>, vec_b:&Vec<usize>) -> bool {
    for i in 0..vec_a.len() {
        if vec_a[i] >= vec_b[i] {return true}
    }

    return false
}

#[derive(Debug, Clone, PartialEq)]
pub struct NdMatrix<T> {
    pub data:Vec<T>,//data of the matrix

    dimensions:usize,//how many layers
    size:Vec<usize>,//size of each layer

    length:usize,//length of the vector

    count:usize,//iterator count
}

use std::thread;
impl<T: Clone + Send + 'static> NdMatrix<T> {
    pub fn new(dim:Vec<usize>, default: T, threads:usize) -> Self {

        let dimensions = dim.len();
        let size = dim;

        let mut length:usize = 1;

        for i in 0..size.len() {
            length *= size[i];
        }
        
        //MULTITHREADING MAGIC
        let split = length / threads;
        let remod = length % threads;

        let mut thread_table:Vec<thread::JoinHandle<Vec<T>>> = vec![];

        for _ in 0..threads {//thread inits
            let thr_split = split;
            let thr_default = default.clone();

            thread_table.push(thread::spawn(move || {
                vec![thr_default; thr_split]
            }));
        }

        let mut data:Vec<T> = vec![];
        for thread in thread_table {
            let mut split_table = thread.join().unwrap();

            data.append(&mut split_table);
        }
        data.append(&mut vec![default; remod]);
        
        //let data = vec![default; length];
        
        NdMatrix {data, dimensions, size, length, count:0}
    }


    pub fn pos(&self, index:Vec<usize>) -> Result<T, Error> {
        if index.len() != self.size.len() {return Err(Error::InvalidDimensions)}
        if oob(&index, &self.size) {return Err(Error::OOBIndex)}


        let total = self.pos_to_nth(index).unwrap();
        
        Ok(self.data[total].clone())
    }

    pub fn nth(&self, index:usize) -> Result<T, Error> {
        if index >= self.length {return Err(Error::OOBIndex)}

        Ok(self.data[index].clone())
    }


    pub fn set_pos(&mut self, index:Vec<usize>, value:T) -> Result<(), Error> {
        if index.len() != self.size.len() {return Err(Error::InvalidDimensions)}
        if oob(&index, &self.size) {return Err(Error::OOBIndex)}
        
        let total = self.pos_to_nth(index).unwrap();

        self.data[total] = value;

        Ok(()) //returning None is actually the good path as no errors were returned
    }

    pub fn set_nth(&mut self, index:usize, value:T) -> Result<(), Error> {
        if index >= self.length {return Err(Error::OOBIndex)}

        self.data[index] = value;

        Ok(())
    }


    //thank you chatgpt for writing these 2 methods because i wanted to pull my hair out
    //no idea wtfs goin on in the for loop tho
    pub fn pos_to_nth(&self, index:Vec<usize>) -> Result<usize, Error> {
        if index.len() != self.size.len() {return Err(Error::InvalidDimensions)}
        if oob(&index, &self.size) {return Err(Error::OOBIndex)}       
        
        let mut result = 0;
        let mut stride = 1;
        for (p, s) in index.iter().rev().zip(self.size.iter().rev()) {
            result += p * stride;
            stride *= s;
        }
        Ok(result)
    }

    pub fn nth_to_pos(&self, index:usize) -> Result<Vec<usize>, Error> {
        if index >= self.length {return Err(Error::OOBIndex)}

        let mut result = Vec::with_capacity(self.size.len());
        let mut rem = index;
        for s in self.size.iter().rev() {
            let p = rem % s;
            rem /= s;
            result.push(p);
        }
        result.reverse();
        Ok(result)
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


impl<T: Clone + Send + 'static> Iterator for NdMatrix<T> {
    type Item = (T, usize, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.len() {
            let item = self.data[self.count].clone();
            let index = self.count;
            let position = self.nth_to_pos(self.count).unwrap();
            self.count += 1;
            Some((item, index, position))
        }else{None}
    }
}


//arithmetic
impl<T:num_traits::Num + Clone + Copy + Send + 'static +
num_traits::CheckedAdd + num_traits::CheckedSub + num_traits::CheckedMul + num_traits::CheckedDiv> 
NdMatrix<T> {
    //basic arithmetic
    pub fn add(&mut self, operand:NdMatrix<T>) -> Result<(), Error> {
        if self.dimensions != operand.dimensions {return Err(Error::InvalidDimensions)}
        
        let res = self.data[0].checked_add(&operand.data[0]);
        if res == None {return Err(Error::CannotOperate)}

        for i in 0..self.len() {
            self.data[i] = self.data[i] + operand.data[i]
        }

        Ok(())
    }
    
    pub fn sub(&mut self, operand:NdMatrix<T>) -> Result<(), Error> {
        if self.dimensions != operand.dimensions {return Err(Error::InvalidDimensions)}

        let res = self.data[0].checked_sub(&operand.data[0]);
        if res == None {return Err(Error::CannotOperate)}

        for i in 0..self.len() {
            self.data[i] = self.data[i] - operand.data[i]
        }

        Ok(())
    }

    pub fn mul(&mut self, operand:NdMatrix<T>) -> Result<(), Error> {
        if self.dimensions != operand.dimensions {return Err(Error::InvalidDimensions)}

        let res = self.data[0].checked_mul(&operand.data[0]);
        if res == None {return Err(Error::CannotOperate)}

        for i in 0..self.len() {
            self.data[i] = self.data[i] * operand.data[i]
        }

        Ok(())
    }
    
    pub fn div(&mut self, operand:NdMatrix<T>) -> Result<(), Error> {
        //error catches
        if self.dimensions != operand.dimensions {return Err(Error::InvalidDimensions)}

        let res = self.data[0].checked_div(&operand.data[0]);
        if res == None {return Err(Error::CannotOperate)}

        for i in 0..self.len() {
            self.data[i] = self.data[i] / operand.data[i]
        }

        Ok(())
    }


    //const arithmetic
    pub fn const_add(&mut self, operand:T) {
        
        for i in 0..self.len() {
            self.data[i] = self.data[i] + operand
        }

    }

    pub fn const_sub(&mut self, operand:T) {
        
        for i in 0..self.len() {
            self.data[i] = self.data[i] - operand
        }
      
    }

    pub fn const_mul(&mut self, operand:T) {
        
        for i in 0..self.len() {
            self.data[i] = self.data[i] * operand
        }
     
    }

    pub fn const_div(&mut self, operand:T) {
        
        for i in 0..self.len() {
            self.data[i] = self.data[i] / operand
        }
     
    }

}




#[derive(Clone, Debug)]
pub enum Error {
    InvalidDimensions,
    OOBIndex,
    CannotOperate,
}