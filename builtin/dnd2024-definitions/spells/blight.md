---
splat_schema: 2
splat_id: 6eba6cf9-5f77-5c1f-8cd1-b5408e407b95
splat_kind: spell
splat_key: dnd2024:spells/blight
splat_name: Blight
splat_origin: dnd2024:reference/spell/blight
splat_sources:
- SRD 5.2.1, pp. 113, https://www.dndbeyond.com/srd
---

# Blight


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Necromancy |
| Classes | Druid, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 113 -->

Level 4 Necromancy (Druid, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Instantaneous

A creature that you can see within range makes a Constitution saving throw, taking 8d8 Necrotic damage on a failed save or half as much damage on a successful one. A Plant creature automatically fails the save.

Alternatively, target a nonmagical plant that isn’t a creature, such as a tree or shrub. It doesn’t make a save; it simply withers and dies.

Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 4.

Original layout: [[../../srd-5.2.1/pages/113|page 113]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/blight ` |
| ` implementation ` | ` dnd2024:spell_blight ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Modes

<!-- splat:yaml path=splat_rule/parameters/modes -->

```yaml
creature:
  target:
    kind: creature
    visible: true
    range_feet: 30
  save:
    ability: constitution
    dc: spell_save_dc
    automatic_failure_types:
    - plant
  damage:
    dice:
      base_count: 8
      sides: 8
      extra_per_slot: 1
      base_slot: 4
    type: necrotic
  success_damage_scale:
    numerator: 1
    denominator: 2
plant:
  target:
    kind: plant_object
    magical: false
    creature: false
    range_feet: 30
  saving_throw: false
  effects:
  - wither
  - die
```

#### Target binding

<!-- splat:values path=splat_rule/parameters/target_binding -->

| Field | Value |
| --- | --- |
| ` exactly_one ` | ` ["target_creature", "target_object"] ` |
| ` must_match_selected_mode ` | ` true ` |
