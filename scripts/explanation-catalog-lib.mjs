/**
 * Shared Experience catalog parsing and key resolution (Sprint 132).
 * Used by sync/verify scripts — mirrors kernel + UI resolver semantics.
 */

export function parseGeneratedCatalogTs(source) {
  const marker = "export default ";
  const start = source.indexOf(marker);
  if (start < 0) {
    throw new Error("generated catalog must contain 'export default'");
  }
  const jsonStart = start + marker.length;
  const jsonEnd = source.lastIndexOf(" as const");
  if (jsonEnd < jsonStart) {
    throw new Error("generated catalog must end with ' as const'");
  }
  return JSON.parse(source.slice(jsonStart, jsonEnd));
}

export function stableStringify(value) {
  return `${JSON.stringify(value, null, 2)}\n`;
}

/**
 * Resolve explanation_key using catalog metadata only.
 * Order: exact → prefix suffix → prefix pattern → prefix fallback → unknown (null).
 */
export function lookupCatalogEntry(catalog, key) {
  if (catalog.exact?.[key]) {
    return { entry: catalog.exact[key], tier: "exact" };
  }
  for (const rule of catalog.prefix_rules ?? []) {
    if (!key.startsWith(rule.prefix)) continue;
    const rest = key.slice(rule.prefix.length);
    if (rule.suffixes?.[rest]) {
      return { entry: rule.suffixes[rest], tier: "prefix_suffix" };
    }
    for (const pattern of rule.patterns ?? []) {
      if (pattern.starts_with && rest.startsWith(pattern.starts_with)) {
        return {
          entry: { title: pattern.title, description: pattern.description },
          tier: "prefix_pattern",
        };
      }
    }
    if (rule.fallback) {
      return { entry: rule.fallback, tier: "prefix_fallback" };
    }
  }
  return null;
}

export function fallbackTitleForSignal(catalog, signal) {
  const template =
    catalog.resolution?.unknown?.signal_title_fallback_template ??
    "Attention signal '{signal}' needs attention";
  if (catalog.signal_titles?.[signal]) {
    return catalog.signal_titles[signal];
  }
  return template.replaceAll("{signal}", signal);
}

export function unknownDescription(catalog, explanationKey, signal, source, weight) {
  const template =
    catalog.resolution?.unknown?.description_template ??
    "No Experience translation for '{explanation_key}' yet (signal {signal} from {source}, weight {weight}).";
  return template
    .replaceAll("{explanation_key}", explanationKey)
    .replaceAll("{signal}", signal)
    .replaceAll("{source}", source)
    .replaceAll("{weight}", String(weight));
}

export function resolveFixture(catalog, fixture) {
  const looked = lookupCatalogEntry(catalog, fixture.key);
  if (looked) {
    return {
      title: looked.entry.title,
      description: looked.entry.description,
      known: true,
    };
  }
  return {
    title: fallbackTitleForSignal(catalog, fixture.signal),
    description: unknownDescription(
      catalog,
      fixture.key,
      fixture.signal,
      fixture.source,
      fixture.weight,
    ),
    known: false,
  };
}
