---
splat_schema: 1
splat_id: df0a375f-b01d-5b65-bd59-107144faadb7
splat_kind: item
splat_key: dnd2024:items/staff_of_the_magi
splat_name: Staff of the Magi
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_the_magi
splat_sources:
- title: SRD 5.2.1
  locator: pp. 246, 247
  url: https://www.dndbeyond.com/srd
---

# Staff of the Magi

## Source text

<!-- SRD page 246 -->

Staff, Legendary (Requires Attunement by a Sorcerer,\
Warlock, or Wizard)\
This staff has 50 charges and can be wielded as a\
magic Quarterstaff that grants a +2 bonus to attack\
rolls and damage rolls made with it. While you hold\
it, you gain a +2 bonus to spell attack rolls.\
Spell Absorption. While holding the staff, you\
have Advantage on saving throws against spells.\
In addition, you can take a Reaction when another\
creature casts a spell that targets only you. If you\
do, the staff absorbs the magic of the spell, cancel-\
ing its effect and gaining a number of charges equal\
to the absorbed spell’s level. However, if doing so\
brings the staff’s total number of charges above 50,\
the staff explodes as if you activated its Retributive\
Strike (see below).\
Spells. While holding the staff, you can cast one of\
the spells on the following table from it, using your\
spell save DC. The table indicates how many charges\
you must expend to cast the spell.\
Charge\
Spell Cost\
Arcane Lock 0\
Conjure Elemental 7\
Detect Magic 0\
Dispel Magic 3\
Enlarge/Reduce 0\
Fireball (level 7 version) 7\
Flaming Sphere 2\
Ice Storm 4\
Invisibility 2\
Knock 2\
Light 0\
Lightning Bolt (level 7 version) 7\

<!-- SRD page 247 -->

Charge\
Spell Cost\
Mage Hand 0\
Passwall 5\
Plane Shift 7\
Protection from Evil and Good 0\
Telekinesis 5\
Wall of Fire 4\
Web 2\
Regaining Charges. The staff regains 4d6 + 2 ex-\
pended charges daily at dawn. If you expend the last\
charge, roll 1d20. On a 20, the staff regains 1d12 + 1\
charges.\
Retributive Strike. You can take a Magic action to\
break the staff over your knee or against a solid sur-\
face. The staff is destroyed and releases its magic in\
an explosion that fills a 30-foot Emanation originat-\
ing from itself. You have a 50 percent chance to in-\
stantly travel to a random plane of existence, avoid-\
ing the explosion. If you fail to avoid the effect, you\
take Force damage equal to 16 times the number\
of charges in the staff. Each other creature in the\
area makes a DC 17 Dexterity saving throw. On a\
failed save, a creature takes Force damage equal to\
6 times the number of charges in the staff. On a suc-\
cessful save, a creature takes half as much damage.\

