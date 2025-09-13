# .NET Core Web Application Example

A minimal ASP.NET Core web application built with .NET 9.0 that demonstrates basic web API setup and containerization.

## 🚀 Quick Start

### Prerequisites
- .NET 9.0 SDK or later
- Docker (optional, for containerization)

### Local Development

1. **Install dependencies**
   ```bash
   dotnet restore
   ```

2. **Build the application**
   ```bash
   dotnet build
   ```

3. **Run the application**
   ```bash
   dotnet run
   ```

4. **Test the application**
   - Open your browser to `http://localhost:5040`
   - You should see: "Hello World! (.NET)"

### Docker Deployment

1. **Build Docker image**
   ```bash
   docker build -t example-dotnet .
   ```

2. **Run container**
   ```bash
   docker run -p 8080:8080 example-dotnet
   ```

3. **Test the application**
   - Open your browser to `http://localhost:8080`

## 📁 Project Structure

```
example-dotnet/
├── Program.cs                  # Main application entry point
├── example-dotnet.csproj      # Project configuration
├── Dockerfile                 # Multi-stage Docker build
├── Properties/
│   └── launchSettings.json    # Development launch settings
├── appsettings.json          # Application configuration
├── appsettings.Development.json # Development-specific settings
└── README.md                 # This file
```

## 🔧 Configuration

### Development Ports
- HTTP: `http://localhost:5040`
- HTTPS: `https://localhost:7020`

### Production Port (Docker)
- HTTP: `http://localhost:8080`

### Key Features
- ✅ .NET 9.0 minimal APIs
- ✅ Multi-stage Dockerfile for optimized builds
- ✅ Development and production configurations
- ✅ Nullable reference types enabled
- ✅ Implicit usings for cleaner code

## 📦 Dependencies

- **Newtonsoft.Json** (13.0.3) - JSON serialization
- **System.Text.Json** (9.0.2) - High-performance JSON APIs

## 🐳 Docker Details

The Dockerfile uses a multi-stage build:
1. **Build stage**: Uses `mcr.microsoft.com/dotnet/sdk:9.0` for compilation
2. **Runtime stage**: Uses `mcr.microsoft.com/dotnet/aspnet:9.0` for deployment

This approach significantly reduces the final image size by excluding build tools from the runtime image.

## 📚 Learn More

- [ASP.NET Core Documentation](https://docs.microsoft.com/en-us/aspnet/core/)
- [.NET 9.0 What's New](https://docs.microsoft.com/en-us/dotnet/core/whats-new/dotnet-9)
- [Minimal APIs](https://docs.microsoft.com/en-us/aspnet/core/fundamentals/minimal-apis)