#Environment
- installed Rust v1.31-nightly;
- docker.
# Running
To build this project use this command:<p>
_**docker build -t myapp .**_ <p>
To run this project use this command: <p>
_**docker run --rm -it myapp**_

# API
## Upload images
### Upload using Multipart/Form-data
Request </p>
Method: **POST** <p>
_URL: **/load/image**; Key: "**photo**"_

Response #1 </p>
```json
{
  "status": "ok",
  "code": 200,
  "image_id": "62cd5275-69d8-4e96-9ac9-a68e826ced36"
}
```
Response #2
```json
{
  "status": "failed",
  "code": 400,
  "msg": "Image not found"
}
```
### Upload using Base64
Request </p>
Method: **POST** <p>
_URL: **/load/image64**_

```json
{
  "base64": ["<image1_b64>", "<image2_b64>"]
}
```

Response #1 </p>
```json
{
  "status": "ok",
  "code": 200,
  "image_ids": ["<image_id_1>", "<image_id_2>"]
}
```
Response #2
```json
{
  "status": "failed",
  "code": 400,
  "msg": "Undefined image format"
}
```

### Upload using urls
Request </p>
Method: **POST** <p>
_URL: **/load/image-url**_

```json
{
  "base64": ["<image1_url>", "<image2_url>"]
}
```

Response #1 </p>
```json
{
  "status": "ok",
  "code": 200,
  "image_ids": ["<image_id_1>", "<image_id_2>"]
}
```
Response #2
```json
{
  "status": "failed",
  "code": 400,
  "msg": "Undefined image format"
}
```
## Download thumbs
### Download thumb by image id
Request </p>
Method: **GET** <p>
_URL: **/download/thumb/<image_id>**_

Response #1 </p>
```json
{
  "status": "ok",
  "code": 200,
  "image_b64": "<image_b64>"
}
```
Response #2
```json
{
  "status": "failed",
  "code": 500,
  "msg": "File not found"
}
```
## Service
### Shutdown app
Request </p>
Method: **POST** <p>
_URL: **/shutdown**_



