import React, { useState, useEffect } from 'react';
import { Box, Text, useInput } from 'ink';
import TextInput from 'ink-text-input';
import { COLORS } from '../utils/theme.js';
import type { Config } from '../types.js';

interface SettingsProps {
  config: Config;
  onSave: (config: Config) => void;
  onCancel: () => void;
}

type FieldName = 'url' | 'apiKey' | 'deployment' | 'apiVersion';

const FIELDS: { name: FieldName; label: string; placeholder: string }[] = [
  { name: 'url', label: 'URL', placeholder: 'https://your-resource.openai.azure.com' },
  { name: 'apiKey', label: 'API Key', placeholder: 'Your Azure API key' },
  { name: 'deployment', label: 'Deployment', placeholder: 'Your model deployment name' },
  { name: 'apiVersion', label: 'API Version', placeholder: '2024-02-15-preview' },
];

export function Settings({ config, onSave, onCancel }: SettingsProps) {
  const [activeField, setActiveField] = useState(0);
  const [values, setValues] = useState<Config>({ ...config });

  useEffect(() => {
    setValues({ ...config });
  }, [config]);

  useInput((input, key) => {
    if (key.escape) {
      onCancel();
      return;
    }

    if (key.tab || (key.return && activeField < FIELDS.length - 1)) {
      setActiveField((prev) => (prev + 1) % FIELDS.length);
      return;
    }

    if (key.return && activeField === FIELDS.length - 1) {
      onSave(values);
      return;
    }
  });

  const updateField = (field: FieldName) => (value: string) => {
    setValues((prev) => ({ ...prev, [field]: value }));
  };

  return (
    <Box flexDirection="column" marginY={1}>
      <Text bold color={COLORS.primary}>Settings</Text>
      <Text color={COLORS.muted}>Configure your Azure OpenAI connection</Text>
      <Box flexDirection="column" marginTop={1}>
        {FIELDS.map((field, index) => (
          <Box key={field.name} marginY={0}>
            <Box width={12}>
              <Text color={activeField === index ? COLORS.primary : COLORS.muted}>
                {field.label}:
              </Text>
            </Box>
            <Box flexGrow={1}>
              {activeField === index ? (
                <TextInput
                  value={values[field.name]}
                  onChange={updateField(field.name)}
                  placeholder={field.placeholder}
                  focus={true}
                  mask={field.name === 'apiKey' ? '*' : undefined}
                />
              ) : (
                <Text color={COLORS.white}>
                  {field.name === 'apiKey' && values[field.name]
                    ? '*'.repeat(Math.min(values[field.name].length, 20))
                    : values[field.name] || <Text color={COLORS.dim}>{field.placeholder}</Text>}
                </Text>
              )}
            </Box>
          </Box>
        ))}
      </Box>
    </Box>
  );
}
