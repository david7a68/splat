---
splat_schema: 1
splat_id: 49f600ee-d551-5574-975d-3a01920ffa48
splat_kind: rule
splat_key: dnd2024:rules/weapons_graze
splat_name: Graze
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
---

# Graze

## Source text

<!-- SRD page 90 -->

If your attack roll with this weapon misses a creature, you can deal damage to that creature equal to the ability modifier you used to make the attack roll. This damage is the same type dealt by the weapon, and the damage can be increased only by increasing the ability modifier.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_graze ` |
| ` implementation ` | ` dnd2024:rule_weapons_graze ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` weapon_attack_misses_creature ` |

#### Optional damage

<!-- splat:values path=splat_rule/parameters/optional_damage -->

| Field | Value |
| --- | --- |
| ` amount ` | ` attack_ability_modifier ` |
| ` type ` | ` weapon_damage_type ` |
| ` increase_sources_allowed ` | ` ["ability_modifier_increase"] ` |
