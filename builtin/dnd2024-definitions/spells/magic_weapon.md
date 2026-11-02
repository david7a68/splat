---
splat_schema: 1
splat_id: 2f5179e0-f2fa-5e40-ab0b-947f031eb8e2
splat_kind: spell
splat_key: dnd2024:spells/magic_weapon
splat_name: Magic Weapon
splat_origin:
  key: dnd2024:reference/spell/magic_weapon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 146, 147
  url: https://www.dndbeyond.com/srd
---

# Magic Weapon

## Source text

<!-- SRD page 146 -->

Level 2 Transmutation (Paladin, Ranger, Sorcerer, Wizard)\
Casting Time: Bonus Action\
Range: Touch\
Components: V, S\
Duration: 1 hour

<!-- SRD page 147 -->

You touch a nonmagical weapon. Until the spell ends, that weapon becomes a magic weapon with a +1 bonus to attack rolls and damage rolls. The spell ends early if you cast it again.

Using a Higher-Level Spell Slot. The bonus increases to +2 with a level 3–5 spell slot. The bonus increases to +3 with a level 6+ spell slot.

Original layout: [[../../srd-5.2.1/pages/146|page 146]], [[../../srd-5.2.1/pages/147|page 147]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["paladin", "ranger", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` Bonus Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/magic_weapon ` |
| ` implementation ` | ` dnd2024:spell_magic_weapon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` magic_weapon ` | ` true ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` weapon ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` touch ` | ` true ` |
| ` magical ` | ` false ` |

#### Attack and damage bonus by slot

<!-- splat:yaml path=splat_rule/parameters/attack_and_damage_bonus_by_slot -->

```yaml
'2': 1
'3': 2
'4': 2
'5': 2
'6': 3
'7': 3
'8': 3
'9': 3
```

#### End

<!-- splat:values path=splat_rule/parameters/end -->

| Field | Value |
| --- | --- |
| ` event ` | ` caster.recasts_same_spell ` |
