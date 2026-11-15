// Types for the viewer's use of card.js (242: the player view's printed cards).
export const esc: (s: unknown) => string;
export const tierOf: (tier: number) => 1 | 2;
export const frameOf: (card: { kind: string; tier: number }) => string;
export function inset(layout: unknown): unknown;
export function fitCards(root?: Document | ShadowRoot | Element): Promise<unknown[]>;
export function renderBack(tier: number, layout: unknown, keep: number): string;
export function renderCard(card: unknown, layout: unknown, frameUrl: string, fit?: unknown, artUrl?: string): string;
