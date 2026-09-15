---
splat_schema: 1
splat_id: 815fe32c-d12a-540a-acde-9d65b2337972
splat_kind: spell
splat_key: dnd2024:spells/prismatic_spray
splat_name: Prismatic Spray
splat_origin:
  key: dnd2024:reference/spell/prismatic_spray
splat_sources:
- title: SRD 5.2.1
  locator: pp. 154, 155
  url: https://www.dndbeyond.com/srd
---

# Prismatic Spray

## Source text

<!-- SRD page 154 -->

Level 7 Evocation (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Instantaneous\
Eight rays of light flash from you in a 60-foot Cone.\
Each creature in the Cone makes a Dexterity saving\
throw. For each target, roll 1d8 to determine which\
color ray affects it, consulting the Prismatic Rays\
table.\
Prismatic Rays\
1d8 Ray\
1 Red. Failed Save: 12d6 Fire damage. Successful\
Save: Half as much damage.\
2 Orange. Failed Save: 12d6 Acid damage. Successful\
Save: Half as much damage.\
3 Yellow. Failed Save: 12d6 Lightning damage. Suc-\
cessful Save: Half as much damage.\
4 Green. Failed Save: 12d6 Poison damage. Success-\
ful Save: Half as much damage.\

<!-- SRD page 155 -->

1d8 Ray\
5 Blue. Failed Save: 12d6 Cold damage. Successful\
Save: Half as much damage.\
6 Indigo. Failed Save: The target has the Restrained\
condition and makes a Constitution saving throw\
at the end of each of its turns. If it successfully\
saves three times, the condition ends. If it fails\
three times, it has the Petrified condition until it\
is freed by an effect like the Greater Restoration\
spell. The successes and failures needn’t be con-\
secutive; keep track of both until the target col-\
lects three of a kind.\
7 Violet. Failed Save: The target has the Blinded\
condition and makes a Wisdom saving throw at\
the start of your next turn. On a successful save,\
the condition ends. On a failed save, the condi-\
tion ends, and the creature teleports to another\
plane of existence (GM’s choice).\
8 Special. The target is struck by two rays. Roll\
twice, rerolling any 8.\

Original layout: [[../../srd-5.2.1/pages/154|page 154]], [[../../srd-5.2.1/pages/155|page 155]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["bard", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/prismatic_spray ` |
| ` implementation ` | ` dnd2024:spell_prismatic_spray ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cone ` |
| ` length_feet ` | ` 60 ` |
| ` origin ` | ` caster ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` selection ` | ` all_in_area ` |

#### Initial save

<!-- splat:values path=splat_rule/parameters/initial_save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` dc ` | ` spell_save_dc ` |
| ` count_per_target ` | ` 1 ` |

#### Ray roll

<!-- splat:values path=splat_rule/parameters/ray_roll -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 1d8 ` |
| ` scope ` | ` target ` |

#### Rays

<!-- splat:rows path=splat_rule/parameters/rays -->

| Row | roll | color | initial_success | ray_count | roll_each | reroll_values | apply_both |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` red ` |  |  |  |  |  |
| ` 1 ` | ` 2 ` | ` orange ` |  |  |  |  |  |
| ` 2 ` | ` 3 ` | ` yellow ` |  |  |  |  |  |
| ` 3 ` | ` 4 ` | ` green ` |  |  |  |  |  |
| ` 4 ` | ` 5 ` | ` blue ` |  |  |  |  |  |
| ` 5 ` | ` 6 ` | ` indigo ` | ` unaffected ` |  |  |  |  |
| ` 6 ` | ` 7 ` | ` violet ` | ` unaffected ` |  |  |  |  |
| ` 7 ` | ` 8 ` | ` special ` |  | ` 2 ` | ` 1d8 ` | ` [8] ` | ` true ` |

##### Failure damage

<!-- splat:values path=splat_rule/parameters/rays/0/failure_damage -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 12d6 ` |
| ` type ` | ` fire ` |

##### Success damage scale

<!-- splat:values path=splat_rule/parameters/rays/0/success_damage_scale -->

| Field | Value |
| --- | --- |
| ` numerator ` | ` 1 ` |
| ` denominator ` | ` 2 ` |

##### Failure damage

<!-- splat:values path=splat_rule/parameters/rays/1/failure_damage -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 12d6 ` |
| ` type ` | ` acid ` |

##### Success damage scale

<!-- splat:values path=splat_rule/parameters/rays/1/success_damage_scale -->

| Field | Value |
| --- | --- |
| ` numerator ` | ` 1 ` |
| ` denominator ` | ` 2 ` |

##### Failure damage

<!-- splat:values path=splat_rule/parameters/rays/2/failure_damage -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 12d6 ` |
| ` type ` | ` lightning ` |

##### Success damage scale

<!-- splat:values path=splat_rule/parameters/rays/2/success_damage_scale -->

| Field | Value |
| --- | --- |
| ` numerator ` | ` 1 ` |
| ` denominator ` | ` 2 ` |

##### Failure damage

<!-- splat:values path=splat_rule/parameters/rays/3/failure_damage -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 12d6 ` |
| ` type ` | ` poison ` |

##### Success damage scale

<!-- splat:values path=splat_rule/parameters/rays/3/success_damage_scale -->

| Field | Value |
| --- | --- |
| ` numerator ` | ` 1 ` |
| ` denominator ` | ` 2 ` |

##### Failure damage

<!-- splat:values path=splat_rule/parameters/rays/4/failure_damage -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 12d6 ` |
| ` type ` | ` cold ` |

##### Success damage scale

<!-- splat:values path=splat_rule/parameters/rays/4/success_damage_scale -->

| Field | Value |
| --- | --- |
| ` numerator ` | ` 1 ` |
| ` denominator ` | ` 2 ` |

##### Initial failure

<!-- splat:values path=splat_rule/parameters/rays/5/initial_failure -->

| Field | Value |
| --- | --- |
| ` condition ` | ` restrained ` |

##### Subsequent save

<!-- splat:yaml path=splat_rule/parameters/rays/5/subsequent_save -->

```yaml
event: turn.end
subject: target
ability: constitution
dc: spell_save_dc
counters:
- successes
- failures
counters_initial: 0
consecutive_required: false
successes_threshold:
  count: 3
  effect: end_restrained
failures_threshold:
  count: 3
  effect: petrified
  duration: until_freed
stop_saves_on_threshold: true
```

##### Freeing example

<!-- splat:values path=splat_rule/parameters/rays/5/freeing_example -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/greater_restoration ` |

##### Initial failure

<!-- splat:values path=splat_rule/parameters/rays/6/initial_failure -->

| Field | Value |
| --- | --- |
| ` condition ` | ` blinded ` |

##### Subsequent save

<!-- splat:yaml path=splat_rule/parameters/rays/6/subsequent_save -->

```yaml
event: turn.start
subject: caster
occurrence: next
ability: wisdom
dc: spell_save_dc
save_subject: target
success:
  end_condition: blinded
failure:
  end_condition: blinded
  teleport:
    plane:
      chooser: gm
      constraint: different_plane
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
