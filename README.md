# Example Applications Repository

A collection of simple "Hello World" web applications demonstrating basic setup and deployment across different programming languages and frameworks. Each example includes both local development instructions and Docker containerization for easy deployment.

## 🚀 Available Examples

| Language | Framework | Port | Description |
|----------|-----------|------|-------------|
| [.NET](./example-dotnet/) | ASP.NET Core | 8080 | Minimal API with .NET 9.0 |
| [Go](./example-go/) | Gin | 3000 | Lightweight web server with Gin framework |
| [Java](./example-java/) | Spring Boot | 8080 | RESTful API with Spring Boot 3.2 |
| [Node.js](./example-node/) | Express | 3000 | Basic HTTP server with Express |
| [Python](./example-python/) | Flask | 8000 | Simple web application with Flask |

## 🏃‍♂️ Quick Start

Each example can be run in two ways:

### Local Development
Navigate to any example directory and follow the language-specific instructions in its README.

### Docker (Recommended)
```bash
# Choose any example directory
cd example-[language]

# Build and run with Docker
docker build -t example-[language] .
docker run -p [port]:[port] example-[language]
```

## 📁 Repository Structure

```
example-apps/
├── README.md                 # This file
├── example-dotnet/          # .NET Core example
├── example-go/              # Go with Gin example  
├── example-java/            # Java Spring Boot example
├── example-node/            # Node.js Express example
└── example-python/          # Python Flask example
```

## 🔧 Common Requirements

- **Docker**: For containerized deployment (recommended)
- **Language-specific runtime**: For local development (see individual READMEs)

## 🎯 Use Cases

These examples are perfect for:
- Learning basic web development setups across languages
- Testing deployment pipelines
- Container orchestration demonstrations  
- Microservices architecture prototyping
- CI/CD pipeline testing
- Cloud deployment tutorials

## 🤝 Contributing

Feel free to:
- Add examples in new languages/frameworks
- Improve existing examples
- Update documentation
- Report issues or suggest enhancements

## 📋 Testing the Examples

After running any example, test it works by visiting:
- `http://localhost:[PORT]/` - Basic hello world response
- Check individual README files for additional endpoints

Each application returns a simple "Hello World" message with the language/framework identifier.