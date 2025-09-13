# Node.js Web Application Example

A simple HTTP server built with Node.js using the native `http` module and Express framework, demonstrating basic web server setup and containerization.

## 🚀 Quick Start

### Prerequisites
- Node.js 18 or later
- npm (comes with Node.js)
- Docker (optional, for containerization)

### Local Development

1. **Install dependencies**
   ```bash
   npm install
   ```

2. **Run the application**
   ```bash
   # Option 1: Direct execution
   node server.js
   
   # Option 2: Using npm script
   npm start
   ```

3. **Test the application**
   - Open your browser to `http://localhost:3000`
   - You should see: "Hello World (node)"

### Docker Deployment

1. **Build Docker image**
   ```bash
   docker build -t example-node .
   ```

2. **Run container**
   ```bash
   docker run -p 3000:3000 example-node
   ```

3. **Test the application**
   - Open your browser to `http://localhost:3000`

## 📁 Project Structure

```
example-node/
├── server.js            # Main server file
├── package.json         # Node.js project configuration
├── package-lock.json    # Dependency lock file (auto-generated)
├── Dockerfile          # Container build instructions
└── README.md           # This file
```

## 🛣️ API Endpoints

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| ALL | `/` | Any request to root | `"Hello World (node)"` |

The server responds to all HTTP methods (GET, POST, PUT, DELETE, etc.) on the root path.

## 🔧 Configuration

### Port
- Default: `3000`
- Configurable via the `port` variable in `server.js`

### Host
- Binds to `0.0.0.0` (all interfaces) for Docker compatibility
- Accessible via `localhost` when running locally

### Key Features
- ✅ Native Node.js HTTP module (no external dependencies for core functionality)
- ✅ Express.js included for potential expansion
- ✅ Docker containerization with Alpine Linux
- ✅ npm scripts for easy management
- ✅ Lightweight and fast startup

## 📦 Dependencies

- **express** (^4.5.1) - Fast, unopinionated web framework

Note: The current implementation uses Node.js native `http` module. Express is included in `package.json` for easy migration to a more feature-rich setup.

## 🐳 Docker Details

The Dockerfile uses Node.js 22 with Alpine Linux:
- **Base image**: `node:22-alpine` for smaller footprint
- **Working directory**: `/app`
- **Exposed port**: `3000`
- **Start command**: `npm start`

## 🔄 Development Tips

### Expanding to Express
To switch from native HTTP to Express, replace `server.js` content with:

```javascript
const express = require('express');
const app = express();
const port = 3000;

app.get('/', (req, res) => {
  res.send('Hello World (node with Express)');
});

app.listen(port, '0.0.0.0', () => {
  console.log(`Server running at http://0.0.0.0:${port}/`);
});
```

### Adding More Routes
```javascript
// Add to server.js after the main server setup
app.get('/api/health', (req, res) => {
  res.json({ status: 'OK', timestamp: new Date().toISOString() });
});
```

### Environment Variables
```javascript
const port = process.env.PORT || 3000;
const host = process.env.HOST || '0.0.0.0';
```

## 🚀 Performance Notes

- Uses Node.js event loop for high concurrency
- Minimal memory footprint with native HTTP module
- Fast startup time (~100ms)
- Suitable for microservices and serverless deployments

## 📚 Learn More

- [Node.js HTTP Module Documentation](https://nodejs.org/api/http.html)
- [Express.js Documentation](https://expressjs.com/)
- [Node.js Docker Best Practices](https://nodejs.org/en/docs/guides/nodejs-docker-webapp/)