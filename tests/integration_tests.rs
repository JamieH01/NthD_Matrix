#[cfg(test)]
mod integration_tests {
    #[allow(unused_imports)]
    use nd_matrix::*;

    #[test]#[allow(unused_variables)]
    fn matrix_init() {
        let matrix = matrix!([4,3]; usize, 15);
    }

    #[test]#[allow(unused_variables)]
    fn index_test() {
        let matrix = NdMatrix::<u32>::new(vec![4,3,5], 15);
        let value1 = matrix.pos(vec![3,2,4]).unwrap();

        let value2 = matrix.nth(15).unwrap();
        println!("{value1} {value2}"); 
        println!("length: {} dimensions: {} size: {:?}", matrix.len(), matrix.dim(), matrix.size())       
    }

    #[test]#[allow(unused_variables)]
    fn write_test() {
        let mut matrix = NdMatrix::<u32>::new(vec![4,3,5], 15);
        
        matrix.set_pos(vec![2,2,2], 30);

        let value1 = matrix.pos(vec![0,0,0]).unwrap();
        let value2 = matrix.pos(vec![2,2,2]).unwrap();

        println!("{value1} {value2}");

    }

    #[test]#[allow(unused_variables)]
    fn position_conversion() {
        let matrix = matrix!([5,5]; usize, 15);

        let starting_index = 17;
        let index_to_pos = matrix.nth_to_pos(starting_index);
        let pos_to_index = matrix.pos_to_nth(index_to_pos.clone());

        println!("{starting_index} => {index_to_pos:?} => {pos_to_index}");
    }
}