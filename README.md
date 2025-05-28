# Httpp

A plain text HTTP client with dotenv support, zero bloat, and cleaner
syntax than curl scripts.

# Notice

Unfinished software, coded by retards. Expect nothing, be pleasantly
surprised.

# Usage

``` shell
# Execute request file with environment variable
httpp exec file.httpp env-file

# Execute request without environment variable
httpp exec file.httpp

# Generate .httpp file
httpp generate --file new.http --request post

# Version, help, etc
httpp
```

# Syntax

## Note

By default, the `generate` command assumes you will be using a `.env`
file.

## With `.env`

``` http
POST {{APP}}/users
Content-Type: application/json
Accept: application/json

{
  "user": "{{USER}}",
  "password": "{{PASSWORD}}"
}
```

## Without `.env`

``` http
POST http://localhost:9999/users
Content-Type: application/json
Accept: application/json

{
  "user": "NateNateNate",
  "password": "John"
}
```
