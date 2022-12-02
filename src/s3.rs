use bytes::Bytes;
use debugless_unwrap::*;
use image::{ImageBuffer, ImageFormat, imageops::FilterType};
use imagesize::blob_size;
use infer::Infer;
use rusoto_core::credential::{StaticProvider};
use rusoto_core::{HttpClient, Region};
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3 as RS3, S3Client};
use std::fs::File as StdFile;
use std::io::{Cursor, Read};
use std::str::FromStr;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

use crate::Errors;
use crate::File;
use crate::strings;
use crate::traits::Configurable;

/// Set mime type for jpeg
pub static BACKEND_MIME_JPEG: &'static str = "image/jpeg";

/// S3 struct contains s3 specific configurations
#[derive(Clone, Debug)]
pub struct S3 {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub path: String,
    pub region: String,
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

/// Configurable implementation for S3
impl Configurable<S3> for S3 {
    /// Implement new instance
    ///
    /// Example
    /// ```
    /// use library::S3;
    /// use library::traits::Configurable;
    ///
    /// fn main() {
    ///     // Create new instance with default values
    ///     let s3 = S3::new();
    /// }
    /// ```
    fn new() -> Self {
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
    /// use library::traits::Configurable;
    ///
    /// fn main() {
    ///     // Create new s3 instance with default values
    ///     let mut s3 = S3::new();
    ///     s3.clear();
    /// }
    /// ```
    fn clear(&mut self) -> Self {
        Self::default()
    }

    /// Reconfigure instance
    ///
    /// Example
    /// ```
    /// use library::S3;
    /// use library::traits::Configurable;
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
    fn reconfigure(&mut self, item: &S3) {
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

    /// Check if current instance has no value
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    /// use library::traits::Configurable;
    ///
    /// fn main() {
    ///     // Create new paseto instance with default values
    ///     let paseto = Paseto::new();
    ///     let is_valid = paseto.is_none();
    /// }
    /// ```
    fn is_none(&self) -> bool {
        let items = [
            self.clone().access_key_id,
            self.clone().bucket,
            self.clone().path,
            self.clone().region,
            self.clone().module_profile_picture,
        ];

        for item in items {
            if !item.is_empty() {
                return false
            }
        }

        let items = [
            self.clone().image_small_size,
            self.clone().image_medium_size,
            self.clone().image_large_size,
            self.clone().image_xls_size
        ];

        for item in items {
            if item > 0 {
                return false
            }
        }

        true
    }
}

/// S3 implementation
impl S3 {
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
    pub fn get_client(&self) -> Result<S3Client, Errors> {
        // Client is unavailable
        if self.is_none() {
            return Err(Errors::new("Unable to initialize aws s3"));
        }

        // Set access, secret access key & region
        let access_key = self.access_key_id.clone();
        let secret_access_key = self.secret_access_key.clone();
        let region = Region::from_str(&self.region);
        if region.is_err() {
            return Err(Errors::new(region.unwrap_err().to_string()))
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
        Ok(client)
    }

    /// Retrieve file from s3 via file_name
    ///
    /// Example
    /// ```
    /// use library::s3::S3;
    ///
    /// fn main() {
    ///     // Set new s3 instance
    ///     let s3 = S3::default();
    ///     let result = s3.get_file("image.jpg");
    /// }
    /// ```
    pub async fn get_file<T: Into<String>>(&self, file_name: T) -> Result<HttpResponse, Errors> {
        // Use tokio AsyncReadExt
        use tokio::io::AsyncReadExt;

        // Retrieve client
        let client = self.get_client();
        if client.is_err() {
            return Err(Errors::new(client.debugless_unwrap_err().to_string()));
        }

        // Shadow client
        let client = client.unwrap();

        // Set GetObjectRequest
        let req = GetObjectRequest{
            bucket: self.bucket.to_string(),
            key: format!("{}/{}", self.path.as_str(), file_name.into()),
            ..Default::default()
        };

        // Set result
        let result = client
            .get_object(req)
            .await;

        // Check if file was retrieved
        if result.is_err() {
            return Err(Errors::new("File not found"));
        }

        // Shadow result
        let result = result.unwrap();

        // Stream vector
        let mut stream = result.body.unwrap().into_async_read();
        let mut body = Vec::new();

        // Stream to body
        let result = stream.read_to_end(&mut body).await.unwrap();
        if result < 1 {
            return Err(Errors::new("File not found"));
        }

        // Set info
        let info = Infer::new();
        let mime = info.get(&body.clone());

        if mime.is_some() && !mime.clone().unwrap().mime_type().is_empty() {
            return Ok(HttpResponse::build(StatusCode::OK)
                .content_type(mime.clone().unwrap().mime_type())
                .body(body))
        }

        // Return response
        Ok(HttpResponse::build(StatusCode::OK).body(body))
    }

    /// Uploads image from social media via url.
    /// Thumbnail sizes: xls, large, medium & small
    /// xls: default is 1024x1024 px
    /// large: default is 512x512 px
    /// medium: default is 192x192 px
    /// small: default is 72x72 px
    pub async fn upload_image_from_socials<U, A, F, T>(&self, url:U, actor_id:A, file_id:F, thumbnail_size: T) -> Result<File, Errors>
        where U: Into<String>,
              A: Into<String>,
              F: Into<String>,
              T: Into<String>
    {
        // Create bindings
        let url_bindings = url.into();
        let actor_id_bindings = actor_id.into();
        let file_id_bindings = file_id.into();
        let thumbnail_size_bindings = thumbnail_size.into();

        // Client is unavailable
        if self.is_none() {
            return Err(Errors::new("Unable to initialize aws s3"));
        }

        // Retrieve file by byte
        let result = reqwest::get(&url_bindings).await;
        if result.is_err() {
            return Err(Errors::new("Unable to download file from url"));
        }

        // Check bytes
        let result = result.unwrap().bytes().await;
        if result.is_err() {
            return Err(Errors::new("Unable to download file from url"));
        }

        // Set actor and file id's
        let aid = actor_id_bindings.clone();
        let fid = file_id_bindings.clone();

        //Set file bytes
        let file_bytes = result.unwrap();
        let mut file = File::new();

        // Set xls size width
        let width = self.image_xls_size.clone() as u32;
        let height = self.image_xls_size.clone() as u32;

        // Set resize
        if width > 0 && height > 0 {
            let bytes = file_bytes.clone();
            let mut is_thumbnail = false;
            if thumbnail_size_bindings.clone().to_lowercase().as_str() == "xls" {
                is_thumbnail = true
            }

            let result = self.resize_upload_from_memory(file, bytes, width, height, &aid, &fid, is_thumbnail).await;
            if result.is_err() {
                return Err(Errors::new("Unable to download file from url"));
            }

            file = result.unwrap();
        }

        // Set large size width
        let width = self.image_large_size.clone() as u32;
        let height = self.image_large_size.clone() as u32;

        // Set resize
        if width > 0 && height > 0 {
            let bytes = file_bytes.clone();
            let mut is_thumbnail = false;
            if thumbnail_size_bindings.clone().to_lowercase().as_str() == "large" {
                is_thumbnail = true
            }

            let result = self.resize_upload_from_memory(file, bytes, width, height, &aid, &fid, is_thumbnail).await;
            if result.is_err() {
                return Err(Errors::new("Unable to download file from url"));
            }

            file = result.unwrap();
        }

        // Set medium size width
        let width = self.image_medium_size.clone() as u32;
        let height = self.image_medium_size.clone() as u32;

        // Set resize
        if width > 0 && height > 0 {
            let bytes = file_bytes.clone();
            let mut is_thumbnail = false;
            if thumbnail_size_bindings.clone().to_lowercase().as_str() == "medium" {
                is_thumbnail = true
            }

            let result = self.resize_upload_from_memory(file, bytes, width, height, &aid, &fid, is_thumbnail).await;
            if result.is_err() {
                return Err(Errors::new("Unable to download file from url"));
            }

            file = result.unwrap();
        }

        // Set small size width
        let width = self.image_small_size.clone() as u32;
        let height = self.image_small_size.clone() as u32;

        // Set resize
        if width > 0 && height > 0 {
            let bytes = file_bytes.clone();
            let mut is_thumbnail = false;
            if thumbnail_size_bindings.clone().to_lowercase().as_str() == "small" {
                is_thumbnail = true
            }

            let result = self.resize_upload_from_memory(file, bytes, width, height, &aid, &fid, is_thumbnail).await;
            if result.is_err() {
                return Err(Errors::new("Unable to download file from url"));
            }

            file = result.unwrap();
        }

        // Set module
        file.module = Some(self.module_profile_picture.clone());

        // Split string
        let split = url_bindings.split("/").collect::<Vec<&str>>();
        let last = split.last();
        if last.is_none() {
            return Err(Errors::new("Unable to upload file"));
        }

        // Shadow last value
        let mut split = last.unwrap().split(".");
        let label = split.next();
        if label.is_none() {
            return Err(Errors::new("Unable to upload file"));
        }

        // Set file lable
        file.label = Some( format!("{}.jpg", label.unwrap()));

        // Return ok
        Ok(file)
    }

    /// Resize uploading from memory
    pub async fn resize_upload_from_memory<T>(
        &self,
        mut file: File,
        bytes: Bytes,
        width: u32,
        height: u32,
        actor_id: T,
        file_id: T,
        is_thumbnail: bool
    ) -> Result<File, Errors>
        where T: Into<String> + Copy
    {
        // Load an image with normal formatting
        let dynamic_image = image::load_from_memory_with_format(&bytes.clone(), ImageFormat::Jpeg);
        if  dynamic_image.is_err() {
            return Err(Errors::new("Unable to download file from url"));
        }

        // Get client
        let result = self.get_client();
        if result.is_err() {
            return Err(Errors::new(result.debugless_unwrap_err().to_string()));
        }

        // Set client
        let client = result.unwrap();

        // Check if thumbnail
        return match is_thumbnail {
            true => {
                // Set thumbnail name
                file.thumbnail = Some(format!("{}-{}-thumbnail-{}px-{}px.jpg", actor_id.into(), file_id.into(), width.clone(), height.clone()));
                let file_name = file.thumbnail.as_ref().unwrap().clone();

                // Create scaled output
                let scaled = dynamic_image.unwrap().resize(width, height, FilterType::Lanczos3);

                // Create buffer
                let mut cursor = Cursor::new(vec![]);
                let output = scaled.write_to(&mut cursor, ImageFormat::Jpeg);
                if output.is_err() {
                    return Err(Errors::new("Unable to read file"));
                }

                // Save buffer & convert to ByteStream
                let buffer = cursor.get_ref();

                // Set info
                let info = Infer::new();
                let mut mime:Option<String> = None;

                // Check out mime type
                let mime_type = info.get(&buffer.clone());
                if mime_type.is_some() && !mime_type.clone().unwrap().mime_type().is_empty() {
                    mime = Some(String::from(mime_type.clone().unwrap().mime_type()));
                }

                // Set mime type
                file.mime_type = mime.clone();

                // Upload file
                let req = PutObjectRequest {
                    bucket: self.bucket.to_owned(),
                    key: format!("{}/{}", self.path.as_str(), file_name.as_str()),
                    body: Some(buffer.clone().into()),
                    acl: Some("public-read".to_owned()),
                    content_type: mime,
                    ..Default::default()
                };

                let result = client.put_object(req).await;
                if result.is_err() {
                    return Err(Errors::new("Unable to upload file"));
                }

                // Set width and heigth
                file.width = Some(width.clone() as i32);
                file.height = Some(height.clone() as i32);

                // Return ok
                Ok(file)
            },
            false => {
                // Set file name
                file.file_name = Some(format!("{}-{}-{}px-{}px.jpg", actor_id.into(), file_id.into(), width.clone(), height.clone()));
                let file_name = file.file_name.as_ref().unwrap().clone();

                // Unwrap dynamic image
                let mut scaled = dynamic_image.unwrap();

                // Create buffer
                let mut cursor = Cursor::new(vec![]);
                let output = scaled.write_to(&mut cursor, ImageFormat::Jpeg);
                if output.is_err() {
                    return Err(Errors::new("Unable to read file"));
                }

                // Save buffer & convert to ByteStream
                let buffer = cursor.get_ref();
                let data = &buffer.clone()[..];

                // Read file size from buffer
                let blob = blob_size(data);
                if blob.is_err() {
                    return Err(Errors::new("Unable to read file"));
                }

                // Get original sizes
                let size = blob.unwrap();

                // Create scaled output
                let mut img = ImageBuffer::from_fn(width, height, |_x, _y| image::Rgba([0, 0, 0, 0]));
                image::imageops::overlay(
                    &mut img,
                    &mut scaled,
                    (width as i64 - size.width as i64) / 2,
                    (height as i64 - size.height as i64) / 2
                );

                // Create buffer
                let mut cursor = Cursor::new(vec![]);
                let output = img.write_to(&mut cursor, ImageFormat::Jpeg);
                if output.is_err() {
                    return Err(Errors::new("Unable to read file"));
                }

                // Save buffer & convert to ByteStream
                let buffer = cursor.get_ref();

                // Set info
                let info = Infer::new();
                let mut mime:Option<String> = None;

                // Check out mime type
                let mime_type = info.get(&buffer.clone());
                if mime_type.is_some() && !mime_type.clone().unwrap().mime_type().is_empty() {
                    mime = Some(String::from(mime_type.clone().unwrap().mime_type()));
                }

                // Set mime type
                file.mime_type = mime.clone();

                // Upload file
                let req = PutObjectRequest {
                    bucket: self.bucket.to_owned(),
                    key: format!("{}/{}", self.path.as_str(), file_name.as_str()),
                    body: Some(buffer.clone().into()),
                    acl: Some("public-read".to_owned()),
                    content_type: mime.clone(),
                    ..Default::default()
                };

                let result = client.put_object(req).await;
                if result.is_err() {
                    return Err(Errors::new("Unable to upload file"));
                }

                // Set file size
                let length = buffer.clone().len();
                file.file_size = Some(strings::get_file_size(length as f64));
                file.width = Some(width.clone() as i32);
                file.height = Some(height.clone() as i32);

                // Return ok
                Ok(file)
            }
        }
    }

    /// Try a test upload to s3
    /// Example
    /// ```
    /// use library::s3::S3;
    ///
    /// fn main() {
    ///     // Set new s3 instance
    ///     let s3 = S3::default();
    ///     let result = S3::test_upload("access_key_id", "secret_access_key", "region", "bucket_name", "folder_name");
    /// }
    /// ```
    pub async fn test_upload<T>(
        access_key_id: T,
        secret_access_key: T,
        region: T,
        bucket: T,
        folder: T
    ) -> Result<(), Errors>
        where T: Into<String>
    {
        // Set access, secret access key & region
        let access_key = access_key_id.into();
        let secret_access_key = secret_access_key.into();
        let region = Region::from_str(&region.into());
        if region.is_err() {
            return Err(Errors::new(region.unwrap_err().to_string().as_str()))
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
        let mut mime:Option<String> = None;

        // Check out mime type
        let mime_type = info.get(&contents.clone());
        if mime_type.is_some() && !mime_type.clone().unwrap().mime_type().is_empty() {
            mime = Some(String::from(mime_type.clone().unwrap().mime_type()));
        }

        // Upload file
        let req = PutObjectRequest {
            bucket: bucket.into().to_owned(),
            key: format!("{}/doc.txt", folder.into()),
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
}