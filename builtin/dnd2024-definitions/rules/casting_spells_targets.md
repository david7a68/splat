---
splat_schema: 2
splat_id: d7edfa47-92d7-5e30-9291-861ce3ec209a
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_targets
splat_name: Targets
splat_origin: dnd2024:reference/pages/106
splat_sources:
- SRD 5.2.1, pp. 106, https://www.dndbeyond.com/srd
---

# Targets

## Source text

<!-- SRD page 106 -->

A typical spell requires the caster to pick one or more targets to be affected by the spell’s magic. A spell’s description says whether the spell targets creatures, objects, or something else.

A Clear Path to the Target. To target something with a spell, a caster must have a clear path to it, so it can’t be behind Total Cover.

Targeting Yourself. If a spell targets a creature of your choice, you can choose yourself unless the creature must be Hostile or specifically a creature other than you.

Areas of Effect. Some spells, such as Thunderwave, cover an area called an area of effect, which is defined in “Rules Glossary.” The area determines what the spell targets. The description of a spell specifies whether it has an area of effect, which is typically one of these shapes: Cone, Cube, Cylinder, Emanation, Line, or Sphere.

Awareness of Being Targeted. Unless a spell has a perceptible effect, a creature doesn’t know it was targeted by the spell. An effect like lightning is obvious, but a more subtle effect, such as an attempt to read thoughts, goes unnoticed unless a spell’s description says otherwise.

Invalid Targets. If you cast a spell on someone or something that can’t be affected by it, nothing happens to that target, but if you used a spell slot to cast the spell, the slot is still expended.

If the spell normally has no effect on a target that succeeds on a saving throw, the invalid target appears to have succeeded on its saving throw, even though it didn’t attempt one (giving no hint that the creature is an invalid target). Otherwise, you perceive that the spell did nothing to the target.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_targets ` |
| ` implementation ` | ` dnd2024:rule_casting_spells_targets ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` target_kind_source ` | ` spell_description ` |
| ` clear_path_required ` | ` true ` |
| ` total_cover_blocks_targeting ` | ` true ` |
| ` area_shapes ` | ` ["cone", "cube", "cylinder", "emanation", "line", "sphere"] ` |
| ` area_targets ` | ` spell_area_geometry ` |

#### Self targeting

<!-- splat:values path=splat_rule/parameters/self_targeting -->

| Field | Value |
| --- | --- |
| ` allowed_if_creature_of_choice ` | ` true ` |
| ` excluded_if ` | ` ["hostile_required", "other_creature_required"] ` |

#### Target awareness

<!-- splat:values path=splat_rule/parameters/target_awareness -->

| Field | Value |
| --- | --- |
| ` requires_perceptible_effect ` | ` true ` |
| ` spell_exception_overrides ` | ` true ` |

#### Invalid target

<!-- splat:yaml path=splat_rule/parameters/invalid_target -->

```yaml
effect: none
spent_slot_refunded: false
perception:
  if_successful_save_normally_no_effect: apparent_successful_save
  otherwise: no_effect_observed
saving_throw_rolled: false
```
