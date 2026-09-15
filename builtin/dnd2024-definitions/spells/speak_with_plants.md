---
splat_schema: 1
splat_id: f3e9921a-757f-5580-a3cd-2d5c4bfe400a
splat_kind: spell
splat_key: dnd2024:spells/speak_with_plants
splat_name: Speak with Plants
splat_origin:
  key: dnd2024:reference/spell/speak_with_plants
splat_sources:
- title: SRD 5.2.1
  locator: pp. 164
  url: https://www.dndbeyond.com/srd
---

# Speak with Plants

## Source text

<!-- SRD page 164 -->

Level 3 Transmutation (Bard, Druid, Ranger)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: 10 minutes\
You imbue plants in an immobile 30-foot Emanation\
with limited sentience and animation, giving them\
the ability to communicate with you and follow your\
simple commands. You can question plants about\
events in the spell’s area within the past day, gain-\
ing information about creatures that have passed,\
weather, and other circumstances.\
You can also turn Difficult Terrain caused by plant\
growth (such as thickets and undergrowth) into\
ordinary terrain that lasts for the duration. Or you\
can turn ordinary terrain where plants are present\
into Difficult Terrain that lasts for the duration.\
The spell doesn’t enable plants to uproot them-\
selves and move about, but they can move their\
branches, tendrils, and stalks for you.\
If a Plant creature is in the area, you can commu-\
nicate with it as if you shared a common language.\

Original layout: [[../../srd-5.2.1/pages/164|page 164]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["bard", "druid", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 10 minutes ` |
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
| ` mechanic ` | ` dnd2024:spells/speak_with_plants ` |
| ` implementation ` | ` dnd2024:spell_speak_with_plants ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` terrain_duration ` | ` spell_duration ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` plants_in_area ` |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 30 ` |
| ` origin ` | ` caster_at_cast ` |
| ` immobile ` | ` true ` |

#### Animation

<!-- splat:values path=splat_rule/parameters/animation -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |
| ` limited_sentience ` | ` true ` |
| ` communicates_with ` | ` caster ` |
| ` follows ` | ` simple_commands ` |
| ` movable_parts ` | ` ["branches", "tendrils", "stalks"] ` |
| ` uproot_or_move_about ` | ` false ` |

#### Questions

<!-- splat:values path=splat_rule/parameters/questions -->

| Field | Value |
| --- | --- |
| ` scope ` | ` events_in_spell_area ` |
| ` history_days ` | ` 1 ` |
| ` topics ` | ` ["passing_creatures", "weather", "other_circumstances"] ` |

#### Terrain choices

<!-- splat:table
path: splat_rule/parameters/terrain_choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  From:
    path: from
    type: symbol
  To:
    path: to
    type: symbol
-->

| Key | From | To |
| --- | --- | --- |
| ` row_1 ` | ` Plant growth difficult terrain ` | ` Ordinary ` |
| ` row_2 ` | ` Ordinary with plants ` | ` Difficult terrain ` |

#### Plant creatures

<!-- splat:values path=splat_rule/parameters/plant_creatures -->

| Field | Value |
| --- | --- |
| ` communication ` | ` shared_language_equivalent ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 10
  unit: minute
```
