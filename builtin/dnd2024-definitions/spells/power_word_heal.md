---
splat_schema: 2
splat_id: c6cccb53-4af6-55ef-af03-c177bb9950d2
splat_kind: spell
splat_key: dnd2024:spells/power_word_heal
splat_name: Power Word Heal
splat_origin: dnd2024:reference/spell/power_word_heal
splat_sources:
- SRD 5.2.1, pp. 154, https://www.dndbeyond.com/srd
---

# Power Word Heal


## Spell

| Field | Value |
| --- | --- |
| Level | 9 |
| School | Enchantment |
| Classes | Bard, Cleric |
| Casting Time | Action |
| Range | 60 feet |
| Components | V |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 154 -->

Level 9 Enchantment (Bard, Cleric)\
Casting Time: Action\
Range: 60 feet\
Component: V\
Duration: Instantaneous

A wave of healing energy washes over one creature you can see within range. The target regains all its Hit Points. If the creature has the Charmed, Frightened, Paralyzed, Poisoned, or Stunned condition, the condition ends. If the creature has the Prone condition, it can use its Reaction to stand up.

Original layout: [[../../srd-5.2.1/pages/154|page 154]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/power_word_heal ` |
| ` implementation ` | ` dnd2024:spell_power_word_heal ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` end_conditions ` | ` ["charmed", "frightened", "paralyzed", "poisoned", "stunned"] ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` visible ` | ` true ` |

#### Healing

<!-- splat:values path=splat_rule/parameters/healing -->

| Field | Value |
| --- | --- |
| ` restore ` | ` all_hit_points ` |

#### Prone

<!-- splat:values path=splat_rule/parameters/prone -->

| Field | Value |
| --- | --- |
| ` stand_optional ` | ` true ` |
| ` cost ` | ` target_reaction ` |
