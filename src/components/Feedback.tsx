import React from 'react';
import { Box, Text } from 'ink';
import { COLORS } from '../utils/theme.js';
import type { AnalysisResult } from '../types.js';

interface FeedbackProps {
  result: AnalysisResult;
}

export function Feedback({ result }: FeedbackProps) {
  return (
    <Box flexDirection="column" marginY={1}>
      {result.improvements.length > 0 && (
        <Box flexDirection="column" marginBottom={1}>
          <Text bold color={COLORS.primary}>Areas to Improve:</Text>
          {result.improvements.map((improvement, index) => (
            <Text key={index} color={COLORS.white}>
              {'  - '}{improvement}
            </Text>
          ))}
        </Box>
      )}

      {result.unclearParts.length > 0 && (
        <Box flexDirection="column">
          <Text bold color={COLORS.primary}>Unclear Parts:</Text>
          {result.unclearParts.map((part, index) => (
            <Text key={index} color={COLORS.white}>
              {'  - '}{part}
            </Text>
          ))}
        </Box>
      )}

      {result.improvements.length === 0 && result.unclearParts.length === 0 && (
        <Text color={COLORS.success}>Your prompt is clear and well-structured!</Text>
      )}
    </Box>
  );
}
