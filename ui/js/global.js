// access the pre-bundled global API functions
//const {invoke} = window.__TAURI__.tauri;
const invoke = window.__TAURI__.invoke;
const ss = window.__TAURI__.event;
const { emit, listen } = window.__TAURI__.event;

const COMMAND_PREFIX = "command:";

const patchedSend = async function(params) {
  // Make readonly properties writable
  Object.defineProperty(this, "readyState", { writable: true });
  Object.defineProperty(this, "status", { writable: true });
  Object.defineProperty(this, "statusText", { writable: true });
  Object.defineProperty(this, "response", { writable: true });

  // Set response
  const query = new URLSearchParams(params);
  this.response = await invoke(this.command, Object.fromEntries(query));
  this.readyState = XMLHttpRequest.DONE;
  this.status = 200;
  this.statusText = "OK";

  // We only need load event to trigger a XHR response
  this.dispatchEvent(new ProgressEvent("load"));
};

window.addEventListener("DOMContentLoaded", () => {
  document.body.addEventListener("htmx:beforeSend", event => {
    const path = event.detail.requestConfig.path;
    if (path.startsWith(COMMAND_PREFIX)) {
      event.detail.xhr.command = path.slice(COMMAND_PREFIX.length);
      event.detail.xhr.send = patchedSend;
    }
  });
});

// https://speedsheet.io/s/tauri#6PEg
// https://github.com/tauri-apps/create-tauri-app/issues/48
// https://stackblitz.com/edit/js-virtualized-list-vsd2zk?file=style.css,index.js

// https://github.com/tbranyen/hyperlist

/* const unlisten = listen('clicky', (event) => {
	// event.event is the event name (useful if you want to use a single callback fn for multiple event types)
	// event.payload is the payload object
	console.log(event.event);
	console.log(event.payload);

}); */

// Prevent right click menu
//document.addEventListener('contextmenu', event => event.preventDefault());

var logData = new Array();
var serialData = new Array();
var container = document.createElement("div");
var config = {
  height: window.innerHeight,
  itemHeight: 20,
  total: 1000,
  // Set to true to put into 'chat mode'.
  reverse: false,
  scrollerTagName: "div",

  generate(row) {
    var newHeight = 50;

    let item = logData[row];
    var el = Object.assign(document.createElement("div"), {
      innerHTML: `ITEM ${item}`
    });

    return { element: el, height: newHeight };
  }
};

var list = HyperList.create(container, config);

window.onresize = e => {
  config.height = window.innerHeight;
  list.refresh(container, config);
};

container.classList.add("containerr");

document.body.appendChild(container);

let userx = "Volvo";



function changeColor() {
  document.getElementById("box").style.backgroundColor = "#00FF00";
  document.getElementById("box").style.color = "#000";
  document.getElementById("box").style.padding = "5px";
}

// listen a event emitted from the backend
// https://github.com/tauri-apps/tauri/issues/3276
emit("a1", "This is a message");

// emit multiple events
setInterval(() => {
  emit("a2", "That message is a simple message!!!");
}, 1_000);

// listen a event emitted from the backend
listen("b1", ev => {
  console.log(
    `I got a backend event!!!\n\n\tThis is the content of the event:\n${ev.payload}`
  );
  console.log(`Other properties:\n\n${ev.id}\n${ev.event}`);
});



listen("serialEvent", ev => {
	//console.log(`SERIAL EVENT | ${ev.payload}`);
	serialData.push(ev.payload);
  });

var prev_length = 0;

function addSerialData() {
	var current_size = serialData.length;

	if (current_size != prev_length) {
		let diff = current_size - prev_length;
		let index = prev_length;
		prev_length = current_size;
		for (let i = 0; i < diff; i++) { 

			// Get the table element in which you want to add row
			let table = document.getElementById("logTable");

			// Create a row using the inserRow() method and
			// specify the index where you want to add the row
			let row = table.insertRow(-1); // We are adding at the end
			
			// Create table cells
			let c1 = row.insertCell(0);
			let c2 = row.insertCell(1);
			let c3 = row.insertCell(2);
			
			// Add data to c1 and c2
			c1.innerText = row.rowIndex;
			c2.innerText = serialData[index + i];
			c3.innerText = serialData[index + i];
			
			//console.log(`CONSOLE LENGTH = ${logData.length}`);
		}
	}
  
}

function clearBuffer(){
	var Table = document.getElementById("logTable");
Table.innerHTML = `<thead>
<th style="width:10%">Index</th>
<th style="width:10%">Time</th>
<th>Message</th>
</thead>`;
serialData.length = 0;
}


function appendLog(target, logItem) {
  console.log("YESS");
  var logx2 = document.getElementById("logx2");
  var rxData = JSON.parse(logItem);
  //console.log(`RXDATA2 = ${rxData.name} / ${rxData.value}`);
  logData.push(rxData);

  console.log(
    `LOG DATA = ${logData[logData.length].name} / ${logData[logData.length]
      .value}`
  );
  var h1 = document.createElement("li");
  h1.innerHTML = `${rxData.name} | ${rxData.value} | <code>${logItem}</code>`;
  document.getElementById("logx").appendChild(h1);

  logx2.innerHTML = rxData.value;
}

var counter = document.querySelector("#counter");
var number = 0;


var countUp = function() {

  // Update the UI
  counter.textContent = serialData.length;

  // if the number is less than 500, run it again
	addSerialData();

	config.height = window.innerHeight;
	config.total = logData.length;
	window.requestAnimationFrame(countUp);
};

// Start the animation
window.requestAnimationFrame(countUp);
listen("wowza", ev => {
  console.log(ev.payload);
  appendLog("logx", ev.payload);
});
