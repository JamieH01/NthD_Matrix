#[cfg(test)]
mod integration_tests {
    #[allow(unused_imports)]
    use nd_matrix::*;
    use std::time::Instant;
    #[test]#[allow(unused_variables)]
    fn matrix_init() {

        let time = Instant::now();
        let matrix = matrix!([255,255,255,30]; usize, 15);
        println!("Elapsed: {:.2?}\nVec length: {}\nThreads: {}\n", time.elapsed(), matrix.len(), 1);

        let time = Instant::now();
        let matrix = matrix!([255,255,255,30]; usize, 15, 16);
        println!("Elapsed: {:.2?}\nVec length: {}\nThreads: {}", time.elapsed(), matrix.len(), 16);
    }

    #[test]#[allow(unused_variables)]
    fn index_test() {
        let matrix = NdMatrix::<u32>::new(vec![4,3,5], 15, 1);
        let value1 = matrix.pos(vec![3,2,4]).unwrap();

        let value2 = matrix.nth(15).unwrap();
        println!("{value1} {value2}"); 
        println!("length: {} dimensions: {} size: {:?}", matrix.len(), matrix.dim(), matrix.size())       
    }

    #[test]#[allow(unused_variables)]
    fn write_test() {
        let mut matrix = NdMatrix::<u32>::new(vec![4,3,5], 15, 1);
        

        matrix.set_pos(vec![2,2,2], 30);

        let value1 = matrix.pos(vec![0,0,0]).unwrap();
        let value2 = matrix.pos(vec![2,2,2]).unwrap();
        
        matrix.set_pos(vec![15,15,15], 30);

        println!("{value1} {value2}");

    }

    #[test]#[allow(unused_variables)]
    fn position_conversion() {
        let matrix = matrix!([5,5]; usize, 15);

        let starting_index = 17;
        let index_to_pos = matrix.nth_to_pos(starting_index).unwrap();
        let pos_to_index = matrix.pos_to_nth(index_to_pos.clone()).unwrap();

        println!("{starting_index} => {index_to_pos:?} => {pos_to_index}");
    }

    #[test]#[allow(unused_variables)]
    fn arith_test() {
        let mut const_matrix = matrix!([5,5]; usize, 15);
        const_matrix.const_add(12);
        let const_val = const_matrix.nth(3).unwrap();
        
        let mut matrix_a = matrix!([5,5]; usize, 10);        
        let mut matrix_b = matrix!([5,5]; usize, 10); 

        matrix_a.add(matrix_b);
        let val = matrix_a.nth(5).unwrap();




        println!("{const_val} {val}")
    }

    //use::jml::*;
    //use device_query::{DeviceQuery, DeviceState, Keycode};
    //#[test]#[allow(unused_variables)]
    //fn uv_map_test() {
    //    let mut window = WindowContainer::new(255, 255, "UV Map", Color::Black.value());
    //    let mut matrix = matrix!([255, 255, 255]; u32, 0);
//
    //    for i in 0..matrix.len() {
    //        let pos = matrix.nth_to_pos(i).unwrap();
    //        let val = from_u8_rgb(try_it!(pos[0]), try_it!(pos[1]), try_it!(pos[2]));
    //        matrix.set_nth(i, val);
    //    }
//
    //    for i in 0..window.buffer.len() {
    //        let pos = window.buffer.nth_to_pos(i);
    //        window.buffer.set_nth(i, matrix.pos(vec![pos.0, pos.1, 0]).unwrap_or(0))
    //    }
//
    //    let mut depth = 0;
    //    escape_loop! ({
    //        window.update();
    //        depth += 1;
    //        if depth > 255 {depth = 0}
    //        for i in 0..window.buffer.len() {
    //            let pos = window.buffer.nth_to_pos(i);
    //            window.buffer.set_nth(i, matrix.pos(vec![pos.0, pos.1, depth]).unwrap_or(0))
    //        }            
    //    });
    //
    //
    //}

}