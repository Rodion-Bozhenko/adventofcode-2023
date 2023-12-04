#include <cctype>
#include <cstdint>
#include <cstdio>
#include <fstream>
#include <stdexcept>
#include <string>
#include <unordered_map>

using namespace std;

int part_two();

int main() {
  fstream input_file;
  string filepath = "../input.txt";
  input_file.open(filepath, ios::in);
  if (!input_file.is_open()) {
    throw runtime_error("Failed to open file: " + filepath);
  }

  string line;
  int sum;
  while (getline(input_file, line)) {
    uint8_t left;
    uint8_t right;
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
  input_file.close();

  printf("PART 1 SUM: %d\n", sum);
  printf("PART 2 SUM: %d\n", part_two());

  return 0;
}

unordered_map<string, int> lookup_table = {
    {"one", 1}, {"two", 2},   {"three", 3}, {"four", 4}, {"five", 5},
    {"six", 6}, {"seven", 7}, {"eight", 8}, {"nine", 9}};

int lookup_word(string line) {
  for (auto const &[k, _] : lookup_table) {
    if (line.find(k) != string::npos) {
      return lookup_table[k];
    }
  }
  return -1;
}

int part_two() {
  fstream input_file;
  string filepath = "../input.txt";
  input_file.open(filepath, ios::in);
  if (!input_file.is_open()) {
    throw runtime_error("Failed to open file: " + filepath);
  }

  string line;
  int sum;
  while (getline(input_file, line)) {
    int left = -1;
    int right = -1;
    for (int i = 0; i < line.length(); i++) {
      if (left == -1) {
        int d = lookup_word(line.substr(0, i + 1));
        if (d != -1) {
          left = d * 10;
          break;
        }
      }
      if (left == -1 && isdigit(line[i])) {
        left = (line[i] - '0') * 10;
        break;
      }
    }
    for (int i = line.length(); i > -1; i--) {
      if (right == -1) {
        int d = lookup_word(line.substr(i, line.length()));
        if (d != -1) {
          right = d;
          break;
        }
      }
      if (right == -1 && isdigit(line[i])) {
        right = line[i] - '0';
        break;
      }
    }

    sum += left + right;
  }

  return sum;
}
