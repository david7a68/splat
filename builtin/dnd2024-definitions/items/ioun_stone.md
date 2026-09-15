---
splat_schema: 1
splat_id: 07e6945f-0ea0-58cd-84e9-59c41d125c5a
splat_kind: item
splat_key: dnd2024:items/ioun_stone
splat_name: Ioun Stone
splat_origin:
  key: dnd2024:reference/magic_item/ioun_stone
splat_sources:
- title: SRD 5.2.1
  locator: pp. 227, 228
  url: https://www.dndbeyond.com/srd
---

# Ioun Stone

## Source text

<!-- SRD page 227 -->

Wondrous Item, Rarity Varies (Requires Attunement)\
Roughly marble sized, Ioun Stones are named after\
Ioun, a god of knowledge and prophecy revered on\
some worlds. Many types of Ioun Stones exist, each\
type a distinct combination of shape and color.\
When you take a Magic action to toss an Ioun\
Stone into the air, the stone orbits your head at a\
distance of 1d3 feet, conferring its benefit to you\
while doing so. You can have up to three Ioun Stones\
orbiting your head at the same time.\
Each Ioun Stone orbiting your head is considered\
to be an object you are wearing. The orbiting stone\
avoids contact with other creatures and objects, ad-\
justing its orbit to avoid collisions and thwarting all\
attempts by other creatures to attack or snatch it.\
As a Utilize action, you can seize and stow any\
number of Ioun Stones orbiting your head. If your\
Attunement to an Ioun Stone ends while it’s orbit-\
ing your head, the stone falls as though you had\
dropped it.\
The type of stone determines its rarity and\
effects.\
Absorption (Very Rare). While this pale lavender\
ellipsoid orbits your head, you can take a Reaction\
to cancel a spell of level 4 or lower cast by a crea-\
ture you can see. A canceled spell has no effect, and\
any resources used to cast it are wasted. Once the\
stone has canceled 20 levels of spells, it burns out,\
turns dull gray, and loses its magic.\
Agility (Very Rare). Your Dexterity increases by\
2, to a maximum of 20, while this deep-red sphere\
orbits your head.\
Awareness (Rare). While this dark-blue rhomboid\
orbits your head, you have Advantage on Initiative\
rolls and Wisdom (Perception) checks.\
Fortitude (Very Rare). Your Constitution in-\
creases by 2, to a maximum of 20, while this pink\
rhomboid orbits your head.\
Greater Absorption (Legendary). While this mar-\
bled lavender and green ellipsoid orbits your head,\
you can take a Reaction to cancel a spell of level 8\
or lower cast by a creature you can see. A canceled\
spell has no effect, and any resources used to cast\
it are wasted. Once the stone has canceled 20 lev-\
els of spells, it burns out, turns dull gray, and loses\
its magic.\
Insight (Very Rare). Your Wisdom increases by\
2, to a maximum of 20, while this incandescent blue\
sphere orbits your head.\
Intellect (Very Rare). Your Intelligence increases\
by 2, to a maximum of 20, while this marbled scarlet\
and blue sphere orbits your head.\
Leadership (Very Rare). Your Charisma increases\
by 2, to a maximum of 20, while this marbled pink\
and green sphere orbits your head.\

<!-- SRD page 228 -->

Mastery (Legendary). Your Proficiency Bonus in-\
creases by 1 while this pale green prism orbits your\
head.\
Protection (Rare). You gain a +1 bonus to Armor\
Class while this dusty-rose prism orbits your head.\
Regeneration (Legendary). You regain 15 Hit\
Points at the end of each hour this pearly white\
spindle orbits your head if you have at least 1 Hit\
Point.\
Reserve (Rare). This vibrant purple prism stores\
spells cast into it, holding them until you use them.\
The stone can store up to 4 levels of spells at a time.\
When found, it contains 1d4 levels of stored spells\
chosen by the GM.\
Any creature can cast a spell of level 1 through 4\
into the stone by touching it as the spell is cast. The\
spell has no effect, other than to be stored in the\
stone. If the stone can’t hold the spell, the spell is\
expended without effect. The level of the slot used\
to cast the spell determines how much space it uses.\
While this stone orbits your head, you can cast\
any spell stored in it. The spell uses the slot level,\
spell save DC, spell attack bonus, and spellcast-\
ing ability of the original caster but is otherwise\
treated as if you cast the spell. The spell cast from\
the stone is no longer stored in it, freeing up space.\
Strength (Very Rare). Your Strength increases by\
2, to a maximum of 20, while this pale blue rhom-\
boid orbits your head.\
Sustenance (Rare). You don’t need to eat or drink\
while this clear spindle orbits your head.\

