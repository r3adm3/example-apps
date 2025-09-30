# Quarkus Hello World Application

A simple Hello World REST API built with Quarkus.

## Project Structure

```
.
├── src/
│   └── main/
│       └── java/
│           └── org/
│               └── acme/
│                   └── GreetingResource.java
├── pom.xml
├── Dockerfile
└── README.md
```

## Prerequisites

- Java 17 or higher
- Maven 3.8.1 or higher
- Docker (for containerization)

## Running the Application Locally

### Development Mode (with live reload)

```bash
mvn quarkus:dev
```

Or on Windows:

```bash
mvnw.cmd quarkus:dev
```

The application will start on `http://localhost:8080`

### Test the Endpoint

```bash
curl http://localhost:8080/hello
```

Expected response: `Hello World!`

### Package and Run

Build the application:

```bash
mvn clean package
```

Run the packaged application:

```bash
java -jar target/quarkus-app/quarkus-run.jar
```

## Containerizing the Application

### Build the Application

First, package the application:

```bash
mvn clean package
```

### Build Docker Image

```bash
docker build -t quarkus-hello-world .
```

### Run the Container

```bash
docker run -p 8080:8080 quarkus-hello-world
```

The application will be available at `http://localhost:8080/hello`

### Stop the Container

Find the container ID:

```bash
docker ps
```

Stop the container:

```bash
docker stop <container-id>
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET    | `/hello` | Returns "Hello World!" |

## Building a Native Executable (Optional)

If you want to create a native executable for faster startup:

```bash
mvn package -Pnative
```

This requires GraalVM to be installed.

## Additional Resources

- [Quarkus Documentation](https://quarkus.io/guides/)
- [Quarkus REST Guide](https://quarkus.io/guides/resteasy-reactive)
