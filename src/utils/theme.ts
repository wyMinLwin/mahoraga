export const COLORS = {
  primary: '#D4AF37',
  secondary: '#B0B0B0',
  muted: '#666666',
  success: '#22C55E',
  error: '#EF4444',
  white: '#FFFFFF',
  dim: '#888888',
} as const;

export type ColorKey = keyof typeof COLORS;
