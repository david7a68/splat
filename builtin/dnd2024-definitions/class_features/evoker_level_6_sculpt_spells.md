---
splat_schema: 1
splat_id: 7691b3f2-db16-5bb6-a466-04cc54464ebe
splat_kind: rule
splat_key: dnd2024:class_features/evoker_level_6_sculpt_spells
splat_name: 'Level 6: Sculpt Spells'
splat_origin:
  key: dnd2024:reference/pages/082
splat_sources:
- title: SRD 5.2.1
  locator: pp. 82
  url: https://www.dndbeyond.com/srd
---

# Level 6: Sculpt Spells

## Source text

<!-- SRD page 82 -->

You can create pockets of relative safety within the\
effects of your evocations. When you cast an Evoca-\
tion spell that affects other creatures that you can\
see, you can choose a number of them equal to 1\
plus the spell’s level. The chosen creatures automat-\
ically succeed on their saving throws against the\
spell, and they take no damage if they would nor-\
mally take half damage on a successful save.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/evoker_level_6_sculpt_spells ` |
| ` implementation ` | ` dnd2024:class_features_evoker_level_6_sculpt_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.casts_evocation_spell ` |
| ` saving_throws_against_spell ` | ` automatic_success ` |

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
not_actor: true
affected_by_spell: true
visible_to_actor: true
maximum:
  op: add
  args:
  - 1
  - var: spell.level
selection: actor_choice
```

#### Damage on success

<!-- splat:values path=splat_rule/parameters/damage_on_success -->

| Field | Value |
| --- | --- |
| ` if_normally_half ` | ` 0 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` wizard ` |
| ` minimum_level ` | ` 6 ` |