Original layout: [[../../srd-5.2.1/pages/227|page 227]], [[../../srd-5.2.1/pages/228|page 228]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rarity_varies ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ioun_stone ` |
| ` implementation ` | ` dnd2024:item_ioun_stone ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` benefits_require_orbit ` | ` true ` |
| ` attunement_required ` | ` true ` |

#### Activate

<!-- splat:yaml path=splat_rule/parameters/activate -->

```yaml
action: magic
toss_in_air: true
orbit_distance_ft:
  roll:
    count: 1
    sides: 3
maximum_simultaneously_orbiting: 3
```

#### Orbit

<!-- splat:values path=splat_rule/parameters/orbit -->

| Field | Value |
| --- | --- |
| ` counts_as ` | ` worn ` |
| ` avoids_collisions ` | ` true ` |
| ` other_creatures_cannot ` | ` ["attack", "snatch"] ` |

#### Stow

<!-- splat:values path=splat_rule/parameters/stow -->

| Field | Value |
| --- | --- |
| ` action ` | ` utilize ` |
| ` quantity ` | ` any_number_orbiting ` |

#### Attunement end

<!-- splat:values path=splat_rule/parameters/attunement_end -->

| Field | Value |
| --- | --- |
| ` orbiting ` | ` fall_as_dropped ` |

#### Variants

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | name | rarity | ability | increase | maximum | advantage | proficiency_bonus_increase | ac_bonus | needs_food | needs_drink |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` absorption ` | ` very_rare ` |  |  |  |  |  |  |  |  |
| ` 1 ` | ` greater_absorption ` | ` legendary ` |  |  |  |  |  |  |  |  |
| ` 2 ` | ` agility ` | ` very_rare ` | ` dexterity ` | ` 2 ` | ` 20 ` |  |  |  |  |  |
| ` 3 ` | ` fortitude ` | ` very_rare ` | ` constitution ` | ` 2 ` | ` 20 ` |  |  |  |  |  |
| ` 4 ` | ` insight ` | ` very_rare ` | ` wisdom ` | ` 2 ` | ` 20 ` |  |  |  |  |  |
| ` 5 ` | ` intellect ` | ` very_rare ` | ` intelligence ` | ` 2 ` | ` 20 ` |  |  |  |  |  |
| ` 6 ` | ` leadership ` | ` very_rare ` | ` charisma ` | ` 2 ` | ` 20 ` |  |  |  |  |  |
| ` 7 ` | ` strength ` | ` very_rare ` | ` strength ` | ` 2 ` | ` 20 ` |  |  |  |  |  |
| ` 8 ` | ` awareness ` | ` rare ` |  |  |  | ` ["initiative", "wisdom_perception"] ` |  |  |  |  |
| ` 9 ` | ` mastery ` | ` legendary ` |  |  |  |  | ` 1 ` |  |  |  |
| ` 10 ` | ` protection ` | ` rare ` |  |  |  |  |  | ` 1 ` |  |  |
| ` 11 ` | ` regeneration ` | ` legendary ` |  |  |  |  |  |  |  |  |
| ` 12 ` | ` reserve ` | ` rare ` |  |  |  |  |  |  |  |  |
| ` 13 ` | ` sustenance ` | ` rare ` |  |  |  |  |  |  | ` false ` | ` false ` |

##### Reaction

<!-- splat:values path=splat_rule/parameters/variants/0/reaction -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` visible_creature_casts_spell ` |
| ` max_spell_level ` | ` 4 ` |
| ` effect ` | ` cancel_spell ` |
| ` resources_wasted ` | ` true ` |
| ` capacity_spell_levels ` | ` 20 ` |
| ` cumulative_levels_at_capacity ` | ` ["burn_out", "nonmagical"] ` |
| ` color_after ` | ` dull_gray ` |

##### Reaction

<!-- splat:values path=splat_rule/parameters/variants/1/reaction -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` visible_creature_casts_spell ` |
| ` max_spell_level ` | ` 8 ` |
| ` effect ` | ` cancel_spell ` |
| ` resources_wasted ` | ` true ` |
| ` capacity_spell_levels ` | ` 20 ` |
| ` cumulative_levels_at_capacity ` | ` ["burn_out", "nonmagical"] ` |
| ` color_after ` | ` dull_gray ` |

##### Healing

<!-- splat:yaml path=splat_rule/parameters/variants/11/healing -->

```yaml
hp: 15
interval:
  amount: 1
  unit: hours
requires_hp_at_least: 1
```

##### Spell storage

<!-- splat:yaml path=splat_rule/parameters/variants/12/spell_storage -->

```yaml
capacity_spell_levels: 4
initial_spell_levels:
  roll:
    count: 1
    sides: 4
initial_spells_choice: gm
store:
  caster: any_creature
  touch_required_during_cast: true
  spell_level_min: 1
  spell_level_max: 4
  occupancy: cast_slot_level
  normal_spell_effect: false
  insufficient_capacity: expend_without_effect
release:
  cast_as: user
  copy_original_caster:
  - slot_level
  - save_dc
  - spell_attack_bonus
  - spellcasting_ability
  remove_spell_from_storage: true
  free_capacity: true
```

#### Absorption remaining budget policy

<!-- splat:values path=splat_rule/parameters/absorption_remaining_budget_policy -->

| Field | Value |
| --- | --- |
| ` adjudicator ` | ` gm ` |
| ` scope ` | ` spell_level_exceeds_remaining_lifetime_budget ` |
| ` options ` | ` ["reject_absorption", "absorb_and_burn_out"] ` |
| ` source_unspecified ` | ` true ` |

#### Absorption burnout threshold

<!-- splat:values path=splat_rule/parameters/absorption_burnout_threshold -->

| Field | Value |
| --- | --- |
| ` comparison ` | ` greater_than_or_equal ` |
| ` levels ` | ` 20 ` |
