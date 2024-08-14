# Iterator



iterator 就是實作了std::iter::Iterator trait的任何值


```rust
trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
  ...// 許多其他方法
}
```

Item 是iterator產生的值的型態。next方法會回傳Some(v)，v是iterator的下一個值，當序列結束，會回傳None。

如果某個型態可以用自然的方式迭代，你可以為那個型態實作`std::iter::IntoIterator`，他的into_iter方法會接受一個值，並回傳它的`iterator`：


```rust
trait IntoIterator where Self::IntoIter: Iterator<Item=Self::Item> {
  typ Item;
  type IntoIter: Iterator;
  fn into_iter(self) -> Self::IntoIter;
}
```


Rust的for迴圈就是如此整合上面的元素，假設你要迭代一個向量印出他們：


```extnedr
println!("There's:");
let v = vec!["a", "b", "c"];

for element in &v {
  println!("{}", element);
}
```


在底層，for迴圈其實就是呼叫IntoIterator與Iterator方法的簡寫而已


```rust
let mut iterator = (&v).into_iter();
while let Some(element) = iterator.next {
  println!("{}", element);
}
```


我們來了解一下iterator的術語：

- 之前說過，iterator是實作了Iterator的任何型態

- iterable是可以實作Interator的任何型態。

- iterator會產生值

- iterator產生的值稱為項目(item)。在此，項目是"a"、"b"、"c"

- 接受iterator產生的值的項目是consumer，在上面例子，for迴圈是consumer

## 建立Iterator

### iter與iter_mut方法

大多數的集合型態都提供iter與iter_mut方法，這些方法會回傳迭代該型態的iterator，可產生每一個項目的共享或可變參考。&[T]與&mut [T]這種陣列slice也有iter與iter_mut方法，如果不用for迴圈，以下是最常用來取得iterator方法：


```rust
let v = vec![1,2,3,4,5];
let mut iterator = v.iter();
rprintln!("{:?}", iterator.next());
rprintln!("{:?}", iterator.next());
rprintln!("{:?}", iterator.next());
rprintln!("{:?}", iterator.next());
rprintln!("{:?}", iterator.next());
rprintln!("{:?}", iterator.next());
#> Some(1)
#> Some(2)
#> Some(3)
#> Some(4)
#> Some(5)
#> None
```

每個型態實作的iter與iter_mut後的iterator可能有些不同，例如std::path::Path的iter方法回傳一個iterator，它每次會回傳一個路徑組件，這些項目型態是&std::ffi::OsStor。


```rust
use std::ffi::OsStr;
use std::path::Path;

let path = Path::new("C:/User/JimB/Dowloads/Fedora.iso");
let mut iterator = path.iter();
rprintln!("{:?}", iterator.next());
rprintln!("{:?}", iterator.next());
rprintln!("{:?}", iterator.next());
#> Some("C:")
#> Some("User")
#> Some("JimB")
```

如果某種型態迭代不只一種，那種型態通常為各種遍歷方式提供轉數的方法，例如&str字串就沒有iter方法，而是有`s.bytes()`與`s.chars()`來取代。

### IntoIterator

如果型態實作了IntoIterator，你可以自己呼叫它的into_iter方法，就像for迴圈的做法


```rust
use std::collections::BTreeSet;
let mut favorites = BTreeSet::new();
favorites.insert("Lucky in the Sky With Diamonds".to_string());
favorites.insert("Liebestraume No. 3".to_string());

let mut it = favorites.into_iter();
rprintln!("{:?}", it.next());
rprintln!("{:?}", it.next());
rprintln!("{:?}", it.next());
#> Some("Liebestraume No. 3")
#> Some("Lucky in the Sky With Diamonds")
#> None
```

不同狀況的iterator會產生不同的item，來處理共享參考(&T)、可變參考(&mut T)與移動(T)：

- 當into_iter接收集合的共享參考後，它會回傳一個iterator，這個iterator會產生項目的共享參考。例如，上面的程式中，(&favorite).into_iter()會回傳一個iterator，它的Item型態是&String

- 當into_iter收到集合的可變參考後，它會回傳一個iterator，這個iterator會產生項目的可變參考。例如，如果vector是Vec<String>，那麽呼叫(&mut vector).into_iter()會回傳一個iterator，它的Item型態是&mut String

- 當into_iter以值收到集合後，into_iter會回傳一個iterator，該iterator擁有集合的所有權，並以值回傳項目；項目的所有權會從集合移交給耗用者，原始的集合在過程中會被耗用。例如上述的程式中呼叫favorites.into_iter()會回傳一個iterator，它會以值產生每個字串，耗用者將每一個字串的所有權。當iterator被卸除時，在BTreeSet裡面剩餘的任何元素也會被卸除，然後集合的空殼也會被丟棄。


