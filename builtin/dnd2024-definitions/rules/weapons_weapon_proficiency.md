---
splat_schema: 1
splat_id: 186e1d16-7b5f-5615-9830-e45b15865420
splat_kind: rule
splat_key: dnd2024:rules/weapons_weapon_proficiency
splat_name: Weapon Proficiency
splat_origin:
  key: dnd2024:reference/pages/089
splat_sources:
- title: SRD 5.2.1
  locator: pp. 89
  url: https://www.dndbeyond.com/srd
---

# Weapon Proficiency

## Source text

<!-- SRD page 89 -->

Anyone can wield a weapon, but you must have proficiency with it to add your Proficiency Bonus to an attack roll you make with it. A player character’s features can provide weapon proficiencies. A monster is proficient with any weapon in its stat block.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_weapon_proficiency ` |
| ` implementation ` | ` dnd2024:rules_weapons_weapon_proficiency ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` wield_requires_proficiency ` | ` false ` |

#### Attack proficiency bonus

<!-- splat:values path=splat_rule/parameters/attack_proficiency_bonus -->

| Field | Value |
| --- | --- |
| ` requires_weapon_proficiency ` | ` true ` |

#### Monster proficiency

<!-- splat:values path=splat_rule/parameters/monster_proficiency -->

| Field | Value |
| --- | --- |
| ` all_stat_block_weapons ` | ` true ` |
