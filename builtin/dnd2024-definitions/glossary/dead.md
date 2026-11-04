---
splat_schema: 2
splat_id: 10ccec18-2354-5c74-9132-2d99af8798d3
splat_kind: rule
splat_key: dnd2024:glossary/dead
splat_name: Dead
splat_origin: dnd2024:reference/glossary/dead
splat_sources:
- SRD 5.2.1, pp. 180, https://www.dndbeyond.com/srd
---

# Dead

## Source text

<!-- SRD page 180 -->

A dead creature has no Hit Points and can’t regain them unless it is first revived by magic such as the Raise Dead or Revivify spell. When such a spell is cast, the spirit knows who is casting it and can refuse. The spirit of a dead creature has left the body and departed for the Outer Planes, and reviving the creature requires calling the spirit back.

If the creature returns to life, the revival effect determines the creature’s current Hit Points. Unless otherwise stated, the creature returns to life with any conditions, magical contagions, or curses that were affecting it at death if the durations of those effects are still ongoing. If the creature died with any Exhaustion levels, it returns with 1 fewer level. If the creature had Attunement to one or more magic items, it is no longer attuned to them.

Original layout: [[../../srd-5.2.1/pages/180|page 180]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/dead ` |
| ` implementation ` | ` dnd2024:glossary_dead ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` hit_points ` | ` 0 ` |
| ` healing_requires_prior_magical_revival ` | ` true ` |
| ` attunements_ended ` | ` true ` |

#### Revival

<!-- splat:yaml path=splat_rule/parameters/revival -->

```yaml
spirit_knows_caster: true
spirit_can_refuse: true
requires_spirit_return: true
hit_points: revival_effect_defined
ongoing_effects_return:
  kinds:
  - condition
  - magical_contagion
  - curse
  only_if_duration_ongoing: true
  unless_revival_states_otherwise: true
exhaustion_levels:
  op: max
  args:
  - 0
  - op: subtract
    args:
    - var: death_exhaustion_levels
    - 1
```
