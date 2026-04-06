int main() {
  // this is a comment
  int a = 5;
  int b = 2;
  int c = a + b;
  print(c);

  if (a == b) {
    print("uguali");
  } else if (a > b) {
    print("a maggiore");
  } else {
    print("b maggiore");
  }


  while (a < 12) {
    a = a + 1;
    print(a);
  }

  return 0;
}
