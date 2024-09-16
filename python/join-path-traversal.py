import os

# Path
path = "/home"

# Join various path components
print(os.path.join(path, "username/", "file.txt"))

# Path
path = "/var/logs"

# Join various path components
print(os.path.join(path, "/nginx", "file.txt"))

# Path
path = "/usr"

# Join various path components
print(os.path.join(path, "bin/", "file.txt", "/home"))