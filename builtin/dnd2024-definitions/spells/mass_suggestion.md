---
splat_schema: 1
splat_id: 584df967-6461-5764-a029-9a90b1989351
splat_kind: spell
splat_key: dnd2024:spells/mass_suggestion
splat_name: Mass Suggestion
splat_origin:
  key: dnd2024:reference/spell/mass_suggestion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 148
  url: https://www.dndbeyond.com/srd
---

# Mass Suggestion

## Source text

<!-- SRD page 148 -->

Level 6 Enchantment (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, M (a snake’s tongue)\
Duration: 24 hours

You suggest a course of activity—described in no more than 25 words—to twelve or fewer creatures you can see within range that can hear and understand you. The suggestion must sound achievable and not involve anything that would obviously deal damage to any of the targets or their allies. For example, you could say, “Walk to the village down that road, and help the villagers there harvest crops until sunset.” Or you could say, “Now is not the time for violence. Drop your weapons, and dance! Stop in an hour.”

Each target must succeed on a Wisdom saving throw or have the Charmed condition for the duration or until you or your allies deal damage to the target. Each Charmed target pursues the suggestion to the best of its ability. The suggested activity can continue for the entire duration, but if the suggested activity can be completed in a shorter time, the spell ends for a target upon completing it.

Using a Higher-Level Spell Slot. The duration is longer with a spell slot of level 7 (10 days), 8 (30 days), or 9 (366 days).

Original layout: [[../../srd-5.2.1/pages/148|page 148]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 24 hours ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a snake’s tongue ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/mass_suggestion ` |
| ` implementation ` | ` dnd2024:spell_mass_suggestion ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 12 ` |
| ` within_spell_range ` | ` true ` |
| ` visible ` | ` true ` |
| ` can_hear_caster ` | ` true ` |
| ` understands_caster ` | ` true ` |

#### Suggestion

<!-- splat:yaml path=splat_rule/parameters/suggestion -->

```yaml
word_limit: 25
must_sound_achievable: true
must_not_obviously_damage:
- targets
- target_allies
content_validation:
  chooser: gm
  input_type: choice
  constraints:
  - achievable_appearance
  - no_obvious_damage
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  condition: charmed
  duration: spell_duration
  behavior: pursue_suggestion_best_ability
  end_on_target:
  - caster_or_caster_ally_deals_damage_to_target
  - suggested_activity_complete
success:
  effect: none
```

#### Duration by slot

<!-- splat:yaml path=splat_rule/parameters/duration_by_slot -->

```yaml
'6':
  hours: 24
'7':
  days: 10
'8':
  days: 30
'9':
  days: 366
```
