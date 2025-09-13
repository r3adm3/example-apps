# Go Web Application Example

A lightweight web server built with Go and the Gin framework, demonstrating modern Go web development practices and containerization.

## 🚀 Quick Start

### Prerequisites
- Go 1.24 or later
- Docker (optional, for containerization)

### Local Development

1. **Install dependencies**
   ```bash
   go mod download
   ```

2. **Run the application**
   ```bash
   go run hello.go
   ```

3. **Test the application**
   - Open your browser to `http://localhost:3000/hello`
   - You should see: "Hello, World! (go)"

### Docker Deployment

1. **Build Docker image**
   ```bash
   docker build -t example-go .
   ```

2. **Run container**
   ```bash
   docker run -p 3000:3000 example-go
   ```

3. **Test the application**
   - Open your browser to `http://localhost:3000/hello`

## 📁 Project Structure

```
example-go/
├── hello.go              # Main application file
├── go.mod               # Go module definition
├── go.sum               # Dependency checksums (auto-generated)
├── Dockerfile           # Multi-stage Docker build
└── README.md           # This file
```

## 🛣️ API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/hello` | Returns a simple greeting message |

## 🔧 Configuration

### Port
- Default: `3000`
- Configurable via the `r.Run()` parameter in `hello.go`

### Key Features
- ✅ Gin web framework for high performance
- ✅ Multi-stage Docker build with Alpine Linux
- ✅ Non-root user in container for security
- ✅ Static binary compilation for minimal runtime dependencies
- ✅ JSON routing and middleware support

## 📦 Dependencies

The application uses the [Gin Web Framework](https://gin-gonic.com/) along with its dependencies:

- **gin-gonic/gin** - HTTP web framework
- **bytedance/sonic** - High-performance JSON library
- **go-playground/validator** - Struct validation
- Plus additional supporting libraries (see go.mod)

## 🐳 Docker Details

The Dockerfile implements a multi-stage build:

1. **Builder stage**: 
   - Uses `golang:1.25-alpine` for compilation
   - Downloads dependencies and builds static binary
   - Enables CGO_ENABLED=0 for static linking

2. **Runtime stage**:
   - Uses `alpine:latest` for minimal footprint
   - Runs as non-root user for security
   - Includes CA certificates for HTTPS requests

## 🚀 Performance Notes

- Gin is one of the fastest HTTP routers for Go
- The static binary approach eliminates runtime dependencies
- Alpine Linux base image keeps container size minimal
- Built-in request logging and recovery middleware

## 📚 Learn More

- [Gin Web Framework Documentation](https://gin-gonic.com/docs/)
- [Go Modules Reference](https://golang.org/ref/mod)
- [Go Docker Best Practices](https://docs.docker.com/language/golang/)