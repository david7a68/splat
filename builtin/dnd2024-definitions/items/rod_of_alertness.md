---
splat_schema: 1
splat_id: c112e39d-daba-5b50-88dc-43ce13020c4a
splat_kind: item
splat_key: dnd2024:items/rod_of_alertness
splat_name: Rod of Alertness
splat_origin:
  key: dnd2024:reference/magic_item/rod_of_alertness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 241
  url: https://www.dndbeyond.com/srd
---

# Rod of Alertness

## Source text

<!-- SRD page 241 -->

Rod, Very Rare (Requires Attunement)\
This rod has the following properties.\
Alertness. While holding the rod, you have\
Advantage on Wisdom (Perception) checks and on\
Initiative rolls.\
Spells. While holding the rod, you can cast the fol-\
lowing spells from it:\
•  Detect Evil and Good\
•  Detect Magic\
•  Detect Poison and Disease\
•  See Invisibility\
Protective Aura. As a Magic action, you can plant\
the haft end of the rod in the ground, whereupon\
the rod’s head sheds Bright Light in a 60-foot radius\
and Dim Light for an additional 60 feet. While in\
that Bright Light, you and your allies gain a +1\
bonus to Armor Class and saving throws and can\
sense the location of any Invisible creature that is\
also in the Bright Light.\
The rod’s head stops glowing and the effect ends\
after 10 minutes or when a creature takes a Magic\
action to pull the rod from the ground. Once used,\
this property can’t be used again until the next\
dawn.\

Original layout: [[../../srd-5.2.1/pages/241|page 241]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` rod ` |
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
| ` mechanic ` | ` dnd2024:items/rod_of_alertness ` |
| ` implementation ` | ` dnd2024:item_rod_of_alertness ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attunement_required ` | ` true ` |

#### While held

<!-- splat:values path=splat_rule/parameters/while_held -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` ["wisdom_perception", "initiative"] ` |

##### Spells

<!-- splat:table
path: splat_rule/parameters/while_held/spells
key: Spell
order:
- detect_evil_and_good
- detect_magic
- detect_poison_and_disease
- see_invisibility
columns:
  Spell:
    path: spell
    type: reference
  Usage:
    path: usage
    type: symbol
-->

| Spell | Usage |
| --- | --- |
| [Detect Evil and Good](<../spells/detect_evil_and_good.md> "dnd2024:spells/detect_evil_and_good") | ` At will ` |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` |
| [Detect Poison and Disease](<../spells/detect_poison_and_disease.md> "dnd2024:spells/detect_poison_and_disease") | ` At will ` |
| [See Invisibility](<../spells/see_invisibility.md> "dnd2024:spells/see_invisibility") | ` At will ` |

#### Protective aura

<!-- splat:yaml path=splat_rule/parameters/protective_aura -->

```yaml
action: magic
plant_haft_in_ground: true
light:
  bright_radius_ft: 60
  additional_dim_ft: 60
beneficiaries:
  relation:
  - self
  - ally
  within: rod_bright_light
ac_bonus: 1
save_bonus: 1
sense_invisible_creature_location:
  creature_within: rod_bright_light
duration:
  amount: 10
  unit: minutes
ends_on:
  action: magic
  actor: any_creature
  operation: pull_rod_from_ground
uses:
  maximum: 1
  recovery_event: time.dawn
```
