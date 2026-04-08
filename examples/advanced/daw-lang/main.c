int max(int a, int b) {
  if (a > b) {
    return a;
  } else {
    return b;
  }
}

int main() {
  // this is a comment
  int a = 5;
  int b = 2;
  int c = a + b;
  print(c);

  if (a == b) {
    print("same value");
  } else if (a > b) {
    print("a is bigger");
  } else {
    print("b is bigger");
  }


  while (a < 12) {
    a = a + 1;
    print(a);
  }

  return 0;
}
