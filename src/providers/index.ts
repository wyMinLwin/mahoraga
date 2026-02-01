import type { Config, Provider } from '../types.js';
import { AzureProvider } from './azure.js';

export function createProvider(config: Config): Provider {
  // Currently only Azure is supported
  // Future providers can be selected based on config or additional settings
  return new AzureProvider(config);
}

export { AzureProvider } from './azure.js';
export { BaseProvider } from './base.js';
