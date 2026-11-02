---
splat_schema: 1
splat_id: a8912eaa-95b4-5ee5-b49d-2f254ea1df9d
splat_kind: rule
splat_key: dnd2024:class_features/oath_of_devotion_level_15_smite_of_protection
splat_name: 'Level 15: Smite of Protection'
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
---

# Level 15: Smite of Protection

## Source text

<!-- SRD page 57 -->

Your magical smite now radiates protective energy. Whenever you cast Divine Smite, you and your allies have Half Cover while in your Aura of Protection. The aura has this benefit until the start of your next turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/oath_of_devotion_level_15_smite_of_protection ` |
| ` implementation ` | ` dnd2024:class_features_oath_of_devotion_level_15_smite_of_protection ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` aura ` | ` paladin.aura_of_protection ` |
| ` targets ` | ` ["actor", "allies"] ` |
| ` end ` | ` actor.next_turn.start ` |

#### Trigger

<!-- splat:yaml path=splat_rule/parameters/trigger -->

```yaml
spell_cast:
  key: dnd2024:spells/divine_smite
caster: actor
```

#### Effect

<!-- splat:values path=splat_rule/parameters/effect -->

| Field | Value |
| --- | --- |
| ` cover ` | ` half ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` minimum_level ` | ` 15 ` |
