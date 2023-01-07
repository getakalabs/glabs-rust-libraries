use image::{GenericImageView, ImageFormat, Rgba};
use image::imageops::FilterType;
use infer::Infer;
use reqwest;
use rusoto_core::credential::{StaticProvider};
use rusoto_core::{HttpClient, Region};
use rusoto_s3::{PutObjectRequest, S3 as RusotoS3, S3Client};
use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::default::Default;
use std::fs::File as StdFile;
use std::io::{Cursor, Read};
use std::str::FromStr;
use bstr::ByteSlice;


// TODO: Use OpenCV to detect faces or content aware and use it as gravity
// use opencv::objdetect::CascadeClassifierTrait;
// use opencv::prelude::MatTraitConstManual;
// use opencv::{
//     imgcodecs,
//     core::{
//         Mat, Size,
//         Point, Rect,
//     },
//     objdetect::CascadeClassifier,
// };

use crate::{Errors, File};
use crate::strings;

/// Struct container for s3
#[derive(Debug, Clone, PartialEq, Sanitize, Serialize, Deserialize)]
pub struct S3 {
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub access_key_id: String,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub secret_access_key: String,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub bucket: String,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub region: String,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub module_profile_picture: String,
    pub image_small_size: usize,
    pub image_medium_size: usize,
    pub image_large_size: usize,
    pub image_xls_size: usize,
}

/// Default implementation for S3
impl Default for S3 {
    fn default() -> Self {
        Self {
            access_key_id: String::default(),
            secret_access_key: String::default(),
            bucket: String::default(),
            path: String::default(),
            region: String::default(),
            module_profile_picture: String::default(),
            image_small_size: 0,
            image_medium_size: 0,
            image_large_size: 0,
            image_xls_size: 0,
        }
    }
}

