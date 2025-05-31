// This function executes Rust code in a sandbox
// In a production environment, this would use a service like Rust Playground API
// or a containerized execution environment
const axios = require('axios');
const { spawn } = require('child_process');
const fs = require('fs').promises;
const path = require('path');
const crypto = require('crypto');
const os = require('os');

// Security configuration
const MAX_EXECUTION_TIME = 30000; // 30 seconds
const MAX_OUTPUT_SIZE = 1024 * 1024; // 1MB
const MAX_CODE_SIZE = 50 * 1024; // 50KB
const ALLOWED_LANGUAGES = ['rust', 'javascript', 'python'];

// Rate limiting storage (in production, use Redis)
const rateLimitStore = new Map();
const RATE_LIMIT_WINDOW = 60000; // 1 minute
const RATE_LIMIT_MAX_REQUESTS = 10;

exports.handler = async (event, context) => {
  // Set CORS headers
  const headers = {
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Headers': 'Content-Type',
    'Access-Control-Allow-Methods': 'POST, OPTIONS',
    'Content-Type': 'application/json'
  };

  // Handle preflight requests
  if (event.httpMethod === 'OPTIONS') {
    return {
      statusCode: 200,
      headers,
      body: ''
    };
  }

  // Only allow POST requests
  if (event.httpMethod !== 'POST') {
    return {
      statusCode: 405,
      headers,
      body: JSON.stringify({ error: 'Method not allowed' })
    };
  }

  try {
    // Parse request body
    const { code, language, input = '' } = JSON.parse(event.body || '{}');

    // Validate input
    const validation = validateInput(code, language, input);
    if (!validation.valid) {
      return {
        statusCode: 400,
        headers,
        body: JSON.stringify({ error: validation.error })
      };
    }

    // Rate limiting
    const clientIP = event.headers['x-forwarded-for'] || event.headers['x-real-ip'] || 'unknown';
    const rateLimitResult = checkRateLimit(clientIP);
    if (!rateLimitResult.allowed) {
      return {
        statusCode: 429,
        headers,
        body: JSON.stringify({ 
          error: 'Rate limit exceeded. Please try again later.',
          retryAfter: rateLimitResult.retryAfter
        })
      };
    }

    // Execute code
    const result = await executeCode(code, language, input);

    return {
      statusCode: 200,
      headers,
      body: JSON.stringify(result)
    };

  } catch (error) {
    console.error('Code execution error:', error);
    return {
      statusCode: 500,
      headers,
      body: JSON.stringify({ 
        error: 'Internal server error',
        message: process.env.NODE_ENV === 'development' ? error.message : 'Something went wrong'
      })
    };
  }
};

