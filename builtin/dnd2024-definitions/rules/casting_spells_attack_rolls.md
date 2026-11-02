---
splat_schema: 1
splat_id: 73cb3725-5cb6-53b0-bce4-ffb1751a0b35
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_attack_rolls
splat_name: Attack Rolls
splat_origin:
  key: dnd2024:reference/pages/106
splat_sources:
- title: SRD 5.2.1
  locator: pp. 106
  url: https://www.dndbeyond.com/srd
---

# Attack Rolls

## Source text

<!-- SRD page 106 -->

Some spells require the caster to make an attack roll to determine whether the spell hits a target. Here’s how to calculate the attack modifier for your spells:

Spell attack modifier = your spellcasting ability modifier + your Proficiency Bonus

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_attack_rolls ` |
| ` implementation ` | ` dnd2024:rules_casting_spells_attack_rolls ` |

### Parameters

#### Spell attack modifier

<!-- splat:values path=splat_rule/parameters/spell_attack_modifier -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

##### Args

<!-- splat:table
path: splat_rule/parameters/spell_attack_modifier/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Var:
    path: var
    type: string
-->

| Key | Var |
| --- | --- |
| ` row_1 ` | ` caster.spellcasting_ability_modifier ` |
| ` row_2 ` | ` caster.proficiency_bonus ` |
