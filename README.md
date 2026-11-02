<img src="assets/icon.svg" width="25%">
    
Insite is an assembly-like programming language for building static websites. 

```
; define reusable routines
.section navigation_bar:
    ld horizontal, center ; load a layout
    md "# This is a website!" ; first-class markdown support
    exp
    md "[Go Home](/)"
    str ; store a layout

; define individual pages
.page "Home", "/" ; title, path
    jmp navigation_bar
    ld vertical, center
    md "# Welcome to my home page!"
    str
```
