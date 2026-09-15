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

<!-- splat:rows path=splat_rule/parameters/spell_grants -->

| Row | charge_cost | casting_time | method | slot_cost | level | range | duration | concentration |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 1 ` | ` 7 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 2 ` | ` 0 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 3 ` | ` 3 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 4 ` | ` 0 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 5 ` | ` 7 ` | ` spell_default ` | ` item ` | ` 0 ` | ` 7 ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 6 ` | ` 2 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 7 ` | ` 4 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 8 ` | ` 2 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 9 ` | ` 2 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 10 ` | ` 0 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 11 ` | ` 7 ` | ` spell_default ` | ` item ` | ` 0 ` | ` 7 ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 12 ` | ` 0 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 13 ` | ` 5 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 14 ` | ` 7 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 15 ` | ` 0 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 16 ` | ` 5 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 17 ` | ` 4 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 18 ` | ` 2 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/arcane_lock ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/0/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/conjure_elemental ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/1/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/2/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_magic ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/2/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/3/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dispel_magic ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/3/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/4/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/enlarge_reduce ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/4/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/5/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fireball ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/5/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/6/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/flaming_sphere ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/6/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/7/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/ice_storm ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/7/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/8/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/invisibility ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/8/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/9/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/knock ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/9/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/10/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/light ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/10/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/11/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/lightning_bolt ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/11/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/12/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mage_hand ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/12/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/13/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/passwall ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/13/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/14/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/plane_shift ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/14/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/15/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/protection_from_evil_and_good ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/15/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/16/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/telekinesis ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/16/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/17/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wall_of_fire ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/17/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/18/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/web ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/18/components -->

```yaml
[]
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
