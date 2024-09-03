
1. compile mdbook wjson
   ```shell
    mdbook build ./books/wjson/ -d $"(pwd)/mysite/static/wjson" --open
   ```
   or 
   ```shell
   mdbook serve -o ./books/wjson
   ```
   
2. runs zola
   ```shell
    cd mysite; zola serve -O
   ```