---
splat_schema: 1
splat_id: e41205a0-0b7b-56f7-aed3-c79b3b489277
splat_kind: rule
splat_key: dnd2024:rules/weapons_mastery_properties
splat_name: Mastery Properties
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
---

# Mastery Properties

## Source text

<!-- SRD page 90 -->

Each weapon has a mastery property, which is us-\
able only by a character who has a feature, such as\
Weapon Mastery, that unlocks the property for the\
character. The properties are defined below.\
iMprovisEd wEapons\
If you use an object—such as a table leg, frying pan,\
or bottle—as a makeshift weapon, see “Improvised\
Weapons” in “Rules Glossary.” Also see those rules\
if you wield a weapon in an unusual way, such as using\
a Ranged weapon to make a melee attack.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_mastery_properties ` |
| ` implementation ` | ` dnd2024:rules_weapons_mastery_properties ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` mastery_requires_unlocking_feature ` | ` true ` |
| ` improvised_cases ` | ` ["makeshift_object_weapon", "weapon_used_unusually"] ` |

#### Improvised weapon rules

<!-- splat:values path=splat_rule/parameters/improvised_weapon_rules -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/improvised_weapons ` |
