# example-java

To run locally

```bash
mvn spring-boot:run
```

## Available Endpoints

Once running, you can access:

http://localhost:8080/ - Returns "Hello World!"
http://localhost:8080/greeting - Returns "Hello World!"
http://localhost:8080/greeting?name=John - Returns "Hello John!"
http://localhost:8080/status - Returns application status

This is a minimal Spring Boot web application that includes:

A main application class with embedded Tomcat server
Three REST endpoints
Maven configuration for dependency management
Multi-stage Dockerfile for containerization
Runs on port 8080 by default

The application will start in a few seconds and be ready to serve HTTP requests!