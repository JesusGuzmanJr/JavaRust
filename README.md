# Java & Rust Comparison of a Restful Service

This repository showcases two implementations of a restful api service. Its intent is to enable Java practitioners to compare and contrast a web service implemented in both languages. Both servers are reachable and running in production.
___
## About the tech stacks

### ☕️ Why Java?
Many companies use Java for its ease-of-use and mature libraries.
Java's strongest feature is its object-oriented paradigm that enables developers to model their business logic as a hierarchy of objects.

### 🦀 Why Rust?
Rust earned the top spot as the “most-loved” programming language for the sixth consecutive year of [Stack Overflow’s developer survey]. The Rust ecosystem has recently seen an explosion in adoption with backing from [major companies]. Rust's most unique feature is the borrow checker, a compilation step that validates references and reclaims inaccessible memory thereby eliminating the need for a runtime garbage collector. This enables Rust programs to be blazingly fast and memory-efficient.

[Stack Overflow’s developer survey]: https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/
[major companies]: https://foundation.rust-lang.org/members/

### Stack Comparison

| Stack              | Java                                                   | Rust                                |
| ------------------ | ------------------------------------------------------ | ----------------------------------- |
| domain name        | java.marzipan.club                                     | rust.marzipan.club                  |
| health endpoint    | https://java.marzipan.club/health                      | https://rust.marzipan.club/health   |
| language           | [Java SE 8] ([OpenJDK 8 8u292-b10])                    | [Rust 2018] ([1.54.0])              |
| compiler           | javac 1.8.0_302                                        | rustc 1.54.0 (a178d0322 2021-07-26) |
| target             | java bytecode 52.0                                     | stable-x86_64-unknown-linux-gnu     |
| runtime            | [OpenJDK] ([java-1.8.0-openjdk])                       | [Tokio] ([0.2.25])                  |
| web framework      | [Apache Tomcat] ([8.5.69]) and [Spring Boot] ([2.5.2]) | [Actix Web] ([3.3.2])               |
| package manager    | mvn                                                    | cargo                               |
| manifest file      | [pom.xml]                                              | [cargo.toml]                        |
| configuration file | [application.properties]                               | [config.ron]                        |


[Java SE 8]: https://docs.oracle.com/javase/8
[OpenJDK 8 8u292-b10]: https://mail.openjdk.java.net/pipermail/jdk8u-dev/2021-April/013680.html

[Rust 2018]: https://www.rust-lang.org
[1.54.0]: https://blog.rust-lang.org/2021/07/29/Rust-1.54.0.html

[OpenJDK]: https://openjdk.java.net
[java-1.8.0-openjdk]: https://openjdk.java.net/install

[Tokio]:https://tokio.rs
[0.2.25]: https://crates.io/crates/tokio/0.2.25

[Apache Tomcat]: https://tomcat.apache.org
[8.5.69]: https://tomcat.apache.org/download-80.cgi#8.5.69

[Spring Boot]: https://spring.io
[2.5.2]: https://github.com/spring-projects/spring-boot/tree/v2.5.2

[Actix Web]: https://actix.rs
[3.3.2]: https://crates.io/crates/actix-web/3.3.2

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
| cost                   | $50 per year                     |
---
