function memory_leak() {
  fetch("http://localhost:8000");
}

while (true) memory_leak();