Original layout: [[../../srd-5.2.1/pages/246|page 246]], [[../../srd-5.2.1/pages/247|page 247]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` staff ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |
| ` attunement_requirement_text ` | ` a Sorcerer, Warlock, or Wizard ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_the_magi ` |
| ` implementation ` | ` dnd2024:item_staff_of_the_magi ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_roll_bonus ` | ` 2 ` |
| ` damage_roll_bonus ` | ` 2 ` |
| ` scope ` | ` attacks_with_source_item ` |
| ` spell_grants_require ` | ` held ` |
| ` spell_save_dc ` | ` user_spell_save_dc ` |

#### Weapon

<!-- splat:values path=splat_rule/parameters/weapon -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/quarterstaff ` |

#### While held

<!-- splat:values path=splat_rule/parameters/while_held -->

| Field | Value |
| --- | --- |
| ` spell_attack_bonus ` | ` 2 ` |
| ` save_advantage_against ` | ` ["spells"] ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 50
initial: 50
recovery:
  event: dawn
  amount:
    expression: 4d6+2
    scope: use
  cap: maximum
```

#### Spell grants

<!-- splat:table
path: splat_rule/parameters/spell_grants
key: Spell
order:
- arcane_lock
- conjure_elemental
- detect_magic
- dispel_magic
- enlarge_reduce
- fireball
- flaming_sphere
- ice_storm
- invisibility
- knock
- light
- lightning_bolt
- mage_hand
- passwall
- plane_shift
- protection_from_evil_and_good
- telekinesis
- wall_of_fire
- web
columns:
  Spell:
    path: spell
    type: reference
  Charge cost:
    path: charge_cost
    type: integer
  Casting time:
    path: casting_time
    type: symbol
  Method:
    path: method
    type: symbol
  Slot cost:
    path: slot_cost
    type: integer
  Components:
    path: components
    type: yaml
  Range:
    path: range
    type: symbol
  Duration:
    path: duration
    type: symbol
  Concentration:
    path: concentration
    type: symbol
  Details:
    type: section
    optional: true
-->

| Spell | Charge cost | Casting time | Method | Slot cost | Components | Range | Duration | Concentration | Details |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Arcane Lock](<../spells/arcane_lock.md> "dnd2024:spells/arcane_lock") | ` 0 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-arcane_lock "section:arcane_lock") |
| [Conjure Elemental](<../spells/conjure_elemental.md> "dnd2024:spells/conjure_elemental") | ` 7 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-conjure_elemental "section:conjure_elemental") |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` 0 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-detect_magic "section:detect_magic") |
| [Dispel Magic](<../spells/dispel_magic.md> "dnd2024:spells/dispel_magic") | ` 3 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-dispel_magic "section:dispel_magic") |
| [Enlarge/Reduce](<../spells/enlarge_reduce.md> "dnd2024:spells/enlarge_reduce") | ` 0 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-enlarge_reduce "section:enlarge_reduce") |
| [Fireball](<../spells/fireball.md> "dnd2024:spells/fireball") | ` 7 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-fireball "section:fireball") |
| [Flaming Sphere](<../spells/flaming_sphere.md> "dnd2024:spells/flaming_sphere") | ` 2 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-flaming_sphere "section:flaming_sphere") |
| [Ice Storm](<../spells/ice_storm.md> "dnd2024:spells/ice_storm") | ` 4 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-ice_storm "section:ice_storm") |
| [Invisibility](<../spells/invisibility.md> "dnd2024:spells/invisibility") | ` 2 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-invisibility "section:invisibility") |
| [Knock](<../spells/knock.md> "dnd2024:spells/knock") | ` 2 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-knock "section:knock") |
| [Light](<../spells/light.md> "dnd2024:spells/light") | ` 0 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-light "section:light") |
| [Lightning Bolt](<../spells/lightning_bolt.md> "dnd2024:spells/lightning_bolt") | ` 7 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-lightning_bolt "section:lightning_bolt") |
| [Mage Hand](<../spells/mage_hand.md> "dnd2024:spells/mage_hand") | ` 0 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-mage_hand "section:mage_hand") |
| [Passwall](<../spells/passwall.md> "dnd2024:spells/passwall") | ` 5 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-passwall "section:passwall") |
| [Plane Shift](<../spells/plane_shift.md> "dnd2024:spells/plane_shift") | ` 7 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-plane_shift "section:plane_shift") |
| [Protection from Evil and Good](<../spells/protection_from_evil_and_good.md> "dnd2024:spells/protection_from_evil_and_good") | ` 0 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-protection_from_evil_and_good "section:protection_from_evil_and_good") |
| [Telekinesis](<../spells/telekinesis.md> "dnd2024:spells/telekinesis") | ` 5 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-telekinesis "section:telekinesis") |
| [Wall of Fire](<../spells/wall_of_fire.md> "dnd2024:spells/wall_of_fire") | ` 4 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-wall_of_fire "section:wall_of_fire") |
| [Web](<../spells/web.md> "dnd2024:spells/web") | ` 2 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-web "section:web") |

##### Arcane lock

^splat-af92c00a-arcane_lock

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@arcane_lock/level -->

```yaml
lowest_possible
```

##### Conjure elemental

^splat-af92c00a-conjure_elemental

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@conjure_elemental/level -->

```yaml
lowest_possible
```

##### Detect magic

^splat-af92c00a-detect_magic

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@detect_magic/level -->

```yaml
lowest_possible
```

##### Dispel magic

^splat-af92c00a-dispel_magic

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@dispel_magic/level -->

```yaml
lowest_possible
```

##### Enlarge reduce

^splat-af92c00a-enlarge_reduce

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@enlarge_reduce/level -->

```yaml
lowest_possible
```

##### Fireball

^splat-af92c00a-fireball

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@fireball/level -->

```yaml
7
```

##### Flaming sphere

^splat-af92c00a-flaming_sphere

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@flaming_sphere/level -->

```yaml
lowest_possible
```

##### Ice storm

^splat-af92c00a-ice_storm

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@ice_storm/level -->

```yaml
lowest_possible
```

##### Invisibility

^splat-af92c00a-invisibility

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@invisibility/level -->

```yaml
lowest_possible
```

##### Knock

^splat-af92c00a-knock

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@knock/level -->

```yaml
lowest_possible
```

##### Light

^splat-af92c00a-light

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@light/level -->

```yaml
lowest_possible
```

##### Lightning bolt

^splat-af92c00a-lightning_bolt

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@lightning_bolt/level -->

```yaml
7
```

##### Mage hand

^splat-af92c00a-mage_hand

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@mage_hand/level -->

```yaml
lowest_possible
```

##### Passwall

^splat-af92c00a-passwall

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@passwall/level -->

```yaml
lowest_possible
```

##### Plane shift

^splat-af92c00a-plane_shift

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@plane_shift/level -->

```yaml
lowest_possible
```

##### Protection from evil and good

^splat-af92c00a-protection_from_evil_and_good

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@protection_from_evil_and_good/level -->

```yaml
lowest_possible
```

##### Telekinesis

^splat-af92c00a-telekinesis

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@telekinesis/level -->

```yaml
lowest_possible
```

##### Wall of fire

^splat-af92c00a-wall_of_fire

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@wall_of_fire/level -->

```yaml
lowest_possible
```

##### Web

^splat-af92c00a-web

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@web/level -->

```yaml
lowest_possible
```

#### Absorption

<!-- splat:yaml path=splat_rule/parameters/absorption -->

```yaml
equipment_state: held
action: reaction
trigger: another_creature_casts_spell_targeting_only_user
cancel_effect: true
charge_gain: spell_level
if_result_above_50:
  effect: retributive_strike
  trigger: automatic
  charge_basis: total_after_absorption
```

#### Last charge

<!-- splat:yaml path=splat_rule/parameters/last_charge -->

```yaml
roll:
  expression: 1d20
  scope: use
'on': 20
restore_charges:
  expression: 1d12+1
  scope: use
```

#### Retributive strike

<!-- splat:yaml path=splat_rule/parameters/retributive_strike -->

```yaml
action: magic
method:
- break_over_knee
- break_against_solid_surface
destroy_item: true
area:
  shape: emanation
  radius_ft: 30
  origin: staff
user:
  escape_chance_percent: 50
  on_escape:
    transport: random_plane
    damage: 0
  on_no_escape:
    damage:
      type: force
      amount:
        op: multiply
        args:
        - 16
        - binding: charges_before_destruction
other_creatures:
  save:
    ability: dexterity
    dc: 17
    failure:
      damage:
        type: force
        amount:
          op: multiply
          args:
          - 6
          - binding: charges_before_destruction
    success:
      damage_scale:
        numerator: 1
        denominator: 2
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
| ` eligible_classes ` | ` ["sorcerer", "warlock", "wizard"] ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_staffs ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