/// S3 implementation
impl S3 {
    /// Create new S3 instance
    ///
    /// Example
    /// ```
    /// use library::S3;
    ///
    /// fn main() {
    ///     let s3 = S3::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            access_key_id: String::default(),
            secret_access_key: String::default(),
            bucket: String::default(),
            path: String::default(),
            region: String::default(),
            module_profile_picture: String::from("Profile Picture"),
            image_small_size: 72,
            image_medium_size: 192,
            image_large_size: 512,
            image_xls_size: 1024,
        }
    }

    /// Clear current instance
    ///
    /// Example
    /// ```
    /// use library::S3;
    ///
    /// fn main() {
    ///     // Create new s3 instance with default values
    ///     let mut s3 = S3::new();
    ///     s3.clear();
    /// }
    /// ```
    pub fn clear(&mut self) -> Self {
        Self::default()
    }

    /// Reconfigure instance
    ///
    /// Example
    /// ```
    /// use library::S3;
    ///
    /// fn main() {
    ///     // Create old s3 instance with default values
    ///     let mut old_s3 = S3::new();
    ///
    ///     // Create new s3 instance with new values
    ///     let mut new_s3 = S3::new();
    ///     new_s3.access_key_id = String::from("Some-ID-Here");
    ///
    ///     // Reconfigure
    ///     old_s3.reconfigure(&new_s3);
    /// }
    /// ```
    pub fn reconfigure(&mut self, item: &S3) {
        self.access_key_id = item.clone().access_key_id;
        self.secret_access_key = item.clone().secret_access_key;
        self.bucket = item.clone().bucket;
        self.path = item.clone().path;
        self.region = item.clone().region;
        self.module_profile_picture = item.clone().module_profile_picture;
        self.image_small_size = item.clone().image_small_size;
        self.image_medium_size = item.clone().image_medium_size;
        self.image_large_size = item.clone().image_large_size;
        self.image_xls_size = item.clone().image_xls_size;
    }

    /// Convert custom struct type to S3
    ///
    /// Example
    /// ```
    /// use library::S3;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct S32 {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub access_key_id: Option<String>,
    /// 	#[serde(skip_serializing_if = "Option::is_none")]
    ///     pub secret_access_key: Option<String>,
    /// }
    ///
    /// fn main() {
    ///     let s3 = S3::from(S32{access_key_id: Some(String::from("AccessKeyIDHere")), secret_access_key: None});
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from<T>(input: T) -> Self
        where T: Serialize
    {
        let s = serde_json::to_string(&input).unwrap_or(String::default());
        serde_json::from_str(&s).unwrap_or(S3::default())
    }

    /// Convert custom struct type to S3
    ///
    /// Example
    /// ```
    /// use library::S3;
    ///
    /// fn main() {
    ///     let input = r#"{"access_key_id": "ABC1234", "secret_access_key": "ABC1234"}"#;
    ///     let s3 = S3::from_string(input);
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from_string<T: Into<String>>(input: T) -> Self {
        let bindings = input.into();
        serde_json::from_str(&bindings).unwrap_or(S3::default())
    }

    /// Convert custom struct type from S3 to T
    ///
    /// Example
    /// ```
    /// use library::S3;
    /// use serde::{Serialize, Deserialize};
    ///
    /// /// Struct container for S32
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct S32 {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub access_key_id: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub secret_access_key: Option<String>
    /// }
    ///
    /// /// Note: this is always required. Default implementation for S3
    /// impl Default for S32 {
    ///     fn default() -> Self {
    ///         Self {
    ///             access_key_id: None,
    ///             secret_access_key: None
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let input = r#"{"access_key_id": "ABC1234", "secret_access_key": "ABC1234"}"#;
    ///     let s3 = S3::from_string(input);
    ///
    ///     let s32 = s3.to::<S32>();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn to<T>(&self) -> T
        where T: serde::de::DeserializeOwned + Default
    {
        let value = self.clone();
        let s = serde_json::to_string(&value).unwrap_or(String::default());
        let response: T = serde_json::from_str(&s).unwrap_or(T::default());

        response
    }

    /// Check if s3s has no value
    ///
    /// Example
    /// ```
    /// // Import s3
    /// use library::S3;
    ///
    /// fn main() {
    ///     // Set s3
    ///     let s3 = S3::new();
    ///     let is_empty = s3.is_empty();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }

    /// Normalize s3 by performing sanitation and other important stuff
    ///
    /// Example
    /// ```
    /// // Import s3
    /// use library::S3;
    ///
    /// fn main() {
    ///     // Set s3
    ///     let mut s3 = S3::new();
    ///     let form = s3.normalize();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn normalize(&mut self) -> &mut Self {
        self.sanitize();
        self
    }

    /// Retrieve client
    ///
    /// Example
    /// ```
    /// use library::s3::S3;
    ///
    /// fn main() {
    ///     // Set new s3 instance
    ///     let s3 = S3::default();
    ///     let result = s3.get_client();
    /// }
    /// ```
    pub fn get_client(&self) -> Option<S3Client> {
        // Client is unavailable
        if self.is_empty() {
            return None;
        }

        // Set access, secret access key & region
        let access_key = self.access_key_id.clone();
        let secret_access_key = self.secret_access_key.clone();
        let region = Region::from_str(&self.region);
        if region.is_err() {
            return None;
        }

        // Unwrap region
        let region = region.unwrap();

        // Set aws credentials
        let credentials = StaticProvider::new_minimal(access_key, secret_access_key);

        // Set client
        let client = S3Client::new_with(
            HttpClient::new().expect("Failed to create request dispatcher"),
            credentials,
            region,
        );

        // Return client
        Some(client)
    }

    /// Upload file from url
    pub async fn upload_from_url<U, F>(&self, url: U, file_name: F) -> Result<(), Errors>
        where U: Into<String>,
              F: Into<String>
    {
        // Retrieve client
        let client = self.get_client();
        if client.is_none() {
            return Err(Errors::new("S3 client failed to initialize"));
        }

        // Shadow client
        let client = client.unwrap();

        // Set bindings
        let url_bindings = url.into();
        let filename = file_name.into();
        let filename = strings::replace_filename(filename, "original");

        // Download the image from the URL
        let response = reqwest::get(url_bindings).await;
        if response.is_err() {
            return Err(Errors::new("Unable download file from url"));
        }

        // Check bytes
        let response = response.unwrap().bytes().await;
        if response.is_err() {
            return Err(Errors::new("Unable to download file from url"));
        }

        // Create image buffer
        let cursor = Cursor::new(response.unwrap().as_bytes().to_vec());
        let buffer = cursor.get_ref();

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&buffer.clone())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Retrieve content type
        let extension = strings::get_extension_from_mime(&mime);
        let filename = strings::change_extension(filename, extension);

        // Set metadata
        let mut metadata = HashMap::new();
        metadata.insert(String::from("filename"), filename.clone());

        // Upload original image to s3
        let request = PutObjectRequest {
            metadata: Some(metadata),
            bucket: self.bucket.to_owned(),
            key: format!("{}/{}", self.path, filename),
            body: Some(buffer.clone().into()),
            acl: Some("public-read".to_owned()),
            content_type: Some(mime),
            ..Default::default()
        };

        // Upload file
        let result = client.put_object(request).await;
        if result.is_err() {
            return Err(Errors::new("Unable to upload your file"));
        }

        Ok(())
    }

    /// Upload original image. This will add `-original` to your filename before uploading
    pub async fn upload_original<T>(&self, data: Vec<u8>, file_name: T, sizes: Option<Vec<(u32, u32)>>) -> Result<(), Errors>
        where T: Into<String>
    {
        // Retrieve client
        let client = self.get_client();
        if client.is_none() {
            return Err(Errors::new("S3 client failed to initialize"));
        }

        // Shadow client
        let client = client.unwrap();

        // Create bindings
        let file_name_bindings = file_name.into();

        // Set filename
        let filename = strings::replace_filename(file_name_bindings, "original");

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&data.clone())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Set metadata
        let mut metadata = HashMap::new();
        metadata.insert(String::from("filename"), filename.clone());

        // Upload original image to s3
        let request = PutObjectRequest {
            metadata: Some(metadata),
            bucket: self.bucket.to_owned(),
            key: format!("{}/{}", self.path, filename.clone()),
            body: Some(data.clone().into()),
            acl: Some("public-read".to_owned()),
            content_type: Some(mime.clone()),
            ..Default::default()
        };

        // Upload file
        let result = client.put_object(request).await;
        if result.is_err() {
            return Err(Errors::new("Unable to upload your file"));
        }

        // Check if current mime type is image
        if sizes.is_some() && File::is_image(mime) {
            for (width, height) in sizes.unwrap() {
                let retain_size = false;
                let result = self.generate_thumbnail(data.clone(), &filename, width as u32, height as u32, retain_size).await;
                if result.is_err() {
                    return result;
                }
            }
        }

        Ok(())
    }

    /// Upload a thumbnail of image
    pub async fn generate_thumbnail<T>(&self, data: Vec<u8>, file_name: T, width: u32, height: u32, retain_size: bool) -> Result<(), Errors>
        where T: Into<String>
    {
        // Create filename bindings
        let filename = file_name.into();
        let filename = strings::replace_filename(filename, format!("{}x{}", width, height));
        let filename = strings::change_extension(filename, "png");

        // Retrieve client
        let client = self.get_client();
        if client.is_none() {
            return Err(Errors::new("S3 client failed to initialize"));
        }

        // Shadow client
        let client = client.unwrap();

        // Create image buffer
        let cursor = Cursor::new(data.clone());
        let buffer = cursor.get_ref();

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&buffer.clone())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Check if data is image
        if !File::is_image(mime) {
            return Err(Errors::new("Invalid image type"));
        }

        // Load image from data
        let image = image::load_from_memory(&data);
        if image.is_err() {
            return Err(Errors::new("Unable to load image"));
        }

        // Shadow image
        let image = image.unwrap();

        // Calculate the size of the thumbnail
        let (orig_width, orig_height) = image.dimensions();
        let ratio = f64::min( orig_width as f64 / width as f64, orig_height as f64 / height as f64);
        let new_width = (orig_width as f64 / ratio) as u32;
        let new_height = (orig_height as f64 / ratio) as u32;

        let mut thumbnail = if retain_size {
            // image.resize(orig_width, orig_height, FilterType::Lanczos3)
            image
        } else {
            image.resize(new_width, new_height, FilterType::Triangle)
        };

        // Crop the image to a square with the center as the gravity
        let (thumb_width, thumb_height) = thumbnail.dimensions();

        // Convert to f64
        let x:f64 = (thumb_width as f64 - width as f64) / 2.0;
        let y:f64 = (thumb_height as f64 - height as f64) / 2.0;

        // Round images to u32
        let x = x.round() as u32;
        let y = y.round() as u32;

        thumbnail = thumbnail.crop(x, y, width, height);

        // Add transparent padding if needed
        let mut padded_thumbnail = image::ImageBuffer::new(width, height);
        let transparent = Rgba([0, 0, 0, 0]);
        for (_, _, pixel) in padded_thumbnail.enumerate_pixels_mut() {
            *pixel = transparent;
        }

        // Set overlay
        image::imageops::overlay(&mut padded_thumbnail, &thumbnail, x as i64, y as i64);

        // Open the file and read its contents
        let mut cursor = Cursor::new(vec![]);
        let result = thumbnail.write_to(&mut cursor, ImageFormat::Png);
        if result.is_err() {
            return Err(Errors::new("Thumbnail generation failed"));
        }

        // Set buffer
        let buffer = cursor.get_ref();

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&data.clone())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Set metadata
        let mut metadata = HashMap::new();
        metadata.insert(String::from("filename"), filename.clone());

        // Upload original image to s3
        let request = PutObjectRequest {
            metadata: Some(metadata),
            bucket: self.bucket.to_owned(),
            key: format!("{}/{}", self.path, filename),
            body: Some(buffer.clone().into()),
            acl: Some("public-read".to_owned()),
            content_type: Some(mime),
            ..Default::default()
        };

        // Upload file
        let result = client.put_object(request).await;
        if result.is_err() {
            return Err(Errors::new("Unable to upload your file"));
        }

        Ok(())
    }

    /// Test out s3 config and upload
    pub async fn test_image_upload(&self) -> Result<(), Errors> {
        use std::time::Instant;
        let start = Instant::now();

        // Set filename
        let file_name = "nature-1-image.jpg";

        // Set path of sample upload
        let stream = StdFile::open(format!("./assets/sample/{}", file_name));
        if stream.is_err() {
            return Err(Errors::new(format!("Sample {} not found in path", file_name)));
        }

        // Unwrap stream
        let mut stream = stream.unwrap();
        let mut contents: Vec<u8> = Vec::new();

        // Read file to end
        let result = stream.read_to_end(&mut contents);
        if result.is_err() {
            return Err(Errors::new("Unable to read file"));
        }

        // Create vector of width and height
        let sizes = Some(vec![
            (self.image_small_size.clone() as u32, self.image_small_size.clone() as u32),
            (self.image_medium_size.clone() as u32, self.image_medium_size.clone() as u32),
            (self.image_large_size.clone() as u32, self.image_large_size.clone() as u32),
            (self.image_xls_size.clone() as u32, self.image_xls_size.clone() as u32),
        ]);

        // Upload file
        let result = self.upload_original(contents.clone(), file_name, sizes).await;
        if result.is_err() {
            return result;
        }

        let duration = start.elapsed();
        println!("Time elapsed is: {:?}", duration);

        Ok(())
    }

    /// Test out s3 config and upload
    pub async fn test_doc_upload(&self) -> Result<(), Errors> {
        // Retrieve client
        let client = self.get_client();
        if client.is_none() {
            return Err(Errors::new("S3 client failed to initialize"));
        }

        // Shadow client
        let client = client.unwrap();

        // Set path of sample upload
        let stream = StdFile::open("./assets/sample/doc.txt");
        if stream.is_err() {
            return Err(Errors::new("Sample doc.txt not found in path"));
        }

        // Unwrap stream
        let mut stream = stream.unwrap();
        let mut contents: Vec<u8> = Vec::new();

        // Read file to end
        let result = stream.read_to_end(&mut contents);
        if result.is_err() {
            return Err(Errors::new("Unable to read file"));
        }

        // Set info
        let info = Infer::new();
        let mut mime: Option<String> = None;

        // Check out mime type
        let mime_type = info.get(&contents.clone());
        if mime_type.is_some() && !mime_type.clone().unwrap().mime_type().is_empty() {
            mime = Some(String::from(mime_type.clone().unwrap().mime_type()));
        }

        // Upload file
        let req = PutObjectRequest {
            bucket: self.bucket.to_owned(),
            key: format!("{}/doc.txt", self.path),
            body: Some(contents.into()),
            acl: Some("public-read".to_owned()),
            content_type: mime,
            ..Default::default()
        };

        let result = client.put_object(req).await;
        if result.is_err() {
            return Err(Errors::new("Unable to read file"));
        }

        Ok(())
    }

    // ToDo: Update the code to work properly with OpenCV
    // /// Upload original image. This will add `-original` to your filename before uploading
    // pub async fn generate_thumbnail(&self, data: Vec<u8>, file_name: T, width: u32, height: u32, retain_size: bool) -> Result<(), Errors>
    //     where T: Into<String>
    // {
    //     // Create filename bindings
    //     let filename = file_name.into();
    //     let filename = strings::replace_filename(filename, format!("{}x{}", width, height));
    //
    //     // Retrieve client
    //     let client = self.get_client();
    //     if client.is_none() {
    //         return Err(Errors::new("S3 client failed to initialize"));
    //     }
    //
    //     // Shadow client
    //     let client = client.unwrap();
    //
    //     // Load image from data
    //     let image = image::load_from_memory(&data)?;
    //
    //     // Retrieve dimension
    //     let (orig_width, orig_height) = image.dimensions();
    //     let thumbnail = if orig_width > width || orig_height > height {
    //         // Resize original image if its dimensions are larger than the thumbnail dimensions
    //         image.resize(width, height, FilterType::Nearest)
    //     } else {
    //         image
    //     };
    //
    //     let mut thumbnail_data = Vec::new();
    //     let _ = thumbnail.write_to(&mut thumbnail_data, ImageFormat::Png);
    //
    //     // Convert image data to OpenCV matrix
    //     let image_data = imgcodecs::imdecode(
    //         &thumbnail_data,
    //         imgcodecs::IMREAD_COLOR,
    //     )?;
    //
    //     // Detect faces in the image
    //     let mut classifier = CascadeClassifier::new(&filename)?;
    //     classifier.load("haarcascade_frontalface_default.xml")?;
    //
    //     let mut faces = Mat::default();
    //     let _ = classifier.detect_multi_scale(&image_data, &mut faces.into(), 1.1, 3, 0, Size::new(30, 30), Size::default());
    //
    //     let num_faces = faces.size().height;
    //
    //     // Crop image around the detected face(s) or the center of the image if no face is detected
    //     let (x, y, w, h) = if num_faces > 0 {
    //         // Crop around the detected face(s)
    //         let face_rect: Rect = faces.get(0, 0).unwrap();
    //         let x = face_rect.x as u32;
    //         let y = face_rect.y as u32;
    //         let w = face_rect.width as u32;
    //         let h = face_rect.height as u32;
    //         (x, y, w, h)
    //     } else {
    //         // Crop around the center of the image
    //         let x = (orig_width - width) / 2;
    //         let y = (orig_height - height) / 2;
    //         (x, y, width, height)
    //     };
    //
    //     let mut resized_image = image.resize_to_fill(width, height, FilterType::Nearest);
    //     let (resized_width, resized_height) = resized_image.dimensions();
    //     let (dx, dy) = (
    //         (resized_width as i32 - w as i32) / 2,
    //         (resized_height as i32 - h as i32) / 2,
    //     );
    //
    //     let mut cropped_image = resized_image.crop(x as u32 + dx as u32, y as u32 + dy as u32, w, h);
    //     if num_faces > 0 {
    //         // Add transparent padding to the cropped image to show a bit of the whole picture
    //         let padding = 50; // number of pixels of padding to add
    //         let mut padded_image = DynamicImage::new_rgba8(width + 2 * padding, height + 2 * padding);
    //         let (padded_image_width, padded_image_height) = padded_image.dimensions();
    //         let (dx, dy) = (
    //             (padded_image_width - w) / 2,
    //             (padded_image_height - h) / 2,
    //         );
    //
    //         let _ = padded_image.copy_from(&cropped_image, dx, dy);
    //         cropped_image = padded_image;
    //     }
    //
    //     if retain_size {
    //         // Check if the original image is at least 70% smaller than the thumbnail dimensions
    //         if orig_width as f64 * 0.7 <= width as f64 && orig_height as f64 * 0.7 <= height as f64 {
    //             // Add transparent padding to the cropped image to retain its original size
    //             let mut padded_image = DynamicImage::new_rgba8(width, height);
    //             let (padded_image_width, padded_image_height) = padded_image.dimensions();
    //             let (dx, dy) = (
    //                 (padded_image_width - cropped_image.width()) / 2,
    //                 (padded_image_height - cropped_image.height()) / 2,
    //             );
    //
    //             let _ = padded_image.copy_from(&cropped_image, dx, dy);
    //             cropped_image = padded_image;
    //         }
    //     }
    //
    //     let mut cropped_data = Vec::new();
    //     let _ = cropped_image.write_to(&mut cropped_data, ImageFormat::Png);
    //
    //     // Set metadata
    //     let mut metadata = HashMap::new();
    //     metadata.insert(String::from("filename"), filename.clone());
    //
    //     // Upload original image to s3
    //     let request = PutObjectRequest {
    //         metadata: Some(metadata),
    //         bucket: self.bucket.to_owned(),
    //         key: format!("{}/{}", self.path, filename),
    //         body: Some(cropped_data.into()),
    //         acl: Some("public-read".to_owned()),
    //         content_type: Some(String::from("image/png")),
    //         ..Default::default()
    //     };
    //
    //     // Upload file
    //     let result = client.put_object(request).await;
    //     if result.is_err() {
    //         return Err(Errors::new("Unable to upload your file"));
    //     }
    //
    //     Ok(())
    // }
}