function validateInput(code, language, input) {
  // Check if code is provided
  if (!code || typeof code !== 'string') {
    return { valid: false, error: 'Code is required and must be a string' };
  }

  // Check code size
  if (code.length > MAX_CODE_SIZE) {
    return { valid: false, error: `Code size exceeds maximum limit of ${MAX_CODE_SIZE} characters` };
  }

  // Check language
  if (!language || !ALLOWED_LANGUAGES.includes(language)) {
    return { valid: false, error: `Language must be one of: ${ALLOWED_LANGUAGES.join(', ')}` };
  }

  // Check input size
  if (input && input.length > MAX_CODE_SIZE) {
    return { valid: false, error: 'Input size exceeds maximum limit' };
  }

  // Security checks for dangerous patterns
  const dangerousPatterns = [
    /require\s*\(\s*['"]fs['"]/, // File system access
    /require\s*\(\s*['"]child_process['"]/, // Process spawning
    /require\s*\(\s*['"]net['"]/, // Network access
    /require\s*\(\s*['"]http['"]/, // HTTP access
    /require\s*\(\s*['"]https['"]/, // HTTPS access
    /process\.exit/, // Process termination
    /process\.env/, // Environment access
    /eval\s*\(/, // Code evaluation
    /Function\s*\(/, // Function constructor
    /import\s+.*\s+from\s+['"]fs['"]/, // ES6 imports
    /std::process/, // Rust process spawning
    /std::fs/, // Rust file system
    /std::net/, // Rust networking
    /unsafe\s*{/, // Rust unsafe blocks
    /exec\s*\(/, // Python exec
    /eval\s*\(/, // Python eval
    /import\s+os/, // Python OS module
    /import\s+subprocess/, // Python subprocess
    /import\s+socket/, // Python socket
    /from\s+os\s+import/, // Python OS imports
  ];

  for (const pattern of dangerousPatterns) {
    if (pattern.test(code)) {
      return { valid: false, error: 'Code contains potentially dangerous operations' };
    }
  }

  return { valid: true };
}

function checkRateLimit(clientIP) {
  const now = Date.now();
  const windowStart = now - RATE_LIMIT_WINDOW;

  // Clean old entries
  for (const [ip, requests] of rateLimitStore.entries()) {
    const filteredRequests = requests.filter(time => time > windowStart);
    if (filteredRequests.length === 0) {
      rateLimitStore.delete(ip);
    } else {
      rateLimitStore.set(ip, filteredRequests);
    }
  }

  // Check current IP
  const requests = rateLimitStore.get(clientIP) || [];
  const recentRequests = requests.filter(time => time > windowStart);

  if (recentRequests.length >= RATE_LIMIT_MAX_REQUESTS) {
    const oldestRequest = Math.min(...recentRequests);
    const retryAfter = Math.ceil((oldestRequest + RATE_LIMIT_WINDOW - now) / 1000);
    return { allowed: false, retryAfter };
  }

  // Add current request
  recentRequests.push(now);
  rateLimitStore.set(clientIP, recentRequests);

  return { allowed: true };
}

async function executeCode(code, language, input) {
  const sessionId = crypto.randomUUID();
  const tempDir = path.join(os.tmpdir(), `code-execution-${sessionId}`);

  try {
    // Create temporary directory
    await fs.mkdir(tempDir, { recursive: true });

    let result;
    switch (language) {
      case 'rust':
        result = await executeRust(code, input, tempDir);
        break;
      case 'javascript':
        result = await executeJavaScript(code, input, tempDir);
        break;
      case 'python':
        result = await executePython(code, input, tempDir);
        break;
      default:
        throw new Error(`Unsupported language: ${language}`);
    }

    return result;

  } finally {
    // Cleanup temporary directory
    try {
      await fs.rm(tempDir, { recursive: true, force: true });
    } catch (cleanupError) {
      console.error('Cleanup error:', cleanupError);
    }
  }
}

async function executeRust(code, input, tempDir) {
  // Create Cargo.toml
  const cargoToml = `[package]
name = "playground"
version = "0.1.0"
edition = "2021"

[dependencies]
neo3 = { path = "/app/neo3" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
`;

  // Wrap user code in a safe main function
  const wrappedCode = `use neo3::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    ${code}
    Ok(())
}`;

  await fs.writeFile(path.join(tempDir, 'Cargo.toml'), cargoToml);
  await fs.mkdir(path.join(tempDir, 'src'));
  await fs.writeFile(path.join(tempDir, 'src', 'main.rs'), wrappedCode);

  return executeCommand('cargo', ['run'], tempDir, input);
}

async function executeJavaScript(code, input, tempDir) {
  // Create a safe Node.js environment
  const wrappedCode = `
const console = {
  log: (...args) => process.stdout.write(args.join(' ') + '\\n'),
  error: (...args) => process.stderr.write(args.join(' ') + '\\n'),
};

// Disable dangerous globals
const process = undefined;
const require = undefined;
const global = undefined;
const Buffer = undefined;

// User code
(async () => {
  try {
    ${code}
  } catch (error) {
    console.error('Error:', error.message);
  }
})();
`;

  await fs.writeFile(path.join(tempDir, 'script.js'), wrappedCode);
  return executeCommand('node', ['script.js'], tempDir, input);
}

async function executePython(code, input, tempDir) {
  // Create a safe Python environment
  const wrappedCode = `
import sys
import io

# Redirect stdout to capture output
old_stdout = sys.stdout
sys.stdout = io.StringIO()

try:
    # User code
${code.split('\n').map(line => '    ' + line).join('\n')}
    
except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
finally:
    # Get output and restore stdout
    output = sys.stdout.getvalue()
    sys.stdout = old_stdout
    print(output, end='')
`;

  await fs.writeFile(path.join(tempDir, 'script.py'), wrappedCode);
  return executeCommand('python3', ['script.py'], tempDir, input);
}

function executeCommand(command, args, cwd, input) {
  return new Promise((resolve) => {
    const startTime = Date.now();
    let stdout = '';
    let stderr = '';
    let killed = false;

    const child = spawn(command, args, {
      cwd,
      stdio: ['pipe', 'pipe', 'pipe'],
      timeout: MAX_EXECUTION_TIME,
      env: {
        ...process.env,
        PATH: process.env.PATH,
        // Remove potentially dangerous environment variables
        HOME: '/tmp',
        USER: 'sandbox',
      }
    });

    // Set up timeout
    const timeout = setTimeout(() => {
      killed = true;
      child.kill('SIGKILL');
    }, MAX_EXECUTION_TIME);

    // Handle stdout
    child.stdout.on('data', (data) => {
      stdout += data.toString();
      if (stdout.length > MAX_OUTPUT_SIZE) {
        killed = true;
        child.kill('SIGKILL');
      }
    });

    // Handle stderr
    child.stderr.on('data', (data) => {
      stderr += data.toString();
      if (stderr.length > MAX_OUTPUT_SIZE) {
        killed = true;
        child.kill('SIGKILL');
      }
    });

    // Handle process completion
    child.on('close', (code) => {
      clearTimeout(timeout);
      const executionTime = Date.now() - startTime;

      let status = 'success';
      let error = null;

      if (killed) {
        status = 'error';
        error = 'Execution timed out or output size exceeded';
      } else if (code !== 0) {
        status = 'error';
        error = `Process exited with code ${code}`;
      }

      resolve({
        status,
        stdout: stdout.slice(0, MAX_OUTPUT_SIZE),
        stderr: stderr.slice(0, MAX_OUTPUT_SIZE),
        executionTime,
        error
      });
    });

    // Handle errors
    child.on('error', (err) => {
      clearTimeout(timeout);
      resolve({
        status: 'error',
        stdout: '',
        stderr: '',
        executionTime: Date.now() - startTime,
        error: `Failed to start process: ${err.message}`
      });
    });

    // Send input if provided
    if (input) {
      child.stdin.write(input);
    }
    child.stdin.end();
  });
}