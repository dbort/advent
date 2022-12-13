#!/usr/bin/env python3

class DirEnt:
  def __init__(self, size=-1, isdir=False) -> "DirEnt":
    self.isdir = isdir
    self.size = size
    if isdir:
      self.children = {}
    else:
      self.children = None

  def __repr__(self):
    return "DirEnt(" + (f"children={self.children}" if self.isdir else f"size={self.size}") + ")"

def build(input_path: str) -> DirEnt:
  root = DirEnt(isdir=True)
  path = [root]
  numlines = 0
  for line in open(input_path, "r"):
    line = line.strip()
    numlines += 1
    if numlines == 100:
      print(f"{path}, {len(path)}")
    parts = line.split(" ")
    if parts[0] == "$":
      if parts[1] == "cd":
        if parts[2] == "..":
          path.pop()
        elif parts[2] != "/":
          cwd = path[-1]
          path.append(cwd.children[parts[2]])
      else:
        assert parts[1] == "ls", f"{parts}"
        assert len(parts) == 2, f"{parts}"
    else:
      assert len(parts) == 2, f"{parts}"
      cwd = path[-1]
      if parts[0] == "dir":
        isdir = True
        size = -1
      else:
        isdir = False
        size = int(parts[0])
      cwd.children[parts[1]] = DirEnt(size=size, isdir=isdir)
  return root

def find_sizes(ent: DirEnt, sizes: list, depth: int = 0) -> int:
  if ent.isdir:
    sum = 0
    for name, child in ent.children.items():
      size = find_sizes(child, sizes, depth + 1)
      print(("  " * depth) + f"{name}: {size}")
      sum += size
    print(("  " * depth) + f"= {sum}")
    sizes.append(sum)
    return sum
  else:
    return ent.size

def main(input_path: str):
  root = build(input_path)
  sizes = []
  find_sizes(root, sizes)
  sum = 0
  for size in sizes:
    if size <= 100000:
      print(f"+ {size}")
      sum += size
  print(f"sum <= 10000: {sum}")

if __name__ == "__main__":
  # main(input_path="input.txt")
  main(input_path="input.txt")
