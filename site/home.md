## Build static sites <u>quick.</u>

Insite is an assembly-like language for building simple websites in minutes, not days.

```
; reusable components
.section nav_bar:
    ld horizontal, left ; ui layout features
    md "Welcome to Insite!"
    exp
    md "[Home](/)" ; first-class markdown support
    str

; define individual pages and their paths
.page "Home", "/":
    jmp nav_bar
    md "# This is my home page!"
```

<button onclick="window.location.href='/insite/docs';">Get Started</button>
