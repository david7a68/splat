---
splat_schema: 1
splat_id: 28a9289f-6427-5a51-81b0-e6bab514e6cd
splat_kind: rule
splat_key: dnd2024:class_features/draconic_sorcery_level_18_dragon_companion
splat_name: 'Level 18: Dragon Companion'
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
---

# Level 18: Dragon Companion

## Source text

<!-- SRD page 70 -->

You can cast Summon Dragon without a Material component. You can also cast it once without a spell slot, and you regain the ability to cast it in this way when you finish a Long Rest.

Whenever you start casting the spell, you can modify it so that it doesn’t require Concentration. If you do so, the spell’s duration becomes 1 minute for that casting.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/draconic_sorcery_level_18_dragon_companion ` |
| ` implementation ` | ` dnd2024:class_features_draconic_sorcery_level_18_dragon_companion ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` material_component_required ` | ` false ` |

#### Spell

<!-- splat:values path=splat_rule/parameters/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/summon_dragon ` |

#### Free cast

<!-- splat:yaml path=splat_rule/parameters/free_cast -->

```yaml
spell_slot_cost: 0
uses:
  maximum: 1
  recover:
    event: rest.long_rest_completed
    amount: all
```

#### Optional cast modification

<!-- splat:yaml path=splat_rule/parameters/optional_cast_modification -->

```yaml
timing: casting_started
requires_concentration: false
duration:
  minutes: 1
```
