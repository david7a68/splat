---
splat_schema: 1
splat_id: b30e8d90-84cb-5cf9-a633-b10541f71465
splat_kind: item
splat_key: dnd2024:items/dragon_orb
splat_name: Dragon Orb
splat_origin:
  key: dnd2024:reference/magic_item/dragon_orb
splat_sources:
- title: SRD 5.2.1
  locator: pp. 218
  url: https://www.dndbeyond.com/srd
---

# Dragon Orb

## Source text

<!-- SRD page 218 -->

Wondrous Item, Artifact (Requires Attunement)\
An orb is an etched crystal globe about 10 inches in\
diameter. When used, it grows to about 20 inches in\
diameter, and mist swirls inside it.\
While attuned to an orb, you can take a Magic\
action to peer into the orb’s depths. You must then\
make a DC 15 Charisma saving throw. On a success-\
ful save, you control the orb for as long as you re-\
main attuned to it. On a failed save, the orb imposes\
the Charmed condition on you for as long as you\
remain attuned to it.\
While you are Charmed by the orb, you can’t\
voluntarily end your Attunement to it, and the orb\
casts Suggestion on you at will (save DC 18), urging\
you to work toward the evil ends it desires. The\
dragon essence within the orb might want many\
things: the annihilation of a particular society or or-\
ganization, freedom from the orb, to spread suffer-\
ing in the world, to advance the worship of Tiamat,\
or something else the GM decides.\
Spells. The orb has 7 charges and regains 1d4 + 3\
expended charges daily at dawn. If you control the\
orb, you can cast one of the spells on the following\
table from it. The table indicates how many charges\
you must expend to cast the spell.\
Spell Charge\
Cost\
Cure Wounds (level 9 version) 4\
Daylight 1\
Death Ward 2\
Detect Magic 0\
Scrying (save DC 18) 3\
Call Dragons. While you control the orb, you\
can take a Magic action to cause the orb to issue a\
telepathic call that extends in all directions for 40\
miles. Chromatic dragons in range feel compelled\
to come to the orb as soon as possible by the most\
direct route. Dragon deities such as Tiamat are un-\
affected by this call. Chromatic dragons drawn to\
the orb might be Hostile toward you for compelling\
them against their will. Once you have used this\
property, it can’t be used again for 1 hour.\
Destroying an Orb. A Dragon Orb has AC 20 and is\
destroyed if it takes damage from a +3 Weapon or a\
Disintegrate spell. Nothing else can harm it.\

Original layout: [[../../srd-5.2.1/pages/218|page 218]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` artifact ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/dragon_orb ` |
| ` implementation ` | ` dnd2024:item_dragon_orb ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` spells_require_control ` | ` true ` |
| ` attunement_required ` | ` true ` |

#### Diameter inches

<!-- splat:values path=splat_rule/parameters/diameter_inches -->

| Field | Value |
| --- | --- |
| ` idle ` | ` 10 ` |
| ` used ` | ` 20 ` |

#### Control

<!-- splat:yaml path=splat_rule/parameters/control -->

```yaml
action: magic
peer_into_depths: true
save:
  ability: charisma
  dc: 15
  failure:
    condition: charmed
    source: orb
    duration_end: attunement_ends
    voluntary_end_attunement: false
    orb_spells:
    - spell:
        key: dnd2024:spells/suggestion
      save_dc: 18
      usage: at_will
      purpose:
        choice: gm
        category: evil_dragon_essence_objective
  success:
    control: true
    duration_end: attunement_ends
```

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
initial: 7
maximum: 7
recovery:
  event: time.dawn
  amount:
    roll:
      count: 1
      sides: 4
      add: 3
  cap_at_maximum: true
```

#### Spells

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- cure_wounds
- daylight
- death_ward
- detect_magic
- scrying
columns:
  Spell:
    path: spell
    type: reference
  Level:
    path: level
    type: integer
    optional: true
  Charge cost:
    path: charge_cost
    type: integer
  Save dc:
    path: save_dc
    type: integer
    optional: true
-->

| Spell | Level | Charge cost | Save dc |
| --- | --- | --- | --- |
| [Cure Wounds](<../spells/cure_wounds.md> "dnd2024:spells/cure_wounds") | ` 9 ` | ` 4 ` |  |
| [Daylight](<../spells/daylight.md> "dnd2024:spells/daylight") |  | ` 1 ` |  |
| [Death Ward](<../spells/death_ward.md> "dnd2024:spells/death_ward") |  | ` 2 ` |  |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") |  | ` 0 ` |  |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") |  | ` 3 ` | ` 18 ` |

#### Call dragons

<!-- splat:yaml path=splat_rule/parameters/call_dragons -->

```yaml
action: magic
requires_control: true
telepathic_range_miles: 40
targets: chromatic_dragons
excluded: dragon_deities
movement: as_soon_as_possible_most_direct_route
attitude:
  choice: gm
  hostile_permitted: true
cooldown:
  amount: 1
  unit: hours
```

#### Object defense

<!-- splat:values path=splat_rule/parameters/object_defense -->

| Field | Value |
| --- | --- |
| ` ac ` | ` 20 ` |
| ` immune_to_all_other_harm ` | ` true ` |

##### Damage destroys from

<!-- splat:table
path: splat_rule/parameters/object_defense/damage_destroys_from
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Weapon magic bonus:
    path: weapon_magic_bonus
    type: integer
    optional: true
  Spell:
    path: spell
    type: reference
    optional: true
-->

| Key | Weapon magic bonus | Spell |
| --- | --- | --- |
| ` row_1 ` | ` 3 ` |  |
| ` row_2 ` |  | [Disintegrate](<../spells/disintegrate.md> "dnd2024:spells/disintegrate") |
