---
splat_schema: 1
splat_id: 233537fe-fc1f-54cb-8f1c-a0eb4d5f49ed
splat_kind: rule
splat_key: dnd2024:class_features/life_domain_level_6_blessed_healer
splat_name: 'Level 6: Blessed Healer'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
---

# Level 6: Blessed Healer

## Source text

<!-- SRD page 40 -->

The healing spells you cast on others heal you as well. Immediately after you cast a spell with a spell slot that restores Hit Points to one or more creatures other than yourself, you regain Hit Points equal to 2 plus the spell slot’s level.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/life_domain_level_6_blessed_healer ` |
| ` implementation ` | ` dnd2024:class_features_life_domain_level_6_blessed_healer ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` timing ` | ` immediately_after_cast ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` spell_cast ` | ` true ` |
| ` expended_spell_slot ` | ` true ` |
| ` restores_hit_points_to_other_creature ` | ` true ` |

#### Heal self

<!-- splat:yaml path=splat_rule/parameters/heal_self -->

```yaml
op: add
args:
- 2
- stat: spell.slot_level
```
