---
splat_schema: 1
splat_id: 0e2b61b1-d000-5af1-af7c-b265c8b5025f
splat_kind: spell
splat_key: dnd2024:spells/searing_smite
splat_name: Searing Smite
splat_origin:
  key: dnd2024:reference/spell/searing_smite
splat_sources:
- title: SRD 5.2.1
  locator: pp. 160
  url: https://www.dndbeyond.com/srd
---

# Searing Smite

## Source text

<!-- SRD page 160 -->

Level 1 Evocation (Paladin)\
Casting Time: Bonus Action, which you take immedi-\
ately after hitting a target with a Melee weapon or an\
Unarmed Strike\
Range: Self\
Component: V\
Duration: 1 minute\
As you hit the target, it takes an extra 1d6 Fire dam-\
age from the attack. At the start of each of its turns\
until the spell ends, the target takes 1d6 Fire dam-\
age and then makes a Constitution saving throw.\
On a failed save, the spell continues. On a successful\
save, the spell ends.\
Using a Higher-Level Spell Slot. All the damage\
increases by 1d6 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/160|page 160]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["paladin"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 minute ` |
| ` casting_time ` | ` Bonus Action, which you take immediately after hitting a target with a Melee weapon or an Unarmed Strike ` |
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
| ` mechanic ` | ` dnd2024:spells/searing_smite ` |
| ` implementation ` | ` dnd2024:spell_searing_smite ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` identity ` | ` target_hit_by_triggering_attack ` |

#### Extra attack damage

<!-- splat:yaml path=splat_rule/parameters/extra_attack_damage -->

```yaml
dice:
  count:
    op: add
    args:
    - 1
    - op: multiply
      args:
      - 1
      - op: subtract
        args:
        - binding: cast_level
        - 1
  sides: 6
  add: 0
type: fire
```

#### Ongoing

<!-- splat:values path=splat_rule/parameters/ongoing -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |
| ` event ` | ` target_turn_start ` |

##### Steps

<!-- splat:table
path: splat_rule/parameters/ongoing/steps
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Damage dice sides:
    path: damage/dice/sides
    type: integer
    optional: true
  Damage dice add:
    path: damage/dice/add
    type: integer
    optional: true
  Damage type:
    path: damage/type
    type: symbol
    optional: true
  Save ability:
    path: save/ability
    type: symbol
    optional: true
  Save dc binding:
    path: save/dc/binding
    type: symbol
    optional: true
  Save failure continue spell:
    path: save/failure/continue_spell
    type: boolean
    optional: true
  Save success end spell:
    path: save/success/end_spell
    type: boolean
    optional: true
  Details:
    type: section
    optional: true
-->

| Key | Damage dice sides | Damage dice add | Damage type | Save ability | Save dc binding | Save failure continue spell | Save success end spell | Details |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 6 ` | ` 0 ` | ` Fire ` |  |  |  |  | [Rules](#^splat-5ea14734-row_1 "section:row_1") |
| ` row_2 ` |  |  |  | ` Constitution ` | ` Save dc ` | true | true |  |

###### Row 1

^splat-5ea14734-row_1

###### Count

<!-- splat:yaml path=splat_rule/parameters/ongoing/steps/@row_1/damage/dice/count -->

```yaml
op: add
args:
- 1
- op: multiply
  args:
  - 1
  - op: subtract
    args:
    - binding: cast_level
    - 1
```
