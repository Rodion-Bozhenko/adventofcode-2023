#include <cctype>
#include <cstdint>
#include <cstdio>
#include <fstream>
#include <stdexcept>
#include <string>

using namespace std;
int main() {
  fstream input_file;
  string filepath = "../input.txt";
  input_file.open(filepath, ios::in);
  if (!input_file.is_open()) {
    throw runtime_error("Failed to open file: " + filepath);
  }

  string line;
  uint8_t left;
  uint8_t right;
  int sum;
  while (getline(input_file, line)) {
    for (int i = 0; i < line.length(); i++) {
      if (isdigit(line[i])) {
        left = (line[i] - '0') * 10;
        break;
      }
    }
    for (int i = line.length() - 1; i > -1; i--) {
      if (isdigit(line[i])) {
        right = line[i] - '0';
        break;
      }
    }
    sum += left + right;
  }

  printf("PART 1 SUM: %d\n", sum);

  return 0;
}
