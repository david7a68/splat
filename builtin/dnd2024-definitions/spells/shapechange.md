---
splat_schema: 1
splat_id: 42b2915c-83c3-55c3-96d8-24755c610813
splat_kind: spell
splat_key: dnd2024:spells/shapechange
splat_name: Shapechange
splat_origin:
  key: dnd2024:reference/spell/shapechange
splat_sources:
- title: SRD 5.2.1
  locator: pp. 161
  url: https://www.dndbeyond.com/srd
---

# Shapechange

## Source text

<!-- SRD page 161 -->

Level 9 Transmutation (Druid, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a jade circlet worth 1,500+ GP)\
Duration: Concentration, up to 1 hour\
You shape-shift into another creature for the dura-\
tion or until you take a Magic action to shape-shift\
into a different eligible form. The new form must be\
of a creature that has a Challenge Rating no higher\
than your level or Challenge Rating. You must have\
seen the sort of creature before, and it can’t be a\
Construct or an Undead.\
When you cast the spell, you gain a number of\
Temporary Hit Points equal to the Hit Points of the\
first form into which you shape-shift. These Tempo-\
rary Hit Points vanish if any remain when the spell\
ends.\
Your game statistics are replaced by the stat block\
of the chosen form, but you retain your creature\
type; alignment; personality; Intelligence, Wisdom,\
and Charisma scores; Hit Points; Hit Point Dice; pro-\
ficiencies; and ability to communicate. If you have\
the Spellcasting feature, you retain it too.\
Upon shape-shifting, you determine whether your\
equipment drops to the ground or changes in size\
and shape to fit the new form while you’re in it.\

Original layout: [[../../srd-5.2.1/pages/161|page 161]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 9 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["druid", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a jade circlet worth 1,500+ GP ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/shapechange ` |
| ` implementation ` | ` dnd2024:spell_shapechange ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |

#### Form

<!-- splat:yaml path=splat_rule/parameters/form -->

```yaml
chooser: caster
maximum_cr:
  source: caster
  stat_choice:
  - level
  - challenge_rating
sort_previously_seen: true
excluded_types:
- construct
- undead
change_action: magic
```

#### Temporary hp

<!-- splat:yaml path=splat_rule/parameters/temporary_hp -->

```yaml
on_initial_cast_only: true
amount:
  stat:
    of: initial_form
    key: hit_points
remaining_vanish_on_end: true
```

#### Statistics

<!-- splat:values path=splat_rule/parameters/statistics -->

| Field | Value |
| --- | --- |
| ` replace_with ` | ` chosen_form_stat_block ` |
| ` retain ` | ` ["creature_type", "alignment", "personality", "intelligence", "wisdom", "charisma", "hit_points", "hit_point_dice", "proficiencies", "communication_ability", "spellcasting_feature_if_present"] ` |

#### Equipment

<!-- splat:values path=splat_rule/parameters/equipment -->

| Field | Value |
| --- | --- |
| ` choice_on_shape_shift ` | ` ["drop_to_ground", "resize_and_reshape_to_fit"] ` |
| ` fit_duration ` | ` while_in_form ` |
