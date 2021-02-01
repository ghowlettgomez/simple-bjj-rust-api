This repository is a sample restful api made using rust and postgresql.

It was build using the following guide and GitHub Repository:

https://blog.logrocket.com/create-a-backend-api-with-rust-and-postgres/
https://github.com/olajohn-ajiboye/Rust-Rest-API/tree/175cd11222613e4fe7363fb8fe6dde45302eef7b

In order to run the project, one can follow the instructions in the above logrocket guide. One can build and run this via the following process:

cargo build

diesel setup

diesel migration run

systemfd --no-pid -s http::PORT_NUMBER -- cargo watch -x run