也並非每一個型態都會實作三種情況，例如HashSet、BTreeSet與BinaryHeap未實作處理可變參考的Iterator，因為修改本身會違反型態的不可變性：如修改過後產生不同的雜湊值，或與臨值的排列不同。

而有些僅部分可變，如HashMap產生的項目值的可變參考，但只有索引鍵是共享參考。

slice本身不擁有它的元素，所以不能以**值**參考，只實作共享參考與可變參考

另外，`favorites.iter()`與`(&favorites).into_iter()`都是以共享參考作為迭代，但前者更加簡潔與易懂

IntoIterator在泛型程式也很有用，使用`T: IntoIterator`來將型態變數T限制為可以迭代的型態。進一步可以用`T: IntoIterator<Item=U>`  來要求迭代特並型態U。


```rust
use std::fmt::Debug;

fn dump<T, U>(t: T)
    where T: IntoIterator<Item=U>,
    U: debug
{
  for u in t {
    println!("{:?}", u);
  }
}
```

## form_fn 與 successors

若要產生一系列的值，一種簡單的方式是提供一個回傳他們的closure。


當`std::iter::form_fn`接受一個回傳`Option<T>`的closure，並回傳iterator不停呼叫這個函式來產生值，搭配`take()`函式可以限制產生的數字。


```rust
use rand::random;
use std::iter::form_fn;

// 產生1000個隨機線段的長度
// 會在[0,1]區間內產生值
let lengths: Vec<f64> =
    form_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
    .take(1000)
    .collect();
```


如果每個項目依賴前一個項目，`std::iter::successors`函式可以很好的運作


```rust
use std::iter::successors;

let start = 1;

let mut iter = successors(Some(start), |&x| Some(x+2) );

for _ in 0..5 {
  if let Some(val) = iter.next() {
    rprintln!("{}", val);
  }
}
#> 1
#> 3
#> 5
#> 7
#> 9
#> NULL
```

successors接受兩個參數，1是初始值，2是一個回傳值的closure。第一次迭代中，closure的參數是初始值(上例中的`start`)，隨後回傳值(回傳`Some(start+2)`)。第二次迭代，closure的參數是上一次迭代的值(上例中的`start+2`)...


form_fn與successor兩者都可以接受`FnMut closure`，用來捕捉作用域的變數並修改它。例如，這個`fibonacci`函式使用一個`move
closure`來捕捉變數，並將它當成它的運作狀態來使用


```rust
use std::iter::from_fn;

fn fibonacci() -> impl Iterator<Item=usize> {
  let mut state = (0,1);
  std::iter::from_fn(move || {
    state = (state.1, state.0 + state.1);
    Some(state.0)
  })
}

let mut x = fibonacci().take(8);
for _ in 0..8 {
  if let Some(val) = x.next() {
    rprintln!("{}", val);
  }
}
#> 1
#> 1
#> 2
#> 3
#> 5
#> 8
#> 13
#> 21
#> NULL
```


from_fn與successor方法很靈活，幾乎可以用在所有iterator的地方，來讓你的程式更簡潔與，但它們相較於常用模式，較難說明資料如何在計算過程中移動。

### darin方法


如果型態可以用範圍檢索，例如String、向量與VecDeque，drain方法可以擷取範圍內的item。


```rust

let mut outer = "Rust".to_string();
let inner = String::from_iter(outer.drain(1..3));

rprintln!("outer：{}", outer);
rprintln!("inner：{}", inner);
#> outer：Rt
#> inner：us
```

## Iterator改造方法

關於Iterator有兩個重點：

1.對著iterator呼叫adapter不會耗用任何值，而是**回傳一個新iterator**

以下這段程式不會印出任何值：


```rust
["Java", "Rust", "css", "R"]
  .iter().map(|elt| println!("{}", elt));
```

程式會警告你：`note: iterators are lazy and do nothing unless consumed`。

`iter()`方法會回傳ㄧ個迭代陣列元素吧iterator，接著map會回傳第二個iterator，但沒人呼叫鍵索取值，也就是完全沒有人呼叫`next()`來讓這個iterator工作。

2.iterator是抽象零成本的。如map、filter都是泛型，所以執行它們時，當對著iterator執行它們時，Rust會根據參與其中的iterator類型量身打造程式碼，因此它有足夠的資訊將各個iterator的next方法內連至它的耗用者裡面，然後將整個安排視為一個單位，翻譯成機器碼，讓它與你親自撰寫函式來過濾執行ㄧ樣高效。



### map與filter

map adapter可對iterator的項目執行一個closure，filter adpater則可從iterator篩選項目，用closure來決定保留項目。

假設你要迭代多行文字，並省略每一行的空格與結尾，使用map來呼叫trim方法就非常適合


