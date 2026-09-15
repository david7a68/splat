---
splat_schema: 1
splat_id: 967410a1-be27-5ac3-b0f8-f61e07e362d8
splat_kind: item
splat_key: dnd2024:items/staff_of_the_woodlands
splat_name: Staff of the Woodlands
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_the_woodlands
splat_sources:
- title: SRD 5.2.1
  locator: pp. 247
  url: https://www.dndbeyond.com/srd
---

# Staff of the Woodlands

## Source text

<!-- SRD page 247 -->

Staff, Rare (Requires Attunement by a Druid)\
This staff has 6 charges and can be wielded as a\
magic Quarterstaff that grants a +2 bonus to attack\
rolls and damage rolls made with it. While holding\
it, you have a +2 bonus to spell attack rolls.\
Spells. While holding the staff, you can cast one of\
the spells on the following table from it, using your\
spell save DC. The table indicates how many charges\
you must expend to cast the spell.\
Spell Charge\
Cost\
Animal Friendship 1\
Awaken 5\
Barkskin 2\
Locate Animals or Plants 2\
Pass without Trace 2\
Speak with Animals 1\
Speak with Plants 3\
Wall of Thorns 6\
Tree Form. You can take a Magic action to plant\
one end of the staff in earth in an unoccupied space\
and expend 1 charge to transform the staff into\
a healthy tree. The tree is 60 feet tall and has a\
5-foot-diameter trunk, and its branches at the top\
spread out in a 20-foot radius. The tree appears\
ordinary but radiates a faint aura of Transmutation\
magic that can be discerned with the Detect Magic\
spell. While touching the tree and using a Magic\
action, you return the staff to its normal form. Any\
creature in the tree falls when the tree reverts to\
a staff.\
Regaining Charges. The staff regains 1d6 ex-\
pended charges daily at dawn. If you expend the last\
charge, roll 1d20. On a 1, the staff loses its proper-\
ties and becomes a nonmagical Quarterstaff.\

Original layout: [[../../srd-5.2.1/pages/247|page 247]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` staff ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |
| ` attunement_requirement_text ` | ` a Druid ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_the_woodlands ` |
| ` implementation ` | ` dnd2024:item_staff_of_the_woodlands ` |

### Parameters

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 6
initial: 6
recovery:
  event: dawn
  amount:
    dice: 1
    sides: 6
    bonus: 0
```

#### Weapon

<!-- splat:yaml path=splat_rule/parameters/weapon -->

```yaml
base:
  key: dnd2024:equipment/quarterstaff
magical: true
attack_bonus: 2
damage_bonus: 2
```

#### Held

<!-- splat:values path=splat_rule/parameters/held -->

| Field | Value |
| --- | --- |
| ` spell_attack_bonus ` | ` 2 ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | cost | save_dc | requires |
| --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` user_spell_save_dc ` | ` held ` |
| ` 1 ` | ` 5 ` | ` user_spell_save_dc ` | ` held ` |
| ` 2 ` | ` 2 ` | ` user_spell_save_dc ` | ` held ` |
| ` 3 ` | ` 2 ` | ` user_spell_save_dc ` | ` held ` |
| ` 4 ` | ` 2 ` | ` user_spell_save_dc ` | ` held ` |
| ` 5 ` | ` 1 ` | ` user_spell_save_dc ` | ` held ` |
| ` 6 ` | ` 3 ` | ` user_spell_save_dc ` | ` held ` |
| ` 7 ` | ` 6 ` | ` user_spell_save_dc ` | ` held ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/animal_friendship ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/awaken ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/2/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/barkskin ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/3/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_animals_or_plants ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/4/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/pass_without_trace ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/5/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_animals ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/6/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_plants ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/7/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wall_of_thorns ` |

#### Tree form

<!-- splat:yaml path=splat_rule/parameters/tree_form -->

```yaml
action: magic
requires:
  plant_end_in: earth
  space: unoccupied
cost: 1
height_ft: 60
trunk_diameter_ft: 5
top_branch_radius_ft: 20
appearance: ordinary_healthy_tree
detect_magic:
  school: transmutation
  intensity: faint
revert:
  action: magic
  requires: touch_tree
  occupants: fall
```

#### Last charge

<!-- splat:yaml path=splat_rule/parameters/last_charge -->

```yaml
roll:
  dice: 1
  sides: 20
  bonus: 0
on_value: 1
effect: nonmagical_quarterstaff
```

#### Attunement

<!-- splat:yaml path=splat_rule/parameters/attunement -->

```yaml
required: true
eligibility:
  class_any:
  - druid
```
