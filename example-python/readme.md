# Python Flask Web Application Example

A simple web application built with Flask framework, demonstrating Python web development fundamentals and containerization best practices.

## 🚀 Quick Start

### Prerequisites
- Python 3.8 or later
- pip (Python package installer)
- Docker (optional, for containerization)

### Local Development

1. **Set up virtual environment**
   ```bash
   python3 -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   ```

2. **Install dependencies**
   ```bash
   pip install -r requirements.txt
   ```

3. **Run the application**
   ```bash
   # Option 1: Direct execution
   python3 app.py
   
   # Option 2: Using Flask CLI
   flask run
   ```

4. **Test the application**
   - Open your browser to `http://localhost:5000`
   - You should see: "Hello World! (python)"

### Docker Deployment

1. **Build Docker image**
   ```bash
   docker build -t example-python .
   ```

2. **Run container**
   ```bash
   docker run -p 8000:8000 example-python
   ```

3. **Test the application**
   - Open your browser to `http://localhost:8000`

## 📁 Project Structure

```
example-python/
├── app.py               # Main Flask application
├── requirements.txt     # Python dependencies
├── Dockerfile          # Container build instructions
└── README.md           # This file
```

## 🛣️ API Endpoints

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| GET | `/` | Root endpoint | `"Hello World! (python)"` |

## 🔧 Configuration

### Development Port
- Default: `5000` (Flask default)
- Accessible at `http://localhost:5000`

### Production Port (Docker)
- Default: `8000`
- Configured in Dockerfile CMD

### Key Features
- ✅ Flask micro web framework
- ✅ Virtual environment for dependency isolation
- ✅ Multi-stage Docker build with security best practices
- ✅ Non-root user in container
- ✅ Python 3.13 for latest features

## 📦 Dependencies

- **flask** - Lightweight WSGI web application framework
- **numpy** - Fundamental package for scientific computing (example dependency)

## 🐳 Docker Details

The Dockerfile implements security and optimization best practices:

1. **Base image**: `python:3.13-slim` for smaller footprint
2. **Security features**:
   - Runs as non-root user (`appuser`)
   - Disables Python bytecode generation
   - Unbuffered Python output for better logging

3. **Build optimization**:
   - Requirements copied first for better layer caching
   - Build dependencies cleaned after installation
   - Minimal system packages

## 🔄 Development Commands

```bash
# Activate virtual environment
source venv/bin/activate

# Install new dependency
pip install package_name
pip freeze > requirements.txt

# Run with debug mode
export FLASK_DEBUG=1
flask run

# Run on specific port
flask run --port 3000

# Run with host binding for network access
flask run --host=0.0.0.0 --port=5000
```

## 🚀 Expanding the Application

### Adding Routes
```python
@app.route('/api/health')
def health():
    return {"status": "healthy", "service": "python-flask"}

@app.route('/api/user/<name>')
def user(name):
    return f"Hello, {name}! (python)"
```

### Environment Configuration
```python
import os

app.config['DEBUG'] = os.environ.get('FLASK_DEBUG', False)
port = int(os.environ.get('PORT', 5000))

if __name__ == "__main__":
    app.run(host='0.0.0.0', port=port)
```

### JSON Responses
```python
from flask import jsonify

@app.route('/api/data')
def get_data():
    return jsonify({
        'message': 'Hello World!',
        'language': 'python',
        'framework': 'flask'
    })
```

## 📊 Performance Notes

- Flask development server is single-threaded (suitable for development)
- For production, consider using WSGI servers like Gunicorn or uWSGI
- Virtual environments prevent dependency conflicts
- Docker container uses slim base image for faster deployments

## 🔧 Production Deployment

For production use, consider:

```bash
# Install production WSGI server
pip install gunicorn

# Run with Gunicorn
gunicorn -w 4 -b 0.0.0.0:8000 app:app
```

Update Dockerfile CMD to:
```dockerfile
CMD ["gunicorn", "-w", "4", "-b", "0.0.0.0:8000", "app:app"]
```

## 📚 Learn More

- [Flask Documentation](https://flask.palletsprojects.com/)
- [Python Virtual Environments Guide](https://docs.python.org/3/tutorial/venv.html)
- [Python Docker Best Practices](https://docs.docker.com/language/python/)
- [Gunicorn Documentation](https://gunicorn.org/)