var colors = ["amber", "light-amber", "green-1", "apple-green", "green-2", "apple-2c", "green-3"];

colors.forEach(addListener);

function addListener(color, index) {
  var colorClass = document.getElementsByClassName(color)[0];
  colorClass.addEventListener('click', changeColor(color), false);
}

function changeColor(color) {
  return function() {
    var root = document.documentElement;
    root.style.setProperty('--text-color', 'var(--' + color + ')');
  }
}
