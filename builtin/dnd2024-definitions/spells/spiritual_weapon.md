---
splat_schema: 1
splat_id: f9a9d4eb-f7e1-58a8-bc34-0dbfcd22c233
splat_kind: spell
splat_key: dnd2024:spells/spiritual_weapon
splat_name: Spiritual Weapon
splat_origin:
  key: dnd2024:reference/spell/spiritual_weapon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 165
  url: https://www.dndbeyond.com/srd
---

# Spiritual Weapon

## Source text

<!-- SRD page 165 -->

Level 2 Evocation (Cleric)\
Casting Time: Bonus Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
You create a floating, spectral force that resembles\
a weapon of your choice and lasts for the duration.\
The force appears within range in a space of your\
choice, and you can immediately make one melee\
spell attack against one creature within 5 feet of the\
force. On a hit, the target takes Force damage equal\
to 1d8 plus your spellcasting ability modifier.\
As a Bonus Action on your later turns, you can\
move the force up to 20 feet and repeat the attack\
against a creature within 5 feet of it.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for every slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/165|page 165]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["cleric"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Bonus Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/spiritual_weapon ` |
| ` implementation ` | ` dnd2024:spell_spiritual_weapon ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` force_location ` |
| ` within_range ` | ` true ` |
| ` choice ` | ` caster ` |

#### Force

<!-- splat:values path=splat_rule/parameters/force -->

| Field | Value |
| --- | --- |
| ` floating ` | ` true ` |
| ` spectral ` | ` true ` |
| ` weapon_appearance ` | ` caster_choice ` |
| ` duration ` | ` spell_duration ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
initial: optional_immediately_on_cast
kind: melee_spell
target: one_creature
within_force_feet: 5
hit:
  damage:
    dice: 1d8
    add:
      stat: caster_spellcasting_ability_modifier
    type: force
```

#### Repeat

<!-- splat:values path=splat_rule/parameters/repeat -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` timing ` | ` later_caster_turns ` |
| ` movement_max_feet ` | ` 20 ` |
| ` repeat_attack ` | ` true ` |

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 2 ` |
| ` damage_per_level ` | ` 1d8 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```
