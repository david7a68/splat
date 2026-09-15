---
splat_schema: 1
splat_id: ac7030aa-bbb1-54a0-8d11-9dcc37d6c5fe
splat_kind: item
splat_key: dnd2024:items/shield_of_the_cavalier
splat_name: Shield of the Cavalier
splat_origin:
  key: dnd2024:reference/magic_item/shield_of_the_cavalier
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
---

# Shield of the Cavalier

## Source text

<!-- SRD page 243 -->

Armor (Shield), Very Rare (Requires Attunement)\
While holding this Shield, you have a +2 bonus to\
Armor Class. This bonus is in addition to the Shield’s\
normal bonus to AC.\
The Shield has the following additional properties\
that you can use while holding it.\
Forceful Bash. When you take the Attack action,\
you can make one of the attack rolls using the Shield\
against a target within 5 feet of yourself. Apply your\
Proficiency Bonus and Strength modifier to the at-\
tack roll. On a hit, the Shield deals Force damage to\
the target equal to 2d6 + 2 plus your Strength mod-\
ifier, and if the target is a creature, you can push\
it up to 10 feet directly away from yourself. If the\
creature is your size or smaller, you can also knock\
it down, giving it the Prone condition.\
Protective Field. As a Reaction, when you or an\
ally you can see within 5 feet of you is targeted by\
an attack or makes a saving throw against an area\
of effect, you can use the Shield to create an immo-\
bile 5-foot Emanation originating from you. When\
the Emanation appears, any creatures or objects not\
fully contained within it are pushed into the nearest\
unoccupied spaces outside it. The attack or area of\
effect that triggered the Reaction has no effect on\
creatures and objects inside the Emanation, which\
lasts as long as you maintain Concentration, up to 1\
minute. Nothing can pass into or out of the Emana-\
tion. A creature or object inside the Emanation can’t\
be damaged by attacks or effects originating from\
outside, nor can a creature inside the Emanation\
damage anything outside it. Once this property is\
used, it can’t be used again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/243|page 243]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/shield_of_the_cavalier ` |
| ` implementation ` | ` dnd2024:item_shield_of_the_cavalier ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |

#### Armor class

<!-- splat:values path=splat_rule/parameters/armor_class -->

| Field | Value |
| --- | --- |
| ` bonus ` | ` 2 ` |
| ` stacks_with_normal_shield ` | ` true ` |

#### Forceful bash

<!-- splat:yaml path=splat_rule/parameters/forceful_bash -->

```yaml
trigger: attack_action
replaces: one_attack_roll
range_ft: 5
attack_bonus:
  sum:
  - value_from: proficiency_bonus
  - value_from: strength_modifier
on_hit:
  damage:
    roll:
      dice: 2
      sides: 6
      bonus: 2
    add: strength_modifier
    type: force
  if_creature:
    optional_push:
      maximum_ft: 10
      direction: directly_away_from_user
    optional_prone:
      maximum_size: user_size
```

#### Protective field

<!-- splat:yaml path=splat_rule/parameters/protective_field -->

```yaml
action: reaction
resource:
  uses: 1
  recovery: dawn
trigger:
  target:
    any_of:
    - self
    - ally: true
      seen: true
      range_ft: 5
  event:
  - targeted_by_attack
  - saving_throw_against_area_effect
area:
  shape: emanation
  radius_ft: 5
  origin: user_activation_location
  immobile: true
duration:
  value: 1
  unit: minute
concentration: true
appearance:
  partially_contained_creatures_and_objects: push_to_nearest_unoccupied_space_outside
triggering_effect: no_effect_on_contents
boundary:
  passage_in: false
  passage_out: false
  outside_damage_to_inside: false
  inside_creature_damage_to_outside: false
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
