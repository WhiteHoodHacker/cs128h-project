let newMessageForm = document.getElementById('new-message');
let messageField = newMessageForm.querySelector("#message");

newMessageForm.addEventListener("submit", (e) => {
  e.preventDefault();
  const message = messageField.value;

  fetch("/send", {
    method: "POST",
    body: new URLSearchParams({message: message}),
  }).then((response) => {
    if (response.ok) messageField.value = "";
  });

});
