---
splat_schema: 2
splat_id: c96e58c9-06f4-5457-9c3c-43725c3e0c20
splat_kind: spell
splat_key: dnd2024:spells/animal_shapes
splat_name: Animal Shapes
splat_origin: dnd2024:reference/spell/animal_shapes
splat_sources:
- SRD 5.2.1, pp. 108, https://www.dndbeyond.com/srd
---

# Animal Shapes


## Spell

| Field | Value |
| --- | --- |
| Level | 8 |
| School | Transmutation |
| Classes | Druid |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S |
| Duration | 24 hours |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 108 -->

Level 8 Transmutation (Druid)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: 24 hours

Choose any number of willing creatures that you can see within range. Each target shape-shifts into a Large or smaller Beast of your choice that has a Challenge Rating of 4 or lower. You can choose a different form for each target. On later turns, you can take a Magic action to transform the targets again.

A target’s game statistics are replaced by the chosen Beast’s statistics, but the target retains its creature type; Hit Points; Hit Point Dice; alignment; ability to communicate; and Intelligence, Wisdom, and Charisma scores. The target’s actions are limited by the Beast form’s anatomy, and it can’t cast spells. The target’s equipment melds into the new form, and the target can’t use any of that equipment while in that form.

The target gains a number of Temporary Hit Points equal to the Hit Points of the first form into which it shape-shifts. These Temporary Hit Points vanish if any remain when the spell ends. The transformation lasts for the duration or until the target ends it as a Bonus Action.

Original layout: [[../../srd-5.2.1/pages/108|page 108]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/animal_shapes ` |
| ` implementation ` | ` dnd2024:spell_animal_shapes ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 86400 ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` willing ` | ` true ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 30 ` |
| ` count ` | ` any ` |

#### Forms

<!-- splat:values path=splat_rule/parameters/forms -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` choice_per_target ` | ` true ` |
| ` type ` | ` beast ` |
| ` maximum_size ` | ` large ` |
| ` maximum_challenge_rating ` | ` 4 ` |

#### Transform again

<!-- splat:values path=splat_rule/parameters/transform_again -->

| Field | Value |
| --- | --- |
| ` economy ` | ` magic_action ` |
| ` timing ` | ` later_caster_turn ` |
| ` targets ` | ` still_transformed_original_targets ` |
| ` new_choice_per_target ` | ` true ` |
| ` additional_temporary_hit_points ` | ` false ` |

#### Statistics

<!-- splat:values path=splat_rule/parameters/statistics -->

| Field | Value |
| --- | --- |
| ` replace ` | ` form_stat_block ` |
| ` retain ` | ` ["creature_type", "hit_points", "hit_point_dice", "alignment", "communication", "intelligence", "wisdom", "charisma"] ` |

#### Restrictions

<!-- splat:values path=splat_rule/parameters/restrictions -->

| Field | Value |
| --- | --- |
| ` actions_limited_by_form_anatomy ` | ` true ` |
| ` cast_spells ` | ` false ` |
| ` equipment ` | ` melded ` |
| ` equipment_use ` | ` false ` |

#### Temporary hit points

<!-- splat:yaml path=splat_rule/parameters/temporary_hit_points -->

```yaml
amount:
  stat:
    of: first_chosen_form
    key: hit_points
grant_count_per_target: 1
remove_remaining_on_spell_end: true
```

#### End target

<!-- splat:values path=splat_rule/parameters/end_target -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` chooser ` | ` target ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
