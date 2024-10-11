# Blog with Rocket and Diesel

This is just an incomplete REST API for a blog for my learning purposes.
It uses the "Rocket" Rust framework with Diesel ORM, and PostgreSQL database.

## 1. Setup & Usage

1. Need to have **PostgreSQL** installed on your computer.
2. Set the **PQ_LIB_DIR** OS environmental variable to the correct path, for
   example: `C:\Program Files\PostgreSQL\17\lib`.
3. Set the `DATABASE_URL` in the **.env** file (_see example_)

4. Install Diesel CLI

```shell
cargo install diesel_cli --no-default-features --features postgres`
```

5. Configure Diesel

- cd to infrastructure
- generate `diesel.toml` file:

```shell
diesel setup
```
6. Run all SQL migrations (it also generates the `schema.rs` file):

```shell
diesel migration run
```

7. Copy `schema.rs` from `infrastructure/src` to `domain\src`.
8. Update `schema.rs` file path in `diesel.toml` configuration file to the new location

9. Build and run project (from the root of the project, not from inside the `infrastructure` folder!):

```shell
cargo run
```

9. The API is accessible at http://127.0.0.1:8000.


## 2. Endpoints information

```raw
>> (create_post_handler) POST /api/post application/json
>> (list_posts_handler) GET /api/posts
>> (all_options_handler) OPTIONS /api/<_..>
>> (list_post_handler) GET /api/post/<post_id>
>> (update_post_handler) PUT /api/post/<post_id> application/json
>> (delete_post_handler) DELETE /api/post/<post_id>
>> (publish_post_handler) PUT /api/post/publish/<post_id>
```

See the `infrastructure/migrations` folder for the database structure.