// TODO: Proxy script to resize images.

use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageError};
use reqwest::Client;
use std::env;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use bytes::Bytes;
use url::Url;

struct FasterImage {
    source_image: Option<DynamicImage>,
    source_path: String,
    dest_path: String,
    dest_type: ImageType,
}

// Our source and destination image types.
enum ImageType {
    Webp,
    Jpg,
    Png,
}

impl FasterImage {
    fn read_path(&mut self) {
        let paths = fs::read_dir(&self.source_path).unwrap();

        for path in paths {
            let current_img: String = path.unwrap().path().display().to_string();
            let img_path = Path::new(&current_img);
            let file_stem = img_path.file_stem().unwrap();
            let current_file = file_stem.to_str();

            match current_file {
                Some(file_name) => {
                    println!("Optimising {}", current_img);
                    self.open_imageset(
                        current_img.as_str(),
                        format!("{}{}", self.dest_path.clone().as_str(), file_name).as_str(),
                    )
                }
                None => println!("{}", "No extension"),
            }
        }
    }

    fn open_imageset(&mut self, file_path: &str, dest_file: &str) {
        println!("{}", dest_file);
        if let Ok(source) = self.open_image(file_path) {
            //println!("{:?}", source);
            self.source_image = Option::Some(source);
            //TODO: Check if already in cache.
            // Resize three diffrent resolutions
            // 1004
            // 266
            // 728
            // 1506
            // 2245

            if let Err(err) = self.save_image_as_webp(dest_file, 1004, 1004) {
                println!("{}", err);
            }

            if let Err(err) = self.save_image_as_webp(dest_file, 266, 266) {
                println!("{}", err);
            }

            if let Err(err) = self.save_image_as_webp(dest_file, 728, 728) {
                println!("{}", err);
            }

            if let Err(err) = self.save_image_as_webp(dest_file, 1506, 1506) {
                println!("{}", err);
            }

            if let Err(err) = self.save_image_as_webp(dest_file, 2245, 2245) {
                println!("{}", err);
            }
        } else {
            println!("Can not open image {}", file_path)
        }
    }

    fn save_image_as_webp(
        &self,
        dest_file: &str,
        width: u32,
        height: u32,
    ) -> Result<(), ImageError> {
        // destination file type
        let file_extension = match self.dest_type {
            ImageType::Jpg => "jpg".to_string(),
            ImageType::Png => "png".to_string(),
            ImageType::Webp => "webp".to_string(),
        };

        //FIX: Handle error correctly
        if let Some(file_handle) = self.source_image.as_ref() {
            println!("{}.{}", dest_file, file_extension);
            file_handle
                .resize_exact(width, height, image::imageops::FilterType::Lanczos3)
                .save(format!(
                    "{}_{}x{}.{}",
                    dest_file, width, height, file_extension
                ))
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

#[derive(Debug, Deserialize, Serialize)]
struct CosmicResponse {
    objects: Vec<Posts>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Posts {
    slug: String,
    title: String,
    metadata: Option<MetaData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MetaData {
    hero: Option<Hero>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Hero {
    url: String,
}

#[tokio::main]
async fn main() {
    // println!("Enter full path to image folder location to convert to webp");
    // println!("Files are stored in the same path in a sub folder named optimised.");

    //    let mut path_to_images = String::new();

    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // attempt to create optimised folder.
    // let mut fast = FasterImage {
    //     source_path: format!("{}", "/home/sangel/projects/images/"),
    //     dest_path: format!("{}", "/home/sangel/projects/images/"),
    //     dest_type: ImageType::Webp,
    //     source_image: None,
    // };
    // fast.read_path();

    let client = reqwest::Client::new();
    //let response = get_api(&client, "/api/v4/spot/currency_pairs", Option::None).await;
    let response = get_api(&client, "https://api.cosmicjs.com/v3/buckets/dexcelertae2-production/objects?read_key=FPp5lLS4a9dXwtur5SsZzYGuRaE7tuDFrQPYy6CzQyzHAK6ltt&limit=100", "", Option::None).await;
    let response_body = response.text().await.unwrap();
    let sanitized_response = response_body.trim();
    let data = serde_json::from_str::<CosmicResponse>(sanitized_response);
    
    for item in data.unwrap().objects.iter() {
        if let Some(meta_ob) = &item.metadata {
            if let Some(hero) = &meta_ob.hero {
                //println!("{:?}", hero.url);
                let res = get_img(&client, hero.url.as_str()).await;
                let img_bytes = res.bytes().await.unwrap();
                let fname = res.url().path_segments().and_then(|segments| segments.last()).and_then(|name| if name.is_empty() { None } else { Some(name) }).unwrap();
                //println!("{:?}", fname);
                //Do i need to write ??????
                //write_file(img_bytes, "/home/sangel/projects/images/temp.jpg".into());
            }
        }
    }

    //println!("{:?}", data.unwrap().objects);
}


fn extract_filename(url_string: &str) -> Option<String> {
    // Parse the URL
    let url = Url::parse(url_string).ok()?;

    // Extract the path component of the URL
    let path = url.path();

    // Split the path by the '/' delimiter
    let path_segments: Vec<&str> = path.split('/').collect();

    // Get the last segment, which should be the filename
    let filename = path_segments.last()?;

    Some(filename.to_string())
}

fn write_file(data: Bytes, file: String) {
    print!("{}", file);
    fs::write(file, data).expect("Unable to write file");
}

async fn get_img(client: &Client, url: &str) -> reqwest::Response {
    // download crypto icons.
    //let url = format!("{}{}{}", END_POINT_URL, end_point, new_query);
    client.get(url).send().await.unwrap()
}

async fn get_api(
    client: &Client,
    end_point_url: &str,
    end_point: &str,
    query: Option<&str>,
) -> reqwest::Response {
    // return query string or blank add ?
    let new_query = match query {
        Option::Some(qu) => format!("?{}", qu),
        Option::None => "".to_string(),
    };

    // Make a get request to gateio using our endpoint.
    // let client = reqwest::Client::new();
    let url = format!("{}{}{}", end_point_url, end_point, new_query);
    client
        .get(url)
        // .header("Timestamp", time_stamp().to_string())
        // // .header("message", "")
        // .header("Key", KEY)
        // .header(
        //     "Sign",
        //     generate_signature(
        //         "GET",
        //         "/api/v4/spot/candlesticks",
        //         "currency_pair=btc_usdt",
        //         SECRET,
        //     )
        //     .to_string(),
        // )
        .send()
        .await
        .unwrap()
}
