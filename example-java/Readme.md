# Java Spring Boot Web Application Example

A RESTful web application built with Spring Boot 3.2 and Java 17, demonstrating modern Java web development with multiple endpoints and containerization.

## 🚀 Quick Start

### Prerequisites
- Java 17 or later
- Maven 3.6+ 
- Docker (optional, for containerization)

### Local Development

1. **Run the application**
   ```bash
   mvn spring-boot:run
   ```

2. **Test the application**
   - Open your browser to `http://localhost:8080`
   - You should see: "Hello World! (Java/Maven)"

### Docker Deployment

1. **Build Docker image**
   ```bash
   docker build -t example-java .
   ```

2. **Run container**
   ```bash
   docker run -p 8080:8080 example-java
   ```

3. **Test the application**
   - Open your browser to `http://localhost:8080`

## 📁 Project Structure

```
example-java/
├── src/
│   └── main/
│       └── java/
│           └── com/example/helloworld/
│               └── HelloWorldApplication.java  # Main application class
├── pom.xml                                     # Maven configuration
├── Dockerfile                                  # Multi-stage Docker build
└── README.md                                  # This file
```

## 🛣️ API Endpoints

| Method | Endpoint | Description | Example Response |
|--------|----------|-------------|------------------|
| GET | `/` | Basic hello world | `"Hello World! (Java/Maven)"` |
| GET | `/greeting` | Default greeting | `"Hello (java) World!"` |
| GET | `/greeting?name=John` | Personalized greeting | `"Hello (java) John!"` |
| GET | `/status` | Application health check | `"Application is running successfully!"` |

## 🔧 Configuration

### Port
- Default: `8080` (both development and production)

### Key Features
- ✅ Spring Boot 3.2 with auto-configuration
- ✅ RESTful API endpoints with parameter binding
- ✅ Embedded Tomcat server
- ✅ Maven dependency management
- ✅ Multi-stage Docker build
- ✅ Non-root user in container for security

## 📦 Dependencies

- **spring-boot-starter-web** - Web development starter (includes Tomcat, Jackson, Spring MVC)
- **spring-boot-starter-test** - Testing framework starter (JUnit, Mockito, etc.)

See `pom.xml` for complete dependency tree with version management via Spring Boot parent.

## 🐳 Docker Details

The Dockerfile uses a multi-stage build for optimization:

1. **Build stage**:
   - Uses `maven:3.9.11` for building
   - Downloads dependencies offline first for better caching
   - Builds JAR file with `mvn clean package`

2. **Runtime stage**:
   - Uses `eclipse-temurin:latest` JRE for smaller footprint
   - Runs as non-root user (`spring:spring`)
   - Exposes port 8080

## 🏗️ Build Commands

```bash
# Clean and compile
mvn clean compile

# Run tests
mvn test

# Package as JAR
mvn clean package

# Run with specific profile
mvn spring-boot:run -Dspring-boot.run.profiles=dev

# Skip tests during build
mvn clean package -DskipTests
```

## 📊 Application Startup

The application typically starts in 2-3 seconds and includes:
- Embedded Tomcat server on port 8080
- Auto-configured Spring context
- Built-in health monitoring
- Request/response logging (configurable)

## 📚 Learn More

- [Spring Boot Documentation](https://spring.io/projects/spring-boot)
- [Spring Boot 3.2 Release Notes](https://github.com/spring-projects/spring-boot/wiki/Spring-Boot-3.2-Release-Notes)
- [Maven Getting Started Guide](https://maven.apache.org/guides/getting-started/)
- [Building Java Applications with Docker](https://spring.io/guides/gs/spring-boot-docker/)