#!/usr/bin/env python3

import sys

def main():
  index_path = sys.argv[1]
  with open(index_path) as f:
    for line in f:
      start_idx = line.find("href=\"")
      if start_idx == -1:
        continue
      start_idx += 6
      end_idx = line[start_idx:].find("\"")
      assert end_idx != -1
      end_idx += start_idx
      filename = line[start_idx:end_idx]
      url = "http://netlib.org/slatec/src/{}".format(filename)
      print(url)

if __name__ == "__main__":
  main()
