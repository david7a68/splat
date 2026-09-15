---
splat_schema: 1
splat_id: 0e3b269c-567c-55f8-81a7-715243cc0c86
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_verbal_v
splat_name: Verbal (V)
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
---

# Verbal (V)

## Source text

<!-- SRD page 105 -->

A Verbal component is the chanting of esoteric\
words that sound like nonsense to the uninitiated.\
The words must be uttered in a normal speaking\
voice. The words themselves aren’t the source of the\
spell’s power; rather, the particular combination of\
sounds, with specific pitch and resonance, sets the\
threads of magic in motion. Thus, a creature who is\
gagged or in an area of magical silence can’t cast a\
spell with a Verbal component.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_verbal_v ` |
| ` implementation ` | ` dnd2024:rules_casting_spells_verbal_v ` |

### Parameters

#### Verbal

<!-- splat:values path=splat_rule/parameters/verbal -->

| Field | Value |
| --- | --- |
| ` requires_normal_speaking_voice ` | ` true ` |
| ` blocked_by ` | ` ["gagged", "magical_silence"] ` |
