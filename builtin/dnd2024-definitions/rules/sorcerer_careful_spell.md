---
splat_schema: 1
splat_id: 3e97a3ad-aafe-501f-bfa2-a24ce32f01c1
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_careful_spell
splat_name: Careful Spell
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66, 67
  url: https://www.dndbeyond.com/srd
---

# Careful Spell

## Source text

<!-- SRD page 66 -->

Cost: 1 Sorcery Point

When you cast a spell that forces other creatures to make a saving throw, you can protect some of those creatures from the spell’s full force. To do so, spend 1 Sorcery Point and choose a number of those creatures up to your Charisma modifier (minimum of one creature). A chosen creature automatically succeeds on its saving throw against the spell, and it takes no damage if it would normally take half damage on a successful save.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_careful_spell ` |
| ` implementation ` | ` dnd2024:rule_sorcerer_careful_spell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost_sorcery_points ` | ` 1 ` |
| ` trigger ` | ` casting_spell_forcing_other_creatures_to_save ` |
| ` selected_creatures ` | ` spell_save_targets ` |

#### Selected creatures maximum

<!-- splat:yaml path=splat_rule/parameters/selected_creatures_maximum -->

```yaml
op: max
args:
- 1
- variable: charisma_modifier
```

#### Selected effects

<!-- splat:values path=splat_rule/parameters/selected_effects -->

| Field | Value |
| --- | --- |
| ` saving_throw ` | ` automatic_success ` |
| ` half_damage_on_save_becomes ` | ` 0 ` |
