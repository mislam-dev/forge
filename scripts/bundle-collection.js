#!/usr/bin/env node

/**
 * Postman v12 Local Workspace to Postman Collection v2.1 Bundler
 * 
 * Traverses `postman/postman/collections/Forge Platform API`, parses request YAML files,
 * preserves order metadata and test scripts, and emits a standard Postman Collection v2.1 JSON.
 */

const fs = require('fs');
const path = require('path');
const { parse } = require('yaml');
const crypto = require('crypto');

const ROOT_COLLECTION_DIR = path.resolve(__dirname, '../postman/postman/collections/Forge Platform API');
const OUTPUT_FILE = path.resolve(__dirname, '../postman/forge-platform-api.collection.json');

function parseUrl(rawUrl) {
  return rawUrl || '';
}


function processRequestFile(filePath) {
  const fileContent = fs.readFileSync(filePath, 'utf8');
  const data = parse(fileContent);

  const baseName = path.basename(filePath, '.request.yaml');
  const order = typeof data.order === 'number' ? data.order : 9999;

  // Headers
  const headerList = [];
  if (data.headers) {
    if (Array.isArray(data.headers)) {
      for (const h of data.headers) {
        headerList.push({
          key: h.key || h.name,
          value: String(h.value ?? ''),
          disabled: Boolean(h.disabled)
        });
      }
    } else if (typeof data.headers === 'object') {
      for (const [key, val] of Object.entries(data.headers)) {
        headerList.push({
          key: key,
          value: String(val ?? ''),
          disabled: false
        });
      }
    }
  }

  // Request Body
  let requestBody = undefined;
  if (data.body) {
    if (data.body.type === 'json' || data.body.type === 'text') {
      requestBody = {
        mode: 'raw',
        raw: typeof data.body.content === 'string' ? data.body.content : (data.body.content ? JSON.stringify(data.body.content, null, 2) : ''),
        options: {
          raw: {
            language: data.body.type === 'json' ? 'json' : 'text'
          }
        }
      };
    } else if (data.body.type === 'form-data' && Array.isArray(data.body.content)) {
      requestBody = {
        mode: 'formdata',
        formdata: data.body.content.map(item => ({
          key: item.key,
          value: String(item.value ?? ''),
          type: item.type || 'text'
        }))
      };
    } else if (data.body.type === 'urlencoded' && Array.isArray(data.body.content)) {
      requestBody = {
        mode: 'urlencoded',
        urlencoded: data.body.content.map(item => ({
          key: item.key,
          value: String(item.value ?? '')
        }))
      };
    }
  }

  // Events / Scripts
  const events = [];
  if (Array.isArray(data.scripts)) {
    for (const script of data.scripts) {
      const listen = script.type === 'afterResponse' ? 'test' : 'prerequest';
      const execLines = typeof script.code === 'string' ? script.code.split('\n') : [];
      events.push({
        listen: listen,
        script: {
          type: 'text/javascript',
          exec: execLines
        }
      });
    }
  }

  const postmanItem = {
    name: baseName,
    _order: order,
    request: {
      method: (data.method || 'GET').toUpperCase(),
      header: headerList,
      url: parseUrl(data.url),
      description: data.description || ''
    }
  };

  if (requestBody) {
    postmanItem.request.body = requestBody;
  }

  if (events.length > 0) {
    postmanItem.event = events;
  }

  return postmanItem;
}

function processDirectory(dirPath, folderName) {
  let order = 9999;
  let description = '';

  const defPath = path.join(dirPath, '.resources', 'definition.yaml');
  if (fs.existsSync(defPath)) {
    try {
      const defData = parse(fs.readFileSync(defPath, 'utf8'));
      if (typeof defData.order === 'number') {
        order = defData.order;
      }
      if (typeof defData.description === 'string') {
        description = defData.description;
      }
    } catch (e) {
      // Ignore definition parse errors
    }
  }

  const entries = fs.readdirSync(dirPath, { withFileTypes: true });
  const items = [];

  for (const entry of entries) {
    if (entry.name.startsWith('.')) continue; // Skip hidden dirs and .resources

    const fullPath = path.join(dirPath, entry.name);
    if (entry.isDirectory()) {
      const subFolder = processDirectory(fullPath, entry.name);
      if (subFolder.item.length > 0) {
        items.push(subFolder);
      }
    } else if (entry.isFile() && entry.name.endsWith('.request.yaml')) {
      const requestItem = processRequestFile(fullPath);
      items.push(requestItem);
    }
  }

  // Sort items by _order, then by name
  items.sort((a, b) => {
    const orderA = a._order ?? 9999;
    const orderB = b._order ?? 9999;
    if (orderA !== orderB) return orderA - orderB;
    return a.name.localeCompare(b.name);
  });

  // Remove internal _order property before finishing
  for (const item of items) {
    delete item._order;
  }

  return {
    name: folderName,
    _order: order,
    description: description,
    item: items
  };
}

function bundle() {
  console.log(`Bundling Postman collection from: ${ROOT_COLLECTION_DIR}`);

  if (!fs.existsSync(ROOT_COLLECTION_DIR)) {
    console.error(`Error: Collection directory not found at ${ROOT_COLLECTION_DIR}`);
    process.exit(1);
  }

  let rootDescription = 'Forge Platform API Collection';
  const variables = [];

  const rootDefPath = path.join(ROOT_COLLECTION_DIR, '.resources', 'definition.yaml');
  if (fs.existsSync(rootDefPath)) {
    try {
      const rootDef = parse(fs.readFileSync(rootDefPath, 'utf8'));
      if (rootDef.description) {
        rootDescription = rootDef.description;
      }
      if (rootDef.variables && typeof rootDef.variables === 'object') {
        for (const [k, v] of Object.entries(rootDef.variables)) {
          variables.push({
            key: k,
            value: String(v ?? ''),
            type: 'string'
          });
        }
      }
    } catch (e) {
      console.warn('Warning: Could not parse root definition.yaml:', e.message);
    }
  }

  const rootFolder = processDirectory(ROOT_COLLECTION_DIR, 'Forge Platform API');

  const collection = {
    info: {
      _postman_id: crypto.randomUUID(),
      name: 'Forge Platform API',
      description: rootDescription,
      schema: 'https://schema.getpostman.com/json/collection/v2.1.0/collection.json'
    },
    item: rootFolder.item,
    variable: variables
  };

  fs.mkdirSync(path.dirname(OUTPUT_FILE), { recursive: true });
  fs.writeFileSync(OUTPUT_FILE, JSON.stringify(collection, null, 2), 'utf8');

  // Count total requests
  let totalRequests = 0;
  function countRequests(items) {
    for (const item of items) {
      if (item.request) totalRequests++;
      if (item.item) countRequests(item.item);
    }
  }
  countRequests(collection.item);

  console.log(`Successfully bundled ${totalRequests} requests into: ${OUTPUT_FILE}`);
}

bundle();
