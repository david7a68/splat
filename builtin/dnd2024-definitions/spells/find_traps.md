---
splat_schema: 1
splat_id: 90d3db6a-0ae0-581a-aee9-ca9066d4d441
splat_kind: spell
splat_key: dnd2024:spells/find_traps
splat_name: Find Traps
splat_origin:
  key: dnd2024:reference/spell/find_traps
splat_sources:
- title: SRD 5.2.1
  locator: pp. 131
  url: https://www.dndbeyond.com/srd
---

# Find Traps

## Source text

<!-- SRD page 131 -->

Level 2 Divination (Cleric, Druid, Ranger)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous

You sense any trap within range that is within line of sight. A trap, for the purpose of this spell, includes any object or mechanism that was created to cause damage or other danger. Thus, the spell would sense the Alarm or Glyph of Warding spell or a mechanical pit trap, but it wouldn’t reveal a natural weakness in the floor, an unstable ceiling, or a hidden sinkhole.

This spell reveals that a trap is present but not its location. You do learn the general nature of the danger posed by a trap you sense.

Original layout: [[../../srd-5.2.1/pages/131|page 131]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["cleric", "druid", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/find_traps ` |
| ` implementation ` | ` dnd2024:spell_find_traps ` |

### Parameters

#### Detection

<!-- splat:values path=splat_rule/parameters/detection -->

| Field | Value |
| --- | --- |
| ` within_range ` | ` true ` |
| ` line_of_sight ` | ` true ` |
| ` eligible ` | ` object_or_mechanism_created_to_cause_damage_or_danger ` |
| ` includes_spells ` | ` ["alarm", "glyph_of_warding"] ` |
| ` excludes ` | ` ["natural_floor_weakness", "unstable_ceiling", "hidden_sinkhole"] ` |
| ` outputs ` | ` ["presence", "general_danger_nature"] ` |
| ` reveal_location ` | ` false ` |
