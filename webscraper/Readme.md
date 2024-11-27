A Little Web Scraper
===

webscraper is a command line tool for a little web scraper.

!!! It is just a ***TUTORIAL*** program for rust `async` function.

Just pass in two URLs from the command line, fetch both of them concurrently,
and return the result of whichever one finishes first. Just as following:

```bash
D:\proj\github.com\hzget\rust-apps\webscraper>cargo run "http://www.rust-lang.org" "http://www.apple.com"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `target\debug\webscraper.exe http://www.rust-lang.org http://www.apple.com`
http://www.apple.com returned first
Its page title is: 'Apple'
```
