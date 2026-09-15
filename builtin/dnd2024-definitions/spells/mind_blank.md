---
splat_schema: 1
splat_id: b3f3b4e2-e4a0-5b9b-a8f7-909da1308af3
splat_kind: spell
splat_key: dnd2024:spells/mind_blank
splat_name: Mind Blank
splat_origin:
  key: dnd2024:reference/spell/mind_blank
splat_sources:
- title: SRD 5.2.1
  locator: pp. 149
  url: https://www.dndbeyond.com/srd
---

# Mind Blank

## Source text

<!-- SRD page 149 -->

Level 8 Abjuration (Bard, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: 24 hours\
Until the spell ends, one willing creature you touch\
has Immunity to Psychic damage and the Charmed\
condition. The target is also unaffected by anything\
that would sense its emotions or alignment, read\
its thoughts, or magically detect its location, and\
no spell—not even Wish—can gather information\
about the target, observe it remotely, or control its\
mind.\

Original layout: [[../../srd-5.2.1/pages/149|page 149]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 8 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["bard", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 24 hours ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/mind_blank ` |
| ` implementation ` | ` dnd2024:spell_mind_blank ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Immunities

<!-- splat:values path=splat_rule/parameters/immunities -->

| Field | Value |
| --- | --- |
| ` damage ` | ` ["psychic"] ` |
| ` conditions ` | ` ["charmed"] ` |
| ` effects ` | ` ["sense_emotions", "sense_alignment", "read_thoughts", "magically_detect_location"] ` |
| ` spells ` | ` ["gather_information_about_target", "remote_observation", "mind_control"] ` |
| ` includes_wish ` | ` true ` |
