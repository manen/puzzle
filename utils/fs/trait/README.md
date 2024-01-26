# `fs_trait`

the `fs_trait` api is highly generic by default, any resemblance to the `Iterator` trait is very much intentional

```rs
use fs_trait::prelude::*;

let fs = crate::empty().mount_file("README.md", "# hello world!".as_bytes().read_only());
```

notes:

- designed for use with `/`
- uses root slash (is that what they call it? whatever its just `/xd.txt` instead of `xd.txt`, can be worked around using `fs.abs()`)
