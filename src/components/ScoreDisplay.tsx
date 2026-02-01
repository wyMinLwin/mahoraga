import React from 'react';
import { Box, Text } from 'ink';
import { COLORS } from '../utils/theme.js';

interface ScoreDisplayProps {
  score: number;
}

export function ScoreDisplay({ score }: ScoreDisplayProps) {
  const barWidth = 20;
  const filledWidth = Math.round(score * barWidth);
  const emptyWidth = barWidth - filledWidth;

  const filledBar = '\u2588'.repeat(filledWidth);
  const emptyBar = '\u2591'.repeat(emptyWidth);

  // Color based on score
  const scoreColor =
    score >= 0.8 ? COLORS.success :
    score >= 0.5 ? COLORS.primary :
    COLORS.error;

  return (
    <Box marginY={1}>
      <Text>Score: </Text>
      <Text color={scoreColor}>{filledBar}</Text>
      <Text color={COLORS.muted}>{emptyBar}</Text>
      <Text color={scoreColor}> {score.toFixed(2)}</Text>
    </Box>
  );
}
