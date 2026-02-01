#!/usr/bin/env node
import React from 'react';
import { render } from 'ink';
import { App } from './app.js';

const args = process.argv.slice(2);

function printHelp() {
  console.log(`
Mahoraga - Prompt Validation TUI

Usage:
  mahoraga summon    Launch the interactive prompt validator
  mahoraga --help    Show this help message
  mahoraga --version Show version

Commands (in app):
  /settings   Configure API settings
  /default    Reset settings to defaults
  /clear      Clear current analysis
  /exit       Exit the application
  Ctrl+C      Exit the application
`);
}

function printVersion() {
  console.log('mahoraga v1.0.0');
}

function main() {
  const command = args[0];

  if (command === '--help' || command === '-h') {
    printHelp();
    process.exit(0);
  }

  if (command === '--version' || command === '-v') {
    printVersion();
    process.exit(0);
  }

  if (command === 'summon' || !command) {
    render(<App />);
    return;
  }

  console.error(`Unknown command: ${command}`);
  console.error('Run "mahoraga --help" for usage information.');
  process.exit(1);
}

main();
