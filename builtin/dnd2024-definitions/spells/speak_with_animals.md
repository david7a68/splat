---
splat_schema: 1
splat_id: 3647122f-7ad3-526f-bce8-3d79ff153add
splat_kind: spell
splat_key: dnd2024:spells/speak_with_animals
splat_name: Speak with Animals
splat_origin:
  key: dnd2024:reference/spell/speak_with_animals
splat_sources:
- title: SRD 5.2.1
  locator: pp. 164
  url: https://www.dndbeyond.com/srd
---

# Speak with Animals

## Source text

<!-- SRD page 164 -->

Level 1 Divination (Bard, Druid, Ranger, Warlock)\
Casting Time: Action or Ritual\
Range: Self\
Components: V, S\
Duration: 10 minutes

For the duration, you can comprehend and verbally communicate with Beasts, and you can use any of the Influence action’s skill options with them.

Most Beasts have little to say about topics that don’t pertain to survival or companionship, but at minimum, a Beast can give you information about nearby locations and monsters, including whatever it has perceived within the past day.

Original layout: [[../../srd-5.2.1/pages/164|page 164]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "druid", "ranger", "warlock"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 10 minutes ` |
| ` casting_time ` | ` Action or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/speak_with_animals ` |
| ` implementation ` | ` dnd2024:spell_speak_with_animals ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` target ` | ` caster ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 600 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` with_creature_types ` | ` ["beast"] ` |
| ` comprehend ` | ` true ` |
| ` verbal_communication ` | ` true ` |
| ` influence_skill_options ` | ` all ` |

#### Information

<!-- splat:values path=splat_rule/parameters/information -->

| Field | Value |
| --- | --- |
| ` minimum_topics ` | ` ["nearby_locations", "nearby_monsters"] ` |
| ` minimum_perception_history_hours ` | ` 24 ` |
| ` typical_topic_limits ` | ` ["survival", "companionship"] ` |
| ` answers_depend_on ` | ` beast_knowledge ` |
