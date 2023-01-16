<div align="center">
    <h1>aoi - Pre Development Draft</h1>
</div>

aoi *- [葵] popular plant in Japan -* is **struct based web framework**.\
This is pre development draft.

<br/>

## Features
- **struct based architecture** where a server is a struct
- actively using **macros**

<br/>

## Examples ( under planning )

### simple server
```rust
#[server]
MyServer {
    #[GET /]
    {
        self.OK("Hello!")
    }
}

#[main]
{
    let my_server = MyServer;
    bloom!(":3000", my_server)
}
```
```sh
$ curl http://localhost:3000
Hello!
```

### handle db, path param, req body
```rust
struct User {
    id:   u64,
    name: String,
}

struct CreateUser {
    name:     String,
    password: String,
}

#[server]
UserServer {
    conn: ConnectionPool<Postgres>,
} {
    #[GET /api/users/{id:u64}]
    {
        let user = SQL![
            SELECT id, name FROM users
            WHERE id = $id
        ; as User]
            .fetch_one(&self.conn)
            .await?;

        self.OK(user)
    }

    #[POST /api/users, body: CreateUser]
    {
        let CreateUser { name, password } = body;

        let created_user = SQL![
            INSERT INTO users (name, password) VALUES ($name, $password)
            RETURNING id, name
        ; as User]
            .fetch_one(&self.conn)
            .await?;

        self.Created(created_user)
    }
}

#[main]
{
    let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            "postgres://postgres:password@localhost/test"
        ).await?;

    let user_server = UserServer {
        conn: connection_pool,
    };

    bloom!(":8080", user_server)
}
```

### nest servers
```rust
struct CreateUser {
    name:     String,
    password: String,
}

#[server]
RootServer {
    #[GET /]
    {
        self.OK("Hello!")
    }

    #[GET /health_check]
    {
        self.NoContent()
    }
}

#[server]
UserServer {
    conn: ConnectionPool<Postgres>,
} {
    #[POST /, body: CreateUser]
    {
        let CreateUser { name, password } = body;

        let created_user = SQL![
            INSERT INTO users (name, password) VALUES ($name, $password)
            RETURNING id, name
        ; as User]
            .fetch_one(&self.conn)
            .await?;

        self.Created(created_user)
    }

    #[GET /{id:u64}]
    {
        let user = SQL![
            SELECT id, name
            FROM users
            WHERE name = $name AND password = $password
        ; as User]
            .fetch_one(&self.conn)
            .await?;

        self.OK(user)
    }
}

#[main]
{
    let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            "postgres://postgres:password@localhost/test"
        ).await?;

    let user_server = UserServer {
        conn: connection_pool,
    };

    let root_server = RootServer;

    bloom!(":8080",
        /api/users => user_server,
        / => root_server
    )
}
```