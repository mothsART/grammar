function format(idInput, idOutput) {
  "use strict";
  var buffer = msgpack.encode(document.getElementById(idInput).value);
  axios.post(
    '/format', buffer,
    {
      responseType: 'blob',
      headers: {'Content-Type': 'application/msgpack; charset=utf-8'}
    }
  ).then(function(response) {
    var reader = new FileReader();
    reader.onload = function (e) {
        var value = msgpack.decode(new Uint8Array(reader.result));
        document.getElementById(idOutput).value = value;
    }
    reader.readAsArrayBuffer(response.data);
  });
}
