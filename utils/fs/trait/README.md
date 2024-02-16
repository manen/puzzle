# `fs_trait`

the `fs_trait` api is highly generic by default, any resemblance to the `Iterator` trait is very much intentional

```rs
use fs_trait::prelude::*;

let fs = crate::empty().mount_file("README.md", "# hello world!".as_bytes().read_only());
```

notes:

- designed for use with fw slash `/`
- uses absolute paths (in all places not too unconvenient)
