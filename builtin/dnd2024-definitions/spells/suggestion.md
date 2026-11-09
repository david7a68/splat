---
splat_schema: 2
splat_id: 8435cd6f-3662-5bf2-92b7-032054b2fa8d
splat_kind: spell
splat_key: dnd2024:spells/suggestion
splat_name: Suggestion
splat_origin: dnd2024:reference/spell/suggestion
splat_sources:
- SRD 5.2.1, pp. 166, https://www.dndbeyond.com/srd
---

# Suggestion


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Enchantment |
| Classes | Bard, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, M (a drop of honey) |
| Duration | Concentration, up to 8 hours |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 166 -->

Level 2 Enchantment (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, M (a drop of honey)\
Duration: Concentration, up to 8 hours

You suggest a course of activity—described in no more than 25 words—to one creature you can see within range that can hear and understand you. The suggestion must sound achievable and not involve anything that would obviously deal damage to the target or its allies. For example, you could say, “Fetch the key to the cult’s treasure vault, and give the key to me.” Or you could say, “Stop fighting, leave this library peacefully, and don’t return.”

The target must succeed on a Wisdom saving throw or have the Charmed condition for the duration or until you or your allies deal damage to the target. The Charmed target pursues the suggestion to the best of its ability. The suggested activity can continue for the entire duration, but if the suggested activity can be completed in a shorter time, the spell ends for the target upon completing it.

Original layout: [[../../srd-5.2.1/pages/166|page 166]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/suggestion ` |
| ` implementation ` | ` dnd2024:spell_suggestion ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 28800 ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 30 ` |
| ` hears_caster ` | ` true ` |
| ` understands_caster ` | ` true ` |

#### Instruction

<!-- splat:values path=splat_rule/parameters/instruction -->

| Field | Value |
| --- | --- |
| ` maximum_words ` | ` 25 ` |
| ` kind ` | ` course_of_activity ` |
| ` must_sound_achievable ` | ` true ` |
| ` obvious_damage_to_target_or_allies ` | ` forbidden ` |
| ` feasibility_resolution ` | ` supplied_assessment ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` dc ` | ` spell_save_dc ` |
| ` success ` | ` unaffected ` |

#### Failure

<!-- splat:values path=splat_rule/parameters/failure -->

| Field | Value |
| --- | --- |
| ` condition ` | ` charmed ` |
| ` source ` | ` caster ` |
| ` instruction_compliance ` | ` best_effort ` |

#### End target

<!-- splat:values path=splat_rule/parameters/end_target -->

| Field | Value |
| --- | --- |
| ` any ` | ` ["caster_or_allies_deal_damage_to_target", "suggested_activity_completed", "spell_duration_ended"] ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

