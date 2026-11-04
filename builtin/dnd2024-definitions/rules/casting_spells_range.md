---
splat_schema: 2
splat_id: 5d677f85-ca06-54d5-8562-f93436218c4f
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_range
splat_name: Range
splat_origin: dnd2024:reference/pages/105
splat_sources:
- SRD 5.2.1, pp. 105, https://www.dndbeyond.com/srd
---

# Range

## Source text

<!-- SRD page 105 -->

A spell’s range indicates how far from the spellcaster the spell’s effect can originate, and the spell’s description specifies which part of the effect is limited by the range.

A range usually takes one of the following forms:

Distance. The range is expressed in feet.

Touch. The spell’s effect originates on something, as defined by the spell, that the spellcaster must touch within their reach.

Self. The spell is cast on the spellcaster or emanates from them, as specified in the spell.

If a spell has movable effects, they aren’t restricted by its range unless the spell’s description says otherwise.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_range ` |
| ` implementation ` | ` dnd2024:rules_casting_spells_range ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` movable_effects_range_limited_after_cast_default ` | ` false ` |
| ` specific_spell_overrides ` | ` true ` |

#### Range modes

<!-- splat:yaml path=splat_rule/parameters/range_modes -->

```yaml
distance:
  unit: feet
  limits: spell_specified_effect_origin
touch:
  requires_touch: true
  within_caster_reach: true
self:
  origin: caster
  mode_from_spell:
  - on_caster
  - emanates_from_caster
```
