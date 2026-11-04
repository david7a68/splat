---
splat_schema: 2
splat_id: 56db16e5-094d-575f-a106-16569982da7c
splat_kind: rule
splat_key: dnd2024:glossary/magical_effect
splat_name: Magical Effect
splat_origin: dnd2024:reference/glossary/magical_effect
splat_sources:
- SRD 5.2.1, pp. 185, https://www.dndbeyond.com/srd
---

# Magical Effect

## Source text

<!-- SRD page 185 -->

An effect is magical if it is created by a spell, a magic item, or a phenomenon that a rule labels as magical.

Original layout: [[../../srd-5.2.1/pages/185|page 185]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/magical_effect ` |
| ` implementation ` | ` dnd2024:glossary_magical_effect ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` magical_when_any ` | ` ["created_by_spell", "created_by_magic_item", "phenomenon_explicitly_labeled_magical"] ` |
