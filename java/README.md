# Java Implementation

## Setup

### Java

Install the Java development kits. Java 11 is required by the Language Support for Java by Red Had VSCode extension.

    brew install openjdk@8
    brew install openjdk@11

Open VSCode's settings with `Cmd + Shift + P` → `Preference: Open SEttings (JSON)` and add the following settings

```json
    "java.configuration.runtimes": [
        {
            "name": "JavaSE-1.8",
            "path": "/usr/local/opt/openjdk@8/libexec/openjdk.jdk/Contents/Home",
        }
    ],
```

For the system Java wrappers to find the JDKs, symlink them.

    sudo ln -sfn /usr/local/opt/openjdk@8/libexec/openjdk.jdk /Library/Java/JavaVirtualMachines/openjdk-8.jdk

    sudo ln -sfn /usr/local/opt/openjdk@11/libexec/openjdk.jdk /Library/Java/JavaVirtualMachines/openjdk-11.jdk



Install the package manager.

    brew install maven

Install the web server.

    brew install tomcat
## Development
Make a build.

    mvn compile

Run unit tests run.

    mvn test

Make a production build.

    mvn package

Clear build artifacts.

    mvn clean

Make a production build image.

    docker build -t java-car-rentals:latest

Run an ephemeral docker container.

    docker run --rm -it java-car-rentals:latest
