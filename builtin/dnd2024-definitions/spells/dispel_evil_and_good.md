---
splat_schema: 1
splat_id: 98c98622-a138-5960-a9f2-d491794a5a79
splat_kind: spell
splat_key: dnd2024:spells/dispel_evil_and_good
splat_name: Dispel Evil and Good
splat_origin:
  key: dnd2024:reference/spell/dispel_evil_and_good
splat_sources:
- title: SRD 5.2.1
  locator: pp. 124
  url: https://www.dndbeyond.com/srd
---

# Dispel Evil and Good

## Source text

<!-- SRD page 124 -->

Level 5 Abjuration (Cleric, Paladin)\
Casting Time: Action\
Range: Self\
Components: V, S, M (powdered silver and iron)\
Duration: Concentration, up to 1 minute\
For the duration, Celestials, Elementals, Fey, Fiends,\
and Undead have Disadvantage on attack rolls\
against you. You can end the spell early by using ei-\
ther of the following special functions.\
Break Enchantment. As a Magic action, you touch\
a creature that is possessed by or has the Charmed\
or Frightened condition from one or more creatures\
of the types above. The target is no longer pos-\
sessed, Charmed, or Frightened by such creatures.\
Dismissal. As a Magic action, you target one crea-\
ture you can see within 5 feet of you that has one of\
the creature types above. The target must succeed\
on a Charisma saving throw or be sent back to its\
home plane if it isn’t there already. If they aren’t on\
their home plane, Undead are sent to the Shadow-\
fell, and Fey are sent to the Feywild.\

Original layout: [[../../srd-5.2.1/pages/124|page 124]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric", "paladin"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` powdered silver and iron ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/dispel_evil_and_good ` |
| ` implementation ` | ` dnd2024:spell_dispel_evil_and_good ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` creature_types ` | ` ["celestial", "elemental", "fey", "fiend", "undead"] ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Protection

<!-- splat:values path=splat_rule/parameters/protection -->

| Field | Value |
| --- | --- |
| ` disadvantage ` | ` attack_rolls ` |
| ` attacker_types ` | ` configured_types ` |
| ` target ` | ` caster ` |

#### Special functions

<!-- splat:values path=splat_rule/parameters/special_functions -->

| Field | Value |
| --- | --- |
| ` economy ` | ` magic_action ` |
| ` end_spell_on_use ` | ` true ` |
| ` choose_one ` | ` ["break_enchantment", "dismissal"] ` |

#### Break enchantment

<!-- splat:yaml path=splat_rule/parameters/break_enchantment -->

```yaml
target:
  kind: creature
  touch: true
remove_effects_from_configured_types:
- possession
- charmed
- frightened
```

#### Dismissal

<!-- splat:yaml path=splat_rule/parameters/dismissal -->

```yaml
target:
  kind: creature
  visible: true
  range_feet: 5
  types: configured_types
save:
  ability: charisma
  dc: spell_save_dc
failure:
  if_not_on_home_plane:
    transport_to: home_plane
    overrides:
      undead: shadowfell
      fey: feywild
  if_on_home_plane: unaffected
```
