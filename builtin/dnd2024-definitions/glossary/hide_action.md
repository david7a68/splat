---
splat_schema: 2
splat_id: 683841dc-a0ff-5b2e-a884-d671e301c0c8
splat_kind: rule
splat_key: dnd2024:glossary/hide_action
splat_name: Hide [Action]
splat_origin: dnd2024:reference/glossary/hide_action
splat_sources:
- SRD 5.2.1, pp. 183, https://www.dndbeyond.com/srd
---

# Hide [Action]

## Source text

<!-- SRD page 183 -->

With the Hide action, you try to hide yourself. To do so, you must succeed on a DC 15 Dexterity (Stealth) check while you’re Heavily Obscured or behind Three-Quarters Cover or Total Cover, and you must be out of any enemy’s line of sight; if you can see a creature, you can discern whether it can see you.

On a successful check, you have the Invisible condition while hidden. Make note of your check’s total, which is the DC for a creature to find you with a Wisdom (Perception) check.

You stop being hidden immediately after any of the following occurs: you make a sound louder than a whisper, an enemy finds you, you make an attack roll, or you cast a spell with a Verbal component.

Original layout: [[../../srd-5.2.1/pages/183|page 183]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/hide_action ` |
| ` implementation ` | ` dnd2024:glossary_hide_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` ends_immediately_after_any ` | ` ["sound_louder_than_whisper", "enemy_finds_subject", "subject_makes_attack_roll", "subject_casts_verbal_component_spell"] ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` any ` | ` ["heavily_obscured", "three_quarters_cover", "total_cover"] ` |
| ` outside_all_enemy_lines_of_sight ` | ` true ` |

#### Visibility information

<!-- splat:values path=splat_rule/parameters/visibility_information -->

| Field | Value |
| --- | --- |
| ` can_see_creature ` | ` true ` |
| ` can_discern_if_creature_sees_subject ` | ` true ` |

#### Check

<!-- splat:values path=splat_rule/parameters/check -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` skill ` | ` stealth ` |
| ` dc ` | ` 15 ` |

#### Success

<!-- splat:yaml path=splat_rule/parameters/success -->

```yaml
hidden: true
condition: invisible
find_check:
  ability: wisdom
  skill: perception
  dc:
    var: hide_check_total
```
