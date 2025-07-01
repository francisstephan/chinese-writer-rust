function convertToZi() {
  document.body.removeEventListener("keydown", esckey);
  var pyentree = document.getElementById("pinyin").value;
  var lon = pyentree.length;
  if (lon == 0) return;
  var dernier = pyentree.charAt(lon - 1);
  if ("01234/ ".includes(dernier)) {
    if (dernier == "/" || dernier == " ") {
      pyentree = pyentree.substring(0, lon - 1);
      document.getElementById("pinyin").value = pyentree;
    }
    // https://www.webdevtutor.net/blog/javascript-button-click-programmatically
    const button = document.getElementById("subpy");
    button.click();
  }
}

function add(zi) {
  var s = document.getElementById("zistring");
  s.value = s.value + zi;
  entree = document.getElementById("zilist"); // reset list of displayed zi buttons
  if (entree != null) entree.innerHTML = "";
  document.getElementById("pinyin").value = "";
  document.getElementById("pinyin").focus();
  document.body.removeEventListener("keydown", esckey);
}

function copyTextToClipboard() {
  // source : https://stackoverflow.com/questions/400212/how-do-i-copy-to-the-clipboard-in-javascript
  var text = document.getElementById("zistring").value;
  navigator.clipboard.writeText(text).then(
    function () {
      console.log("Async: Copied " + text + "to clipboard!");
    },
    function (err) {
      console.error("Async: Could not copy text: ", err);
    },
  );
  document.getElementById("pinyin").focus();
}

function lookup(text) {
  var url =
    "https://translate.google.com/?sl=auto&tl=en&text=" +
    text +
    "&op=translate";
  window.open(url);
  var elem = document.getElementById("pinyin");
  if (elem) elem.focus();
}

function reset() {
  var entree = document.getElementById("zistring");
  if (entree != null) entree.value = "";
  entree = document.getElementById("zilist");
  if (entree != null) entree.innerHTML = "";
  document.getElementById("pinyin").value = "";
  document.getElementById("pinyin").focus();
}
