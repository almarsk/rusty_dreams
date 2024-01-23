let newMessageForm = document.getElementById("chat-input");
let messageField = newMessageForm.querySelector("#messageInput");
let usernameField = newMessageForm.querySelector("#username");
let dropdown = document.getElementById("userDropdown");
let deleteUser = document.getElementById("userDeletion");
let mess_window = document.getElementById("chatMessages");

var STATE = {
  connected: false,
};

function subscribe(uri) {
  var retryTime = 1;

  function connect(uri) {
    const events = new EventSource(uri);

    events.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);

      if (
        Object.prototype.toString.call(msg) === "[object Object]" &&
        "Message" in msg
      ) {
        console.log(msg);
        if (!("message" in msg.Message)) return;
        addMessage(
          msg.Message.username,
          msg.Message.message,
          msg.Message.date,
          msg.Message.deleted,
        );
        // scroll down upon message
      } else {
        console.log("user deleted, lets go");
        getHistory();
        getMannschaft();
      }
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
  if (getCookieValue(document.cookie, "new_user")) {
    alert(`new user created: ${usernameField.value}`);
  }

  getHistory();
  getMannschaft();

  // Set up the form handler.
  newMessageForm.addEventListener("submit", (e) => {
    e.preventDefault();
    const message = messageField.value;
    const username = usernameField.value;

    if (message.length === 0) return;

    if (STATE.connected) {
      fetch("/message", {
        method: "POST",
        body: new URLSearchParams({
          username,
          message,
          date: getCurrentTime(),
        }),
      }).then((response) => {
        if (response.ok) messageField.value = "";
        //console.log(mess_window.scrollTop);
        //console.log(mess_window.scrollHeight);
        mess_window.scrollTop = mess_window.scrollHeight;
      });
    }
  });

  deleteUser.addEventListener("submit", (e) => {
    e.preventDefault(); // Prevent the default form submission behavior

    //const formData = new FormData(deleteUser);
    //const userValue = formData.get("user");

    fetch(`/delete?user=${dropdown.options[dropdown.selectedIndex].text}`, {
      method: "DELETE", // Use the DELETE method for deleting resources
    })
      .then((response) => {
        if (response.ok) {
          console.log("User deleted successfully");
        } else {
          console.error("Failed to delete user");
        }
      })
      .catch((error) => {
        console.error("Error during delete request:", error);
      });
  });
  subscribe("/events");
}

function addMessage(user, messageText, date, deleted) {
  var messageElement = document.createElement("div");
  messageElement.classList.add("message");

  var messageTime = document.createElement("span");
  messageTime.classList.add("message-time");
  console.log(user, messageText, date, "deleted: " + deleted);
  messageTime.textContent = date;
  messageElement.appendChild(messageTime);

  var messageContent = document.createElement("span");
  messageContent.classList.add("message-content");
  messageContent.textContent = user + ": " + messageText;
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
}

function getCookieValue(cookieString, cookieName) {
  console.log(cookieString);
  const cookies = cookieString.split("; ");

  for (const cookie of cookies) {
    const [name, value] = cookie.split("=");
    if (name === cookieName) {
      return value === "true"; // Parse the value as a boolean
    }
  }

  return false;
}

function logOff() {
  setConnectedStatus(false);

  // Clear cookies
  var cookies = document.cookie.split(";");

  for (var i = 0; i < cookies.length; i++) {
    var cookie = cookies[i];
    var eqPos = cookie.indexOf("=");
    var name = eqPos > -1 ? cookie.substr(0, eqPos) : cookie;
    document.cookie = name + "=;expires=Thu, 01 Jan 1970 00:00:00 GMT;path=/";
  }

  // Reload the page
  window.location.reload();
}

function getHistory() {
  var chatMessages = document.getElementById("chatMessages");
  chatMessages.innerHTML = "";
  fetch("/history", {
    method: "GET",
  }).then((response) => {
    response.json().then((data) => {
      data.forEach((d) => {
        //console.log(`we got ${d.message} from ${d.username}`);
        addMessage(d.username, d.message, d.date, d.deleted);
      });
    });
  });
}

function getMannschaft() {
  dropdown.innerHTML = "";
  fetch("/mannschaft", {
    method: "GET",
  }).then((response) => {
    response.json().then((data) => {
      data
        .filter((d) => d != "")
        .forEach((d, i) => {
          var option = document.createElement("option");
          option.value = "option" + (i + 1);
          option.text = d;
          dropdown.add(option);
        });
    });
  });
}

init();

setTimeout(() => {
  mess_window.scrollTop = mess_window.scrollHeight;
}, 30);
