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

Staff, Very Rare (Requires Attunement by a Sorcerer,\
Warlock, or Wizard)\
This staff has 20 charges and can be wielded as a\
magic Quarterstaff that grants a +2 bonus to attack\
rolls and damage rolls made with it. While hold-\
ing it, you gain a +2 bonus to Armor Class, saving\
throws, and spell attack rolls.\
Spells. While holding the staff, you can cast one of\
the spells on the following table from it, using your\
spell save DC. The table indicates how many charges\
you must expend to cast the spell.\

<!-- SRD page 246 -->

Charge\
Spell Cost\
Cone of Cold 5\
Fireball (level 5 version) 5\
Globe of Invulnerability 6\
Hold Monster 5\
Levitate 2\
Lightning Bolt (level 5 version) 5\
Magic Missile 1\
Ray of Enfeeblement 1\
Wall of Force 5\
Regaining Charges. The staff regains 2d8 + 4\
expended charges daily at dawn. If you expend the\
last charge, roll 1d20. On a 1, the staff retains its +2\
bonus to attack rolls and damage rolls but loses all\
other properties. On a 20, the staff regains 1d8 + 2\
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
4 times the number of charges in the staff. On a suc-\
cessful save, a creature takes half as much damage.\

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

<!-- splat:rows path=splat_rule/parameters/spell_grants -->

| Row | charge_cost | casting_time | method | slot_cost | level | range | duration | concentration |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 5 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 1 ` | ` 5 ` | ` spell_default ` | ` item ` | ` 0 ` | ` 5 ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 2 ` | ` 6 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 3 ` | ` 5 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 4 ` | ` 2 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 5 ` | ` 5 ` | ` spell_default ` | ` item ` | ` 0 ` | ` 5 ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 6 ` | ` 1 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 7 ` | ` 1 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 8 ` | ` 5 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/cone_of_cold ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/0/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fireball ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/1/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/2/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/globe_of_invulnerability ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/2/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/3/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hold_monster ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/3/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/4/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/levitate ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/4/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/5/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/lightning_bolt ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/5/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/6/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/magic_missile ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/6/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/7/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/ray_of_enfeeblement ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/7/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/8/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wall_of_force ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/8/components -->

```yaml
[]
```

#### Last charge

<!-- splat:yaml path=splat_rule/parameters/last_charge -->

```yaml
table:
  roll:
    expression: 1d20
    scope: use
  rows:
  - min: 1
    max: 1
    result:
      retain_only:
      - weapon_attack_bonus
      - weapon_damage_bonus
  - min: 2
    max: 19
    result:
      effect: none
  - min: 20
    max: 20
    result:
      restore_charges:
        expression: 1d8+2
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
