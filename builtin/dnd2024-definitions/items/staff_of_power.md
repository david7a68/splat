---
splat_schema: 1
splat_id: f07a4ba2-2fb0-59dc-a460-dd85f85e57aa
splat_kind: item
splat_key: dnd2024:items/staff_of_power
splat_name: Staff of Power
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_power
splat_sources:
- title: SRD 5.2.1
  locator: pp. 245, 246
  url: https://www.dndbeyond.com/srd
---

# Staff of Power

## Source text

<!-- SRD page 245 -->

Staff, Very Rare (Requires Attunement by a Sorcerer, Warlock, or Wizard)

This staff has 20 charges and can be wielded as a magic Quarterstaff that grants a +2 bonus to attack rolls and damage rolls made with it. While holding it, you gain a +2 bonus to Armor Class, saving throws, and spell attack rolls.

Spells. While holding the staff, you can cast one of the spells on the following table from it, using your spell save DC. The table indicates how many charges you must expend to cast the spell.

<!-- SRD page 246 -->

| Spell | Charge Cost |
| --- | --- |
| Cone of Cold | 5 |
| Fireball (level 5 version) | 5 |
| Globe of Invulnerability | 6 |
| Hold Monster | 5 |
| Levitate | 2 |
| Lightning Bolt (level 5 version) | 5 |
| Magic Missile | 1 |
| Ray of Enfeeblement | 1 |
| Wall of Force | 5 |

Regaining Charges. The staff regains 2d8 + 4 expended charges daily at dawn. If you expend the last charge, roll 1d20. On a 1, the staff retains its +2 bonus to attack rolls and damage rolls but loses all other properties. On a 20, the staff regains 1d8 + 2 charges.

Retributive Strike. You can take a Magic action to break the staff over your knee or against a solid surface. The staff is destroyed and releases its magic in an explosion that fills a 30-foot Emanation originating from itself. You have a 50 percent chance to instantly travel to a random plane of existence, avoiding the explosion. If you fail to avoid the effect, you take Force damage equal to 16 times the number of charges in the staff. Each other creature in the area makes a DC 17 Dexterity saving throw. On a failed save, a creature takes Force damage equal to 4 times the number of charges in the staff. On a successful save, a creature takes half as much damage.

Original layout: [[../../srd-5.2.1/pages/245|page 245]], [[../../srd-5.2.1/pages/246|page 246]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` staff ` |
| ` rarity ` | ` very_rare ` |

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
| ` mechanic ` | ` dnd2024:items/staff_of_power ` |
| ` implementation ` | ` dnd2024:item_staff_of_power ` |

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
| ` ac_bonus ` | ` 2 ` |
| ` saving_throw_bonus ` | ` 2 ` |
| ` spell_attack_bonus ` | ` 2 ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 20
initial: 20
recovery:
  event: dawn
  amount:
    expression: 2d8+4
    scope: use
  cap: maximum
```

#### Spell grants

<!-- splat:table
path: splat_rule/parameters/spell_grants
key: Spell
order:
- cone_of_cold
- fireball
- globe_of_invulnerability
- hold_monster
- levitate
- lightning_bolt
- magic_missile
- ray_of_enfeeblement
- wall_of_force
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
| [Cone of Cold](<../spells/cone_of_cold.md> "dnd2024:spells/cone_of_cold") | ` 5 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-cone_of_cold "section:cone_of_cold") |
| [Fireball](<../spells/fireball.md> "dnd2024:spells/fireball") | ` 5 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-fireball "section:fireball") |
| [Globe of Invulnerability](<../spells/globe_of_invulnerability.md> "dnd2024:spells/globe_of_invulnerability") | ` 6 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-globe_of_invulnerability "section:globe_of_invulnerability") |
| [Hold Monster](<../spells/hold_monster.md> "dnd2024:spells/hold_monster") | ` 5 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-hold_monster "section:hold_monster") |
| [Levitate](<../spells/levitate.md> "dnd2024:spells/levitate") | ` 2 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-levitate "section:levitate") |
| [Lightning Bolt](<../spells/lightning_bolt.md> "dnd2024:spells/lightning_bolt") | ` 5 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-lightning_bolt "section:lightning_bolt") |
| [Magic Missile](<../spells/magic_missile.md> "dnd2024:spells/magic_missile") | ` 1 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-magic_missile "section:magic_missile") |
| [Ray of Enfeeblement](<../spells/ray_of_enfeeblement.md> "dnd2024:spells/ray_of_enfeeblement") | ` 1 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-ray_of_enfeeblement "section:ray_of_enfeeblement") |
| [Wall of Force](<../spells/wall_of_force.md> "dnd2024:spells/wall_of_force") | ` 5 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Spell default ` | ` Spell default ` | ` Spell default ` | [Rules](#^splat-af92c00a-wall_of_force "section:wall_of_force") |

##### Cone of cold

^splat-af92c00a-cone_of_cold

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@cone_of_cold/level -->

```yaml
lowest_possible
```

##### Fireball

^splat-af92c00a-fireball

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@fireball/level -->

```yaml
5
```

##### Globe of invulnerability

^splat-af92c00a-globe_of_invulnerability

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@globe_of_invulnerability/level -->

```yaml
lowest_possible
```

##### Hold monster

^splat-af92c00a-hold_monster

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@hold_monster/level -->

```yaml
lowest_possible
```

##### Levitate

^splat-af92c00a-levitate

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@levitate/level -->

```yaml
lowest_possible
```

##### Lightning bolt

^splat-af92c00a-lightning_bolt

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@lightning_bolt/level -->

```yaml
5
```

##### Magic missile

^splat-af92c00a-magic_missile

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@magic_missile/level -->

```yaml
lowest_possible
```

##### Ray of enfeeblement

^splat-af92c00a-ray_of_enfeeblement

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@ray_of_enfeeblement/level -->

```yaml
lowest_possible
```

##### Wall of force

^splat-af92c00a-wall_of_force

###### Level

<!-- splat:yaml path=splat_rule/parameters/spell_grants/@wall_of_force/level -->

```yaml
lowest_possible
```

#### Last charge

##### Table

###### Roll

<!-- splat:values path=splat_rule/parameters/last_charge/table/roll -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d20 ` |
| ` scope ` | ` use ` |

###### Rows

<!-- splat:table
path: splat_rule/parameters/last_charge/table/rows
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Roll:
    path: ''
    type: range
  Result retain only:
    path: result/retain_only
    type: yaml
    optional: true
  Result effect:
    path: result/effect
    type: symbol
    optional: true
  Result restore charges expression:
    path: result/restore_charges/expression
    type: string
    optional: true
  Result restore charges scope:
    path: result/restore_charges/scope
    type: symbol
    optional: true
-->

| Key | Roll | Result retain only | Result effect | Result restore charges expression | Result restore charges scope |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` ["weapon_attack_bonus", "weapon_damage_bonus"] ` |  |  |  |
| ` row_2 ` | ` 2–19 ` |  | ` None ` |  |  |
| ` row_3 ` | ` 20 ` |  |  | ` 1d8+2 ` | ` Use ` |

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
          - 4
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
