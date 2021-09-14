# Java & Rust Comparison of a Restful Service

This repository showcases two implementations of a restful api service. Its intent is to enable Java practitioners to compare and contrast a web service implemented in both languages. Both are running live now.
___
## About the tech stacks

### ☕️ Why Java?
Many companies use Java for its ease-of-use and mature libraries.
Java's strongest feature is its object-oriented paradigm that enables developers to model their business logic as a hierarchy of objects.

### 🦀 Why Rust?
Rust earned the top spot as the “most-loved” programming language for the sixth consecutive year of [Stack Overflow's developer survey]. The Rust ecosystem has recently seen an explosion in adoption with backing from [major companies]. Rust's most unique feature is the borrow checker, a compilation procedure that validates references and reclaims inaccessible memory thereby eliminating the need for a runtime garbage collector. This enables Rust programs to be blazingly fast and memory-efficient.

[Stack Overflow's developer survey]: https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/
[major companies]: https://foundation.rust-lang.org/members/

### Stack Comparison

| Stack                | [Java]                            | [Rust]                              |
| -------------------- | --------------------------------- | ----------------------------------- |
| domain name          | java.marzipan.club                | rust.marzipan.club                  |
| health endpoint      | http://java.marzipan.club/info    | http://rust.marzipan.club/info      |
| language             | [Java SE 11]                      | [Rust 1.55.0]                       |
| compiler             | javac 11.0.12                     | rustc 1.55.0 (c8dfcfe04 2021-09-06) |
| compilation target   | java bytecode 55.0                | stable-x86_64-unknown-linux-gnu     |
| compiles to          | bytecode                          | machine code                        |
| runtime              | [OpenJDK]                         | [Tokio]                             |
| web framework        | [Apache Tomcat] and [Spring Boot] | [Actix Web]                         |
| package manager      | mvn                               | cargo                               |
| manifest file        | [pom.xml]                         | [cargo.toml]                        |
| configuration file   | [application.properties]          | [config.ron]                        |
| compile and run      | `mvn spring-boot:run`             | `cargo run`                         |
| prod compile and run | *same as above*                   | `cargo run --release`               |
| clean command        | `mvn clean`                       | `cargo clean`                       |
| dependency tree      | `mvn dependency:tree`             | `cargo tree`                        |

[Java]:https://jdk.java.net/java-se-ri/11
[Rust]:https://www.rust-lang.org/

[Java SE 11]: https://access.redhat.com/documentation/en-us/openjdk/11/html/release_notes_for_openjdk_11.0.12/index
[Rust 1.55.0]: https://blog.rust-lang.org/2021/09/09/Rust-1.55.0.html

[OpenJDK]: https://openjdk.java.net
[Tokio]:https://tokio.rs

[Apache Tomcat]: https://tomcat.apache.org
[Spring Boot]: https://spring.io
[Actix Web]: https://actix.rs

[pom.xml]: java/pom.xml
[cargo.toml]: rust/cargo.toml

[application.properties]: java/src/main/resources/application.properties
[config.ron]: rust/config.ron


| Server Specs per Stack |                                  |
| ---------------------- | -------------------------------- |
| operating system       | Fedora 34                        |
| cpu                    | 2  vCPU cores (AMD EPYC 2nd Gen) |
| memory                 | 2 GB RAM                         |
| disk space             | 40 GB NVMe                       |
| cost                   | 50 USD per year                  |
---
