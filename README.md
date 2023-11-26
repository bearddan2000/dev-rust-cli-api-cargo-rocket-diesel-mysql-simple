# dev-rust-cli-api-cargo-rocket-diesel-mysql-simple

## Description
A rust REST Api for rocket web framework that
calls mysql through diesel.

## Tech stack
- rust
- cargo
  - rocket
  - diesel
- mysql connector

## Docker stack
- rust:1.43
- mariadb:latest

## To run
`sudo ./install.sh -u`
- GET : `curl -i localhost/dogs

## To stop (optional)
`sudo ./install.sh -d`

## For help
`sudo ./install.sh -h`

## Credits
- [Rocket example](https://blog.logrocket.com/rust-web-apps-using-rocket-framework/)
- [Diesel example](https://github.com/kayrules/rust-rocket-mysql-starter/tree/master)