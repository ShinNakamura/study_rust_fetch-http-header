# 複数のURLに対して並行でHEADER要素を取得する

URLが改行区切りで書いてあるテキストが用意してあり、

こんな
```txt
https://foo.com
https://bar.com
https://baz.com
```

これを標準入力から読み込んで、それぞれのHEADERを確認し、
`url,status,content-type,content-length,last-modified`
のCSV形式で標準出力に返す。

あまり一気にリクエストするとよくないので
リクエストのタスクの中にスリープを仕込んで、
ある程度リクエストの勢いが緩くなるようにしてある。

