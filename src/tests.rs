extern crate color_convert;
pub mod common;



#[cfg(test)]
mod main{

    use color_convert::color::Color;
    use  color_convert::handles::hex;
    use common::init_color;
    #[test]
    fn test_hex_handle() {

        let setValue = Color::init("#B22222", true, true, false);

        let hex_vec = hex::handle_hex_value( &setValue).unwrap();

        assert_eq!(vec!["b","2","2","2","2","2"], hex_vec);
            
    }

}




