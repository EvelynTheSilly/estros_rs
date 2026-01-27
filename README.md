# EstrOS
a kernel/operating system originally written in zig



## why?
originally i got inspired to work on this by @DevelopedFromScratch on youtube, however nowdays i work on this to help me mentally, no matter how much time passes this project always makes me happy whenever i get sucked into it 

why the rust rewrite?

it mainly falls down to my inexperience with zig, i dont know much of how it works and am especially uncomfortable in its type system, but it also provided me a lot of knowledge when it comes to how cortex a works

## developing
<sub>why would anyone ever wanna use this but me :thinking:</sub>

enter the dev environment
~~~sh
nix develop # if you dont use nix.... too bad? you can find the package list in the flake
~~~
run with
~~~sh
mask run # cleans/builds/runs the project
~~~
debug with 
~~~sh
# terminal 1
mask debug

# terminal 2
mask start_gdb
~~~

## credits
- based on [aarch64-bare-metal-qemu](https://github.com/freedomtan/aarch64-bare-metal-qemu/tree/master)
- name by evelyn (not the same evelyn as above)
- heavily insipired to do any of this in the first place by [developed from scratch](https://www.youtube.com/@DevelopedFromScratch)
