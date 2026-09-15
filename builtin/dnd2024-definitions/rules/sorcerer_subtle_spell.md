---
splat_schema: 1
splat_id: 97069f88-f383-51ec-9e25-eb4943a439fe
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_subtle_spell
splat_name: Subtle Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
---

# Subtle Spell

## Source text

<!-- SRD page 67 -->

Cost: 1 Sorcery Point\
When you cast a spell, you can spend 1 Sorcery\
Point to cast it without any Verbal, Somatic, or Ma-\
terial components, except Material components that\
are consumed by the spell or that have a cost speci-\
fied in the spell.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_subtle_spell ` |
| ` implementation ` | ` dnd2024:rules_sorcerer_subtle_spell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` spell.cast_pending ` |
| ` omit_components ` | ` ["verbal", "somatic", "material_without_cost_or_consumption"] ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` resource ` | ` sorcery_points ` |
| ` amount ` | ` 1 ` |

#### Retain material if

<!-- splat:values path=splat_rule/parameters/retain_material_if -->

| Field | Value |
| --- | --- |
| ` any ` | ` ["consumed", "specified_cost"] ` |
