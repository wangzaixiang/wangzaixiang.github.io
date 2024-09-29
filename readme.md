
1. compile mdbook wjson
   ```shell
    mdbook build ./books/wjson/ -d $"(pwd)/mysite/static/wjson" --open
   ```
   or 
   ```shell
   mdbook serve -o ./books/wjson
   ```
   
2. compile mdbook inside-duckdb
    ```shell
     mdbook build ./books/inside-duckdb/ -d $"(pwd)/mysite/static/inside-duckdb" --open
    ```
    or 
    ```shell
     mdbook serve -d $"(pwd)/mysite/static/inside-duckdb" --open ./books/inside-duckdb 
    ```
   
2. runs zola
   ```shell
    cd mysite; zola serve -O
   ```