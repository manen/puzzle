# `utils/iter/join`

based on [this gist](https://gist.github.com/manen/7eeb6e04a21306da637c08acdb21581d)

```rs
use iter_join::JoinItem;

fn main() {
  let words: [&str; 3] = ["hello", "i'm", "testing"];
  let text = words.into_iter().join(" ").collect::<String>();

  assert_eq!(text, "hello i'm testing");
}
```

```rs
use iter_join::JoinIter;

fn main() {
  let nums_1 = [i32; 3] = [0, 2, 4];
  let nums_2 = [i32; 3] = [1, 3, 5];
  let nums = nums_1.into_iter().join_iter(nums_2).collect::<Vec<i32>>;

  assert_eq!(nums, &[0, 1, 2, 3, 4]);
}
```

join will only return an element from `b` if there's a next element in `a`
