---
splat_schema: 1
splat_id: fe74f7f0-7506-55ab-ab1b-6f4b064bbd09
splat_kind: rule_group
splat_key: dnd2024:species/dragonborn
splat_name: Dragonborn
splat_origin:
  key: dnd2024:reference/pages/084
splat_sources:
- title: SRD 5.2.1
  locator: pp. 84
  url: https://www.dndbeyond.com/srd
---

# Dragonborn

## Source text

<!-- SRD page 84 -->

Creature Type: Humanoid\
Size: Medium (about 5–7 feet tall)\
Speed: 30 feet\
As a Dragonborn, you have these special traits.\
Draconic Ancestry. Your lineage stems from a\
dragon progenitor. Choose the kind of dragon from\
the Draconic Ancestors table. Your choice affects\
your Breath Weapon and Damage Resistance traits\
as well as your appearance.\
Draconic Ancestors\
Dragon Damage Type Dragon Damage Type\
Black Acid Gold Fire\
Blue Lightning Green Poison\
Brass Fire Red Fire\
Bronze Lightning Silver Cold\
Copper Acid White Cold\
Breath Weapon. When you take the Attack action\
on your turn, you can replace one of your attacks\
with an exhalation of magical energy in either a\
15-foot Cone or a 30-foot Line that is 5 feet wide\
(choose the shape each time). Each creature in that\
area must make a Dexterity saving throw (DC 8 plus\
your Constitution modifier and Proficiency Bonus).\
On a failed save, a creature takes 1d10 damage of\
the type determined by your Draconic Ancestry\
trait. On a successful save, a creature takes half as\
much damage. This damage increases by 1d10 when\
you reach character levels 5 (2d10), 11 (3d10), and\
17 (4d10).\
You can use this Breath Weapon a number of\
times equal to your Proficiency Bonus, and you re-\
gain all expended uses when you finish a Long Rest.\
Damage Resistance. You have Resistance to the\
damage type determined by your Draconic Ances-\
try trait.\
Darkvision. You have Darkvision with a range of\
60 feet.\
Draconic Flight. When you reach character level\
5, you can channel draconic magic to give yourself\
temporary flight. As a Bonus Action, you sprout\
spectral wings on your back that last for 10 minutes\
or until you retract the wings (no action required)\
or have the Incapacitated condition. During that\
time, you have a Fly Speed equal to your Speed. Your\
wings appear to be made of the same energy as your\
Breath Weapon. Once you use this trait, you can’t\
use it again until you finish a Long Rest.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` species ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:species/dragonborn ` |
| ` implementation ` | ` dnd2024:species_dragonborn ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` creature_type ` | ` humanoid ` |
| ` size ` | ` medium ` |
| ` speed_feet ` | ` 30 ` |
| ` damage_resistance ` | ` ancestry_damage_type ` |
| ` darkvision_feet ` | ` 60 ` |

#### Height feet approximate

<!-- splat:values path=splat_rule/parameters/height_feet_approximate -->

| Field | Value |
| --- | --- |
| ` minimum ` | ` 5 ` |
| ` maximum ` | ` 7 ` |

#### Ancestry choice

<!-- splat:yaml path=splat_rule/parameters/ancestry_choice -->

```yaml
count: 1
options:
- dragon: black
  damage_type: acid
- dragon: blue
  damage_type: lightning
- dragon: brass
  damage_type: fire
- dragon: bronze
  damage_type: lightning
- dragon: copper
  damage_type: acid
- dragon: gold
  damage_type: fire
- dragon: green
  damage_type: poison
- dragon: red
  damage_type: fire
- dragon: silver
  damage_type: cold
- dragon: white
  damage_type: cold
```

#### Breath weapon

<!-- splat:yaml path=splat_rule/parameters/breath_weapon -->

```yaml
action: replace_one_attack_in_own_turn_attack_action
magical: true
area_choice_each_use:
- shape: cone
  length_feet: 15
- shape: line
  length_feet: 30
  width_feet: 5
targets: all_creatures_in_area
save:
  ability: dexterity
  dc:
    op: add
    args:
    - 8
    - stat: abilities.constitution.modifier
    - &id001
      stat: proficiency_bonus
  success_damage_multiplier: 0.5
damage_type: ancestry_damage_type
dice_by_character_level:
  '1':
    dice:
      count: 1
      sides: 10
  '5':
    dice:
      count: 2
      sides: 10
  '11':
    dice:
      count: 3
      sides: 10
  '17':
    dice:
      count: 4
      sides: 10
uses:
  maximum: *id001
  recover:
    event: rest.long_rest_completed
    amount: all
```

#### Draconic flight

<!-- splat:yaml path=splat_rule/parameters/draconic_flight -->

```yaml
minimum_character_level: 5
action: bonus_action
fly_speed:
  stat: speed
duration:
  minutes: 10
ends_on_condition: incapacitated
retract_action: none
uses:
  maximum: 1
  recover:
    event: rest.long_rest_completed
    amount: all
```
