#[cfg(test)]
mod tests{
    use crate::test_file_helper::{download_thumb_b64_by, upload_by_url};
    use std::fs;

    #[test]
    fn download_thumb_b64_test() {
        let image_id:String = "62cd5275-69d8-4e96-9ac9-a68e826ced36".to_owned();
        match download_thumb_b64_by(image_id){
            Ok(res) => assert!(false),
            Err(err) => {}
        };
    }

    #[test]
    fn upload_by_url_test() {
        fs::create_dir("images");
        let url = "https://sites.google.com/site/prirodanasevseegooglgfgf/_/rsrc/1463456237313/home/priroda_gory_nebo_ozero_oblaka_81150_1920x1080.jpg";
        match upload_by_url(url.to_owned()) {
            Ok(id)=>{
                match download_thumb_b64_by(id) {
                    Ok(f)=>{},
                    Err(err)=>assert!(false)
                }
            },
            Err(err)=>assert!(false)
        }
        fs::remove_dir_all("images");
    }
}