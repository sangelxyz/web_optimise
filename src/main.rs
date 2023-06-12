// TODO: Add command line options.
// Path to convert.
//

use image::{ImageError, DynamicImage};
use image::io::Reader as ImageReader;
use std::{fs,io};
use std::path::Path;

struct FasterImage {
    source_image : Option<DynamicImage>,
    source_path : String,
    dest_path : String,
    dest_type : ImageType,
}

// Our source and destination image types.
enum ImageType {
    Webp,
    Jpg,
    Png,
}

impl FasterImage {
    fn read_path (&mut self) {
        let paths = fs::read_dir(&self.source_path).unwrap();
        //let image_file_name = path::
        for path in paths {
            let current_img: String = path.unwrap().path().display().to_string();

            let img_path = Path::new(&current_img);
            let file_stem = img_path.file_stem().unwrap();
            let current_file = file_stem.to_str();

            match current_file {
                Some(file_name) => {
                    println!("Optimising {}", current_img);
                    self.open_imageset(current_img.as_str(), format!("{}{}", self.dest_path.clone().as_str(), file_name).as_str())
                },
                None => println!("{}", "No extension"),
            }     
        }
    }

    fn open_imageset(&mut self, file_path: &str, dest_file: &str) {
        // "/home/sangel/projects/images/1.png"
        // Source Image
        println!("{}", dest_file);
        if let Ok(source) = self.open_image(file_path) {
            //println!("{:?}", source);
            self.source_image = Option::Some(source);
            if let Err(err) = self.save_image_as_webp(dest_file) {
               println!("{}", err); 
            }
        } else {
            panic!("can not open source image");
        }
     }

    fn save_image_as_webp(&self, dest_file: &str) -> Result<(), ImageError> {
        
        // destination file type
        let file_extension = match self.dest_type {
            ImageType::Jpg => "jpg".to_string(),
            ImageType::Png => "png".to_string(),
            ImageType::Webp => "webp".to_string(),
        };

        //FIX: Handle error correctly
        if let Some(file_handle) = self.source_image.as_ref() {
            println!("{}.{}", dest_file, file_extension);
            file_handle.save(format!("{}.{}", dest_file, file_extension))
            //Ok(())
        } else {
           Ok(()) 
        } 
    }

    fn open_image(&self, file_path: &str) -> Result<DynamicImage, ImageError> {
        //TODO:  check file type by meta data
        let img = ImageReader::open(format!("{}", file_path));

        match img {
            Result::Ok(img_data) => img_data.decode(),
            Result::Err(err) => Err(err.into()),
        }
        
    }
}

fn main() {
    println!("Enter full path to image folder location to convert to webp");
    println!("Files are stored in the same path in a sub folder named optimised.");

    let mut path_to_images = String::new();

    io::stdin()
        .read_line(&mut path_to_images)
        .expect("Failed to read input directory.");

    // attempt to create optimised folder.
    


    let mut fast = FasterImage {
        source_path : format!("{}", path_to_images),
        dest_path : format!("{}", path_to_images),
        dest_type : ImageType::Webp,
        source_image : None,
    };
    fast.read_path();
}
