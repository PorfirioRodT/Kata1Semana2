extern crate color_convert;
//pub mod common;
pub fn init_color(color: i16, r: i16, g: i16, b: i16) -> i32{}


fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod main{

    use color_convert::color::Color;
    use  color_convert::handles::hex;
    //use common::init_color;
    #[test]
    fn test_hex_handle() {

        let setValue = Color::init("#B22222", true, true, false);

        let hex_vec = hex::handle_hex_value( &setValue).unwrap();

        assert_eq!(vec!["B","2","2","2","2","2"], hex_vec);
            
    }

    #[test]
    fn test_hextorgb() {
        
        let hex_vec = vec!["#80ffffff"];
        let hex_result = vec![
            
            vec!["rgb(128, 255, 255)", "RGB(128, 255, 255)", "rgb(255, 255, 255)"],

        ];

        let test_color = init_color(hex_vec);
        for (i, vec_color) in test_color.iter().enumerate() {

            for (index, color) in vec_color.iter().enumerate(){

                assert_eq!(hex_result[i][index], color.to_rgb().unwrap());
                
            }
            
        }

    }

}