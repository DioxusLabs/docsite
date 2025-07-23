# Path Functions

You can use path functions to perform operations on valid path strings.

### join

<!-- TODO: Add specifics.
From the example given, it seems like it just creates a subdirectory path.
What would it do when "extending" file paths? -->
Extend a path; you can extend both directory and file paths.

```lua
local current_path = "~/hello/dioxus"
local new_path = plugin.path.join(current_path, "world")
-- new_path = "~/hello/dioxus/world"
```

### parent

Return the parent path of the specified path. The parent path is always a directory.

```lua
local current_path = "~/hello/dioxus"
local new_path = plugin.path.parent(current_path)
-- new_path = "~/hello/"
```

### exists

Check if the specified path exists, as either a file or a directory.

### is_file

Check if the specified path is a file.

### is_dir

Check if the specified path is a directory.