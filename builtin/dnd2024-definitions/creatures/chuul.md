---
splat_schema: 2
splat_id: a1021e18-4468-569c-9a73-fb1833b1c112
splat_kind: creature
splat_key: dnd2024:creatures/chuul
splat_name: Chuul
splat_origin: dnd2024:reference/creature/chuul
splat_sources:
- SRD 5.2.1, pp. 273, 274, https://www.dndbeyond.com/srd
---

# Chuul


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Aberration |
| Alignment | Chaotic Evil |
| Armor Class | 16 |
| Hit Points | 76 (9d10 + 27) |
| Initiative | +0 (10) |
| Speed | 30 ft., Swim 30 ft. |
| Challenge | 4 (XP 1,100; PB +2) |
| Skills | Perception +4 |
| Senses | Darkvision 60 ft.; Passive Perception 14 |
| Languages | Understands Deep Speech but can’t speak |
| Immunities | Poison; Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 10 | +0 | +0 |
| Con | 16 | +3 | +3 |
| Int | 5 | -3 | -3 |
| Wis | 11 | +0 | +0 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 273 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Aberration, Chaotic Evil\
> AC 16 Initiative +0 (10)\
> HP 76 (9d10 + 27)\
> Speed 30 ft., Swim 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +4 Dex 10 +0 +0 Con 16 +3 +3\
> Int 5 −3 −3 Wis 11 +0 +0 Cha 5 −3 −3\
> Skills Perception +4\
> Immunities Poison; Poisoned\
> Senses Darkvision 60 ft.; Passive Perception 14\
> Languages Understands Deep Speech but can’t speak\
> CR 4 (XP 1,100; PB +2)


### Traits

Amphibious. The chuul can breathe air and water.

Sense Magic. The chuul senses magic within 120 feet of itself. This trait otherwise works like the Detect Magic spell but isn’t itself magical.

### Actions

Multiattack. The chuul makes two Pincer attacks and uses Paralyzing Tentacles.

Pincer. Melee Attack Roll: +6, reach 10 ft. Hit: 9 (1d10 + 4) Bludgeoning damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 14) from one of two pincers.

<!-- SRD page 274 -->

Paralyzing Tentacles. Constitution Saving Throw: DC 13, one creature Grappled by the chuul. Failure: The target has the Poisoned condition and repeats the save at the end of each of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically. While Poisoned, the target has the Paralyzed condition.

Original layout: [[../../srd-5.2.1/pages/273|page 273]], [[../../srd-5.2.1/pages/274|page 274]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/chuul ` |
| ` implementation ` | ` dnd2024:creature_chuul ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["poisoned"] ` |
| ` passive_perception ` | ` 14 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` false ` |
| ` known ` | ` ["deep_speech"] ` |

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` breathing.allow ` |
| ` media ` | ` ["air", "water"] ` |

##### Sense magic

<!-- splat:yaml path=splat_rule/parameters/features/sense_magic -->

```yaml
economy: passive
op: spell.effect
spell:
  key: dnd2024:spells/detect_magic
range_override_ft: 120
magical: false
continuous: true
```

##### Pincer

<!-- splat:yaml path=splat_rule/parameters/features/pincer -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 6
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 1
        sides: 10
      add: 4
    average: 9
- op: grapple
  escape_dc: 14
  when:
    target_kind: creature
    target_maximum_size: large
  limb_slots: 2
  slots_per_target: 1
```

##### Paralyzing tentacles

<!-- splat:yaml path=splat_rule/parameters/features/paralyzing_tentacles -->

```yaml
economy: action
op: saving_throw
ability: constitution
dc: 13
targets:
  kind: creature
  count: 1
  grappled_by: actor
failure:
- op: condition.apply
  condition: poisoned
  duration:
    amount: 1
    unit: minute
  while_active:
  - op: condition.apply
    condition: paralyzed
  repeat_save:
    event: target.turn_end
    ability: constitution
    dc: 13
    success: effect.end
    automatic_success_after:
      amount: 1
      unit: minute
success: []
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` multiattack ` |

###### Sequence

<!-- splat:table
path: splat_rule/parameters/features/multiattack/sequence
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
  Count:
    path: count
    type: integer
-->

| Key | Feature | Count |
| --- | --- | --- |
| ` row_1 ` | ` Pincer ` | ` 2 ` |
| ` row_2 ` | ` Paralyzing tentacles ` | ` 1 ` |

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spellcasting defaults

<!-- splat:yaml path=splat_rule/parameters/spellcasting_defaults -->

```yaml
cast_level: lowest_possible_unless_overridden
upcast_allowed: only_explicit_cast_level_overrides
required_material_components_available: true
casting_time_at_least_1_minute:
  uses_normal_time: true
  magic_action_each_turn: true
  concentration_required: true
other_components: spell_definition_unless_waived
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> swim_speed:
>   unit: ft
>   value: 30
> ```

