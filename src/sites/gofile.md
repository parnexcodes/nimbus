Returns a list of servers available to receive content.

Do not call this function more than once every 10 seconds. If you have a lot of small files to upload, keep the same servers for several files before calling the function again.

Parameters
optional zone
Specifies a zone (eu for Europe, or na for North America). If no server is found for your zone, returns servers from other zones.

Curl Example
curl -X GET 'https://api.gofile.io/servers'
JavaScript Example
fetch('https://api.gofile.io/servers', {
method: 'GET'
})
.then(response => response.json())
.then(data => {
console.log(data);
})
.catch(error => console.error(error));
Response Example
{
"status": "ok",
"data": {
"servers": [
{"name":"store1","zone":"eu"},
{"name":"store3","zone":"na"}
]
}
}

Upload a file to a server.

You can obtain a list of available servers with https://api.gofile.io/servers

If no parameters are provided, a guest account is created, a new public folder is created, and the file is uploaded into it.

It is possible to use the ID of the created guest account as well as the ID of the created folder in new requests to upload more files into the same folder.

Parameters multipart/form-data
required file
The file to be uploaded.

optional token
Your account API token. If not provided, the file is uploaded to a guest account.

optional folderId
The identifier of the folder where the file will be stored. If not provided, a new public folder will be created.

Curl Example
curl -X POST 'https://store1.gofile.io/contents/uploadfile' -H "Authorization: Bearer your_token" -F "file=@file.txt" -F "folderId=5e042945-0e5c-4c1d-9293-4574d376e496"
JavaScript Example
const formData = new FormData();
formData.append("file", fileInput.files[0]);
formData.append("folderId", "5e042945-0e5c-4c1d-9293-4574d376e496");

fetch('https://store1.gofile.io/contents/uploadfile', {
method: 'POST',
headers: {
'Authorization': 'Bearer your_token_here'
},
body: formData
})
.then(response => response.json())
.then(data => {
console.log(data);
})
.catch(error => console.error(error));
Response Example
{
"data": {
"code": "0fVLYU",
"downloadPage": "https://gofile.io/d/0fVLYU",
"fileId": "f3ce5ceb-9ba8-4d65-a2cc-7ee234b73cb2",
"fileName": "file.txt",
"md5": "098f6bcd4621d373cade4e832627b4f6",
"parentFolder": "1885a9ac-1da7-460b-82e0-905ac9a47d33"
},
"status": "ok"
}
