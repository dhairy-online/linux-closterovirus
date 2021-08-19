function memory_leak() {
  // let leaky = new Array(100);
  // let x = 10;
  // return function() { return x }
  fetch("http://localhost:8000");
}

while(true) memory_leak();
