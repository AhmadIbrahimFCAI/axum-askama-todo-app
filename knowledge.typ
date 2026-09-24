+ ```sh
  sudo -u postgres psql
  ```

+ ```sql
  create user ahmad with password 'ahmad';
  alter user ahmad with superuser;
  alter user ahmad with createdb;
  ```

+ @ `.env` ```sh DATABASE_URL=postgres://ahmad:ahmad@localhost/axum_todos ```

+ ```sh
  sqlx database create
  ```


