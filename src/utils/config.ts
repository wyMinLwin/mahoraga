import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';
import type { Config } from '../types.js';

const CONFIG_DIR = path.join(os.homedir(), '.mahoraga');
const CONFIG_FILE = path.join(CONFIG_DIR, 'config.json');

export function getDefaultConfig(): Config {
  return {
    url: '',
    apiKey: '',
    deployment: '',
    apiVersion: '2024-02-15-preview',
  };
}

export function ensureConfigDir(): void {
  if (!fs.existsSync(CONFIG_DIR)) {
    fs.mkdirSync(CONFIG_DIR, { recursive: true });
  }
}

export function readConfig(): Config | null {
  try {
    if (!fs.existsSync(CONFIG_FILE)) {
      return null;
    }
    const data = fs.readFileSync(CONFIG_FILE, 'utf-8');
    return JSON.parse(data) as Config;
  } catch {
    return null;
  }
}

export function writeConfig(config: Config): void {
  ensureConfigDir();
  fs.writeFileSync(CONFIG_FILE, JSON.stringify(config, null, 2), 'utf-8');
}

export function resetConfig(): void {
  const defaultConfig = getDefaultConfig();
  writeConfig(defaultConfig);
}

export function isConfigured(config: Config | null): boolean {
  if (!config) return false;
  return !!(config.url && config.apiKey && config.deployment && config.apiVersion);
}
