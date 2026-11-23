// #1582: Health-factor subscription webhook per account
export interface HealthWebhookConfig {
  url: string;
  threshold: number; // alert when health factor drops below this
}

const registry = new Map<string, HealthWebhookConfig>();

export function registerWebhook(account: string, config: HealthWebhookConfig) {
  registry.set(account, config);
}

export function removeWebhook(account: string) {
  registry.delete(account);
}

export async function notifyIfUnhealthy(account: string, healthFactor: number) {
  const cfg = registry.get(account);
  if (!cfg || healthFactor >= cfg.threshold) return;
  await fetch(cfg.url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ account, healthFactor, threshold: cfg.threshold }),
  }).catch(() => {});
}
