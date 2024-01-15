let newMessageForm = document.getElementById("chat-input");
let statusDiv = document.getElementById("status");
let messageField = newMessageForm.querySelector("#messageInput");
let usernameField = newMessageForm.querySelector("#username");

var STATE = {
  connected: false,
};

function subscribe(uri) {
  var retryTime = 1;

  function connect(uri) {
    const events = new EventSource(uri);

    events.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);
      if (!("message" in msg)) return;

      addMessage(msg.message);
    });

    events.addEventListener("open", () => {
      console.log(`connected to event stream at ${uri}`);
      retryTime = 1;
      setConnectedStatus(true);
    });

    events.addEventListener("error", () => {
      events.close();

      document.cookie = "user_state=;";

      let timeout = retryTime;
      retryTime = Math.min(64, retryTime * 2);
      console.log(`connection lost. attempting to reconnect in ${timeout}s`);
      setTimeout(() => connect(uri), (() => timeout * 1000)());
      setConnectedStatus(false);
    });
  }

  connect(uri);
}

function init() {
  console.log("fetching history");
  fetch("/history", {
    method: "GET",
  }).then((response) => {
    response.text().then((text) => {
      console.log(text);
    });
  });

  // Set up the form handler.
  newMessageForm.addEventListener("submit", (e) => {
    e.preventDefault();
    const message = messageField.value;
    const username = usernameField.value;

    if (message.length === 0) return;

    if (STATE.connected) {
      fetch("/message", {
        method: "POST",
        body: new URLSearchParams({ username, message }),
      }).then((response) => {
        if (response.ok) messageField.value = "";
      });
    }
  });

  subscribe("/events");
}

function addMessage(messageText) {
  var messageElement = document.createElement("div");
  messageElement.classList.add("message");

  var messageTime = document.createElement("span");
  messageTime.classList.add("message-time");
  messageTime.textContent = getCurrentTime();
  messageElement.appendChild(messageTime);

  var messageContent = document.createElement("span");
  messageContent.classList.add("message-content");
  messageContent.textContent = nickname + ": " + messageText;
  messageElement.appendChild(messageContent);

  var chatMessages = document.getElementById("chatMessages");
  chatMessages.appendChild(messageElement);
}

function getCurrentTime() {
  var now = new Date();
  var hours = now.getHours().toString().padStart(2, "0");
  var minutes = now.getMinutes().toString().padStart(2, "0");
  return hours + ":" + minutes;
}

function setConnectedStatus(status) {
  STATE.connected = status;
  statusDiv.className = status ? "connected" : "reconnecting";
}

init();
