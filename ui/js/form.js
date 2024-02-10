function jsonConcat(o1, o2) {
	for (var key in o2) {
	 o1[key] = o2[key];
	}
	return o1;
}

function get_object(elementname) {
	var ch = document.getElementById(elementname);
	let form = new FormData(ch);
	let object = {};
	form.forEach(function(value, key){
		object[key] = value;
	});

	return object;
}

function testResults (form) {

	var finalJson = {};

	jsonConcat(finalJson, get_object('channelForm'));
	jsonConcat(finalJson, get_object('baudForm'));
	jsonConcat(finalJson, get_object('parityForm'));
	jsonConcat(finalJson, get_object('stopBitsForm'));
	jsonConcat(finalJson, get_object('decoderForm'));
	finalJson["parityy"] = parseInt(document.getElementById("parity").value);
	console.log(finalJson);
	console.log(JSON.stringify(finalJson, null, 4));
}

function refresh_sources(){
	var jsonOptions = JSON.parse(request.responseText);
  
      // Loop over the JSON array.
      jsonOptions.forEach(function(item) {
        // Create a new <option> element.
        var option = document.createElement('option');
        // Set the value using the item in the JSON array.
        option.value = item;
        // Add the <option> element to the <datalist>.
        dataList.appendChild(option);
      });
}

function get_user_cfg_data(){
	var finalJson = {};
	finalJson["com_channel"] = document.getElementById("com_channel").value;
	//finalJson["baud_rate"] = parseInt(document.getElementById("baud_rate").value);

	if (document.getElementById("baud_rate").value == "CUSTOM") {
		console.log("SELECTED CUSTOM BAUD RATE OK");
		finalJson["baud_rate"] = parseInt(document.getElementById("baud_rate_custom").value);
	} else {
		finalJson["baud_rate"] = parseInt(document.getElementById("baud_rate").value);
	}



	finalJson["parity"] = parseInt(document.getElementById("parity").value);
	finalJson["stopBits"] = parseInt(document.getElementById("stopBits").value);

	if (document.getElementById("decoder").value == "CUSTOM") {
		console.log("SELECTED CUSTOM");
		finalJson["decoder"] = (document.getElementById("customDecoder").value);
	} else {
		finalJson["decoder"] = (document.getElementById("decoder").value);
	}

	
	console.log(finalJson);
	console.log(JSON.stringify(finalJson, null, 4));
}

function decoderChange(){
	const customDecoderField = (document.getElementById("customDecoderField"));

	if (document.getElementById("decoder").value == "CUSTOM") {
		console.log("SELECTED CUSTOM DECODER");
		customDecoderField.style.display = "block";
	} else {
		customDecoderField.style.display = "none";
	}
}

function baudrateChange(){
	const userSelection = (document.getElementById("baud_rate").value);
	const baudRateCustomDiv = (document.getElementById("baudRateCustom"));

	if (document.getElementById("baud_rate").value == "CUSTOM") {
		console.log("SELECTED CUSTOM BAUD RATE");
		baudRateCustomDiv.style.display = "block";
	} else {
		baudRateCustomDiv.style.display = "none";
	}
}

// When something in the form changes
const form = document.getElementById("channelForm");
form.addEventListener('change', function() {
    console.log("SOEMTHING CHANGED!!");
});
