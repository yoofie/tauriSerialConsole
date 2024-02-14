
function call_cfg() {
	var finalJson = {};
	finalJson["port_name"] = document.getElementById("com_channel").value;
	//finalJson["baud_rate"] = parseInt(document.getElementById("baud_rate").value);
  
	if (document.getElementById("baud_rate").value == "CUSTOM") {
	  console.log("SELECTED CUSTOM BAUD RATE OK");
	  finalJson["baud_rate"] = parseInt(
		document.getElementById("baud_rate_custom").value
	  );
	} else {
	  finalJson["baud_rate"] = parseInt(
		document.getElementById("baud_rate").value
	  );
	}
	
	finalJson["parity"] = parseInt(document.getElementById("parity").value);
	finalJson["stop_bits"] = parseInt(document.getElementById("stopBits").value);
  
	if (document.getElementById("decoder").value == "CUSTOM") {
	  console.log("SELECTED CUSTOM");
	  finalJson["decoder"] = document.getElementById("customDecoder").value;
	} else {
	  finalJson["decoder"] = document.getElementById("decoder").value;
	}
  
	console.log(finalJson);
	var json_cfg_string = JSON.stringify(finalJson, null, 4);
	console.log(json_cfg_string);
  
	//invokee('send_cfg', { cmd: json_cfg_string })
	invoke("send_cfg", { blah: json_cfg_string }).then(message => console.log(message));
  }
  
  function ctrl_play() {

	invoke("ctrl_play").then(message => console.log(message));
  }
  
  function ctrl_pause() {

	invoke("ctrl_pause").then(message => console.log(message));
  }
