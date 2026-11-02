---
splat_schema: 1
splat_id: c53c534a-c02f-5da4-a876-46a1d9d6b4b7
splat_kind: spell
splat_key: dnd2024:spells/mirror_image
splat_name: Mirror Image
splat_origin:
  key: dnd2024:reference/spell/mirror_image
splat_sources:
- title: SRD 5.2.1
  locator: pp. 150
  url: https://www.dndbeyond.com/srd
---

# Mirror Image

## Source text

<!-- SRD page 150 -->

Level 2 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: 1 minute

Three illusory duplicates of yourself appear in your space. Until the spell ends, the duplicates move with you and mimic your actions, shifting position so it’s impossible to track which image is real.

Each time a creature hits you with an attack roll during the spell’s duration, roll a d6 for each of your remaining duplicates. If any of the d6s rolls a 3 or higher, one of the duplicates is hit instead of you, and the duplicate is destroyed. The duplicates otherwise ignore all other damage and effects. The spell ends when all three duplicates are destroyed.

A creature is unaffected by this spell if it has the Blinded condition, Blindsight, or Truesight.

Original layout: [[../../srd-5.2.1/pages/150|page 150]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["bard", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 minute ` |
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
| ` mechanic ` | ` dnd2024:spells/mirror_image ` |
| ` implementation ` | ` dnd2024:spell_mirror_image ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` duplicates_ignore ` | ` all_other_damage_and_effects ` |

#### Duplicates

<!-- splat:values path=splat_rule/parameters/duplicates -->

| Field | Value |
| --- | --- |
| ` initial_count ` | ` 3 ` |
| ` same_space ` | ` true ` |
| ` follow_and_mimic ` | ` true ` |
| ` indistinguishable ` | ` true ` |

#### Trigger

<!-- splat:yaml path=splat_rule/parameters/trigger -->

```yaml
event: attack.hit
target: caster
roll:
  dice_per_remaining_duplicate: 1
  sides: 6
if_any_die_gte: 3
replace_hit_target: one_duplicate
destroy_hit_duplicate: true
```

#### End

<!-- splat:values path=splat_rule/parameters/end -->

| Field | Value |
| --- | --- |
| ` remaining_duplicates ` | ` 0 ` |

#### Attacker bypass

<!-- splat:values path=splat_rule/parameters/attacker_bypass -->

| Field | Value |
| --- | --- |
| ` any ` | ` ["blinded", "blindsight", "truesight"] ` |
