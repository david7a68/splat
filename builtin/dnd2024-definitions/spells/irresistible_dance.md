---
splat_schema: 1
splat_id: 4113a55e-6142-5b7c-96b9-b896f5ecb073
splat_kind: spell
splat_key: dnd2024:spells/irresistible_dance
splat_name: Irresistible Dance
splat_origin:
  key: dnd2024:reference/spell/irresistible_dance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
---

# Irresistible Dance

## Source text

<!-- SRD page 143 -->

Level 6 Enchantment (Bard, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V\
Duration: Concentration, up to 1 minute\
One creature that you can see within range must\
make a Wisdom saving throw. On a successful save,\
the target dances comically until the end of its next\
turn, during which it must spend all its movement\
to dance in place.\
On a failed save, the target has the Charmed con-\
dition for the duration. While Charmed, the target\
dances comically, must use all its movement to\
dance in place, and has Disadvantage on Dexterity\
saving throws and attack rolls, and other creatures\
have Advantage on attack rolls against it. On each\
of its turns, the target can take an action to collect\
itself and repeat the save, ending the spell on itself\
on a success.\

Original layout: [[../../srd-5.2.1/pages/143|page 143]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "wizard"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/irresistible_dance ` |
| ` implementation ` | ` dnd2024:spell_irresistible_dance ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` visible ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  condition: charmed
  duration: spell_duration
  while_charmed:
    dance: true
    all_movement: dance_in_place
    disadvantage:
    - dexterity_saves
    - attack_rolls
    other_creatures_attack_advantage_against_target: true
  repeat_save:
    cost: action
    timing: each_target_turn
    success: end_spell_on_target
success:
  dance: true
  all_movement: dance_in_place
  until: target_next_turn_end
```