```rust
let text = "hello, RUST!\n 妳好，羅斯特\n Java is better\n QQ".to_string();

let v: Vec<&str> = text.lines()
    .map(str::trim)
    .collect();
    
for word in v {
  rprintln!("{}", word);
}
#> hello, RUST!
#> 妳好，羅斯特
#> Java is better
#> QQ
#> NULL
```

map完的iterator還可以繼續改造，假如你想過濾有Java的字串，你可以使用filter：



```rust
let text = "hello, RUST!\n 妳好，羅斯特\n Java is better\n QQ".to_string();

let v: Vec<&str> = text.lines()
    .map(str::trim)
    .filter(|s| !s.contains("Java"))
    .collect();
    
for word in v {
  rprintln!("{}", word);
}
#> hello, RUST!
#> 妳好，羅斯特
#> QQ
#> NULL
```

iterator的adapter的寫法很容易閱讀，每一個adapter都有一個目的，並且有續的把改造結果往下傳


這些adpater的簽章：


```rust
fn map<B, F>(self, f:F) -> impl Iterator<Item=B>
  where Self: Sized, F: FnMut(Self::Item) -> B;

fn filter<P>(self, predicate: P) -> impl Iterator<Item=Self::Item>
    where Self: Sized, P: FnMut(&Self::Item) -> bool;
```

map iterator將每個項目以值傳給他的closure，並依序將closure產生的結果的所有權交給它的耗用者。filter iterator以共享參考傳給它的closure，因此有時在使用filter要注意解參考，如：`filter(|s| *s != QQ);`參數型是`&&str`，要解參考後才能直接比較。

### filter_map與fat_map

map adapter最適合處理每一個項目都要產生下一個項目，而filter adapter適合回傳零或ㄧ個目，但如果以想將項目一次轉換成一個或多個項目呢？或是想刪除某個項目而不處理呢？

filter_map就如語意上的使用方式，就是map+filter，它可以讓vlosure轉換成新項目或選擇卸除，以下是它的簽章：


```rust
fn filter_map<B, F>(self, f: F) -> impl Iterator<Item=B>
    where Self: Sized, F: FnMut(Self::Item) -> Option<B>;
```

這個簽章與mapㄧ樣，但它回傳的是Option<B>，而不是B。當closure回傳`None`時，項目會卸除值，當它回傳`Some(b)`時，b是filter_map iterator產生的下一個項目。

例如，我們要從字串提取數字，必須先以空格分開後，找出單詞後卸除無法解析的詞：


```rust
use std::str::FromStr;

let text = "11\n found .25 289 \n 3.1415 kkday\n";

let numbers: Vec<f64> = text.split_whitespace()
    .filter_map(|t| f64::from_str(t).ok())
    .collect();
    
rprintln!("{:?}", numbers);
    
#> [11.0, 0.25, 289.0, 3.1415]
```


當filter_map的closure試著使用`f64::from_str`來解析各個以空白分開的slice，並回傳Result<f64, ParseFloatError>，它的.ok()會轉換成Option<f64>：解析錯誤會變成None，而成功的解析則是Some(v)

而flat_map則是可以回傳不只一個項目或零個，而是任意數量的項目，它的簽章如此：


```rust
fn flat_map<U, F>(self, f:F) -> impl Iterator<Item=U::Item>
   where F: FnMut(Self::Item) -> U, U: IntoIterator;
```

傳給flat_map的closure必須回傳一個**iterable**，但任何形式的iterable都可以。


```rust
use std::collections::HashMap;

let mut major_cities = HashMap::new();
major_cities.insert ("Japan", vec! ["Tokyo", "Kyoto"]);
major_cities.insert("TheUnitedStates", vec!["Portland","Nashville"]);
major_cities.insert("Brazil", vec! ["São Paulo","Brasilia"]);
major_cities.insert("Kenya", vec! ["Nairobi","Mombasa" ]);

let countries = ["Japan", "Brazil", "Kenya"];

for &city in countries.iter().flat_map(|country| &major_cities[country]) {
  rprintln!("{}", city);
}
#> Tokyo
#> Kyoto
#> São Paulo
#> Brasilia
#> Nairobi
#> Mombasa
#> NULL
```


### flatten

flatten adapter會串接iterator的項目，將數個項目輾平成單層，回傳一個傳遞所有元素的iterator


```rust
use std::collections::HashMap;

let mut transportions = HashMap::new();
transportions.insert("陸地", vec!["火車","公車"]);
transportions.insert("海上", vec!["渡輪", "獨木舟"]);
transportions.insert("空中", vec!["客機", "直升機"]);

let all_transportions:Vec<_> = transportions.values().flatten().collect();

for transportion in &all_transportions {
  rprintln!("{}", transportion);
}
#> 渡輪
#> 獨木舟
#> 客機
#> 直升機
#> 火車
#> 公車
#> NULL
```

flatten的簽章是長這樣的

