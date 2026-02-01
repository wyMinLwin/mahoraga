import React from 'react';
import { Box, Text } from 'ink';
import { COLORS } from '../utils/theme.js';

export function ConfigRequired() {
  return (
    <Box flexDirection="column" marginY={1} paddingX={1}>
      <Text bold color={COLORS.error}>Configuration Required</Text>
      <Text> </Text>
      <Text color={COLORS.white}>To use Mahoraga, you need to configure your API settings.</Text>
      <Text> </Text>
      <Text color={COLORS.white}>Run <Text color={COLORS.primary} bold>/settings</Text> to configure:</Text>
      <Text color={COLORS.muted}>  - <Text color={COLORS.white}>url</Text>: Your Azure OpenAI endpoint</Text>
      <Text color={COLORS.muted}>  - <Text color={COLORS.white}>apiKey</Text>: Your API key</Text>
      <Text color={COLORS.muted}>  - <Text color={COLORS.white}>deployment</Text>: Your model deployment name</Text>
      <Text color={COLORS.muted}>  - <Text color={COLORS.white}>apiVersion</Text>: API version (e.g., 2024-02-15-preview)</Text>
    </Box>
  );
}
