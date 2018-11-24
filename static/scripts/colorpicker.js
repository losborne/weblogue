var colors = ["amber", "light-amber", "green-1", "apple-green", "green-2", "apple-2c", "green-3"];

var color = getCookie("color")
if (color) {
  changeColor(color);
} else {
  updateColor(colors[0]);
}

function getCookie(cookieName) {
  var name = cookieName + "=";
  var decodedCookie = decodeURIComponent(document.cookie);
  var cookiePairs = decodedCookie.split(';');
  for(var i = 0; i < cookiePairs.length; i++) {
    var cookiePair = cookiePairs[i];
    cookiePair = removeLeadingWhitespace(cookiePair);
    if (cookiePair.indexOf(name) == 0) {
      return cookiePair.substring(name.length, cookiePair.length);
    }
  }
  return "";
}

function updateColor(color) {
  return function() {
    document.cookie = "color=" + color + ";path=/";
    changeColor(color)
  }
}

function changeColor(color) {
  var root = document.documentElement;
  root.style.setProperty('--text-color', 'var(--' + color + ')');
}

function removeLeadingWhitespace(string) {
  while (string.charAt(0) == ' ') {
    string = string.substring(1);
  }
  return string;
}

colors.forEach(addListener);

function addListener(color, index) {
  var colorClass = document.getElementsByClassName(color)[0];
  colorClass.addEventListener('click', updateColor(color), false);
}
