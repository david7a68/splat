---
splat_schema: 1
splat_id: e65baf9f-5f63-571a-8be6-01a7317efcf6
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_6_aura_of_protection
splat_name: 'Level 6: Aura of Protection'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
---

# Level 6: Aura of Protection

## Source text

<!-- SRD page 55 -->

You radiate a protective, unseeable aura in a 10-foot Emanation that originates from you. The aura is inactive while you have the Incapacitated condition.

You and your allies in the aura gain a bonus to saving throws equal to your Charisma modifier (minimum bonus of +1).

If another Paladin is present, a creature can benefit from only one Aura of Protection at a time; the creature chooses which aura while in them.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_6_aura_of_protection ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_6_aura_of_protection ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` targets ` | ` ["self", "allies_in_aura"] ` |

#### Aura

<!-- splat:values path=splat_rule/parameters/aura -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 10 ` |
| ` origin ` | ` self ` |
| ` visible ` | ` false ` |
| ` inactive_when_condition ` | ` incapacitated ` |

#### Saving throw bonus

<!-- splat:yaml path=splat_rule/parameters/saving_throw_bonus -->

```yaml
op: max
args:
- 1
- stat: abilities.charisma.modifier
```

#### Stacking

<!-- splat:values path=splat_rule/parameters/stacking -->

| Field | Value |
| --- | --- |
| ` maximum_aura_of_protection_benefits ` | ` 1 ` |
| ` selector ` | ` affected_creature ` |
