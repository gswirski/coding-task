# coding-task

### Detect/identify duplicate files in a (recursive) directory

Write a program that takes one argument (a path to a folder), and prints
whether the folder contains files with duplicate contents. For example,
given the following folder structure:

```
/test
﹂/folder_a
   ﹂/folder_a_a
      ﹂/hello.txt (contents: "hello")
   ﹂/folder_a_b
      ﹂/hello.txt (contents: "world")
```

Running `cargo run -- /test` prints "no duplicates found" (different file contents).

However, the example below:

```
/test
﹂/folder_a
   ﹂/folder_a_a
      ﹂/hello.txt (contents: "hello")
﹂/folder_b
   ﹂/world.txt (contents: "hello")
```

Prints "found duplicates" (exact same contents).

#### Bonus points

Print which files are duplicated.